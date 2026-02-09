// Módulos públicos
pub mod types;
pub mod context;
pub mod visitor;

pub use context::Context;
pub use visitor::TypeChecker;

#[cfg(test)]
mod tests;

use crate::ast::nodes::*;
use crate::errors::SemanticError;
use types::{TypeKind, MethodInfo};
use context::conforms_to;
use std::collections::HashSet;

/// Función principal del análisis semántico
/// Realiza el chequeo en múltiples pasadas:
/// 1. Recolectar tipos y protocolos
/// 2. Establecer jerarquía de herencia
/// 3. Recolectar miembros (atributos y métodos)
/// 4. Chequear cuerpos de funciones y métodos

pub fn check_program(program: &Program) -> Result<Context, Vec<SemanticError>> {
    let mut context = Context::new();
    let mut errors = Vec::new();

    // --- Pasada 1: Recolectar nombres de Tipos y Protocolos ---
    for decl in &program.declarations {
        match decl {
            Declaration::Type(type_decl) => {
                if let Err(e) = context.create_type(&type_decl.name) {
                    errors.push(e);
                }
            }
            Declaration::Protocol(proto_decl) => {
                match context.create_type(&proto_decl.name) {
                    Ok(t) => { t.borrow_mut().kind = TypeKind::Protocol; }
                    Err(e) => errors.push(e),
                }
            }
            _ => {}
        }
    }
    if !errors.is_empty() { return Err(errors); }

    // --- Pasada 2: Establecer Jerarquía (Padres) ---
    for decl in &program.declarations {
        match decl {
            Declaration::Type(type_decl) => {
                if let Some(parent_init) = &type_decl.parent {
                    match context.get_type(&parent_init.name) {
                        Ok(parent_type) => {
                             if parent_type.borrow().kind == TypeKind::Protocol {
                                 errors.push(SemanticError::GenericError(format!("El tipo {} no puede heredar del protocolo {}", type_decl.name, parent_init.name)));
                             } else {
                                  match context.get_type(&type_decl.name) {
                                      Ok(child) => child.borrow_mut().parent = Some(parent_type.clone()),
                                      Err(_) => {} // No debería ocurrir
                                  }
                             }
                        },
                        Err(e) => errors.push(e),
                    }
                } else {
                    // Por defecto hereda de Object (excepto Object mismo)
                    if type_decl.name != "Object" {
                         let obj = context.get_type("Object").unwrap();
                         if let Ok(child) = context.get_type(&type_decl.name) {
                             child.borrow_mut().parent = Some(obj);
                         }
                    }
                }
            }
            Declaration::Protocol(proto_decl) => {
                 if let Some(parent_name) = &proto_decl.parent {
                      match context.get_type(parent_name) {
                           Ok(p) => {
                               if p.borrow().kind != TypeKind::Protocol {
                                   errors.push(SemanticError::GenericError(format!("El protocolo {} no puede heredar del tipo básico {}", proto_decl.name, parent_name)));
                               } else {
                                   if let Ok(child) = context.get_type(&proto_decl.name) {
                                       child.borrow_mut().parent = Some(p.clone());
                                   }
                               }
                           },
                           Err(e) => errors.push(e),
                      }
                 }
            }
            _ => {}
        }
    }
    
    
    // Chequear ciclos
    for (name, type_rc) in &context.types {
        let mut curr = type_rc.borrow().parent.clone();
        let mut visited = HashSet::new();
        visited.insert(name.clone());
        while let Some(p) = curr {
            let pname = p.borrow().name.clone();
            if visited.contains(&pname) {
                errors.push(SemanticError::CircularInheritance(name.clone()));
                break;
            }
            visited.insert(pname);
            curr = p.borrow().parent.clone();
        }
    }
    
    if !errors.is_empty() { return Err(errors); }

    // --- Pasada 3: Recolectar Miembros (Métodos/Atributos) y Funciones Globales ---
    
    // 3a. Funciones Globales
    for decl in &program.declarations {
        if let Declaration::Function(func_decl) = decl {
            let mut params = Vec::new();
            for p in &func_decl.params {
                let p_type = match &p.type_annotation {
                    Some(ann) => {
                        match context.resolve_type(ann) {
                            Ok(t) => t,
                            Err(e) => { errors.push(e); context.get_type("Object").unwrap() }
                        }
                    },
                    None => context.get_type("Object").unwrap(),
                };
                params.push(p_type);
            }
            
            let ret_type = match &func_decl.return_type {
                Some(ann) => {
                    match context.resolve_type(ann) {
                        Ok(t) => t,
                        Err(e) => { errors.push(e); context.get_type("Object").unwrap() }
                    }
                },
                None => context.get_type("Object").unwrap(), // Retorno por defecto Object? 
            };
            
            if let Err(e) = context.define_function(&func_decl.name, params, ret_type) {
                errors.push(e);
            }
        }
    }

    // 3a-bis. Macros (def) – register as functions for semantic analysis
    for decl in &program.declarations {
        if let Declaration::Macro(macro_decl) = decl {
            let mut params = Vec::new();
            for p in &macro_decl.params {
                let ann = match p {
                    MacroParam::Normal { type_annotation, .. }
                    | MacroParam::Symbolic { type_annotation, .. }
                    | MacroParam::Placeholder { type_annotation, .. }
                    | MacroParam::Body { type_annotation, .. } => type_annotation,
                };
                let p_type = match context.resolve_type(ann) {
                    Ok(t) => t,
                    Err(e) => { errors.push(e); context.get_type("Object").unwrap() }
                };
                params.push(p_type);
            }

            let ret_type = match &macro_decl.return_type {
                Some(ann) => {
                    match context.resolve_type(ann) {
                        Ok(t) => t,
                        Err(e) => { errors.push(e); context.get_type("Object").unwrap() }
                    }
                },
                None => context.get_type("Object").unwrap(),
            };

            if let Err(e) = context.define_function(&macro_decl.name, params, ret_type) {
                errors.push(e);
            }
        }
    }
    
    // 3b. Miembros de Tipos
    for decl in &program.declarations {
         if let Declaration::Type(type_decl) = decl {
             let current_type_rc = context.get_type(&type_decl.name).unwrap();
             
             // Parametros del Constructor
             let mut ctor_params = Vec::new();
             for p in &type_decl.params {
                  let p_type = match &p.type_annotation {
                    Some(ann) => match context.resolve_type(ann) { Ok(t) => t, Err(e) => { errors.push(e); context.get_type("Object").unwrap() } },
                    None => context.get_type("Object").unwrap(),
                  };
                  ctor_params.push((p.name.clone(), p_type));
             }
             current_type_rc.borrow_mut().params = ctor_params;

             // Atributos
             for attr in &type_decl.attributes {
                 let attr_type = match &attr.type_annotation {
                     Some(ann) => match context.resolve_type(ann) {
                         Ok(t) => t,
                         Err(e) => { errors.push(e); context.get_type("Object").unwrap() }
                     },
                     None => context.get_type("Object").unwrap(),
                 };
                 // Agregar atributo al tipo
                 if current_type_rc.borrow().attributes.contains_key(&attr.name) {
                     errors.push(SemanticError::AttributeDefined(format!("{}.{}", type_decl.name, attr.name)));
                 } else {
                     current_type_rc.borrow_mut().attributes.insert(attr.name.clone(), attr_type);
                 }
             }
             
             // Métodos
             for method in &type_decl.methods {
                 let mut params = Vec::new();
                 for p in &method.params {
                      let p_type = match &p.type_annotation {
                        Some(ann) => match context.resolve_type(ann) { Ok(t) => t, Err(e) => { errors.push(e); context.get_type("Object").unwrap() } },
                        None => context.get_type("Object").unwrap(),
                      };
                      params.push((p.name.clone(), p_type));
                 }
                 let ret_type = match &method.return_type {
                     Some(ann) => match context.resolve_type(ann) { Ok(t) => t, Err(e) => { errors.push(e); context.get_type("Object").unwrap() } },
                     None => context.get_type("Object").unwrap(),
                 };
                 
                 let method_info = MethodInfo {
                     name: method.name.clone(),
                     params,
                     return_type: ret_type,
                 };
                 
                 if current_type_rc.borrow().methods.contains_key(&method.name) {
                      errors.push(SemanticError::MethodDefined(format!("{}.{}", type_decl.name, method.name)));
                 } else {
                     // Chequear override (sobreescritura)
                     let mut parent = current_type_rc.borrow().parent.clone();
                     while let Some(p) = parent {
                         if let Some(parent_method) = p.borrow().methods.get(&method.name) {
                             if parent_method.params.len() != method_info.params.len() {
                                 errors.push(SemanticError::SignatureMismatch(format!("{}.{} sobreescribe con diferente cantidad de argumentos", type_decl.name, method.name)));
                             } else {
                                // Chequear covarianza de retorno (Impl <= Base)
                                if !method_info.return_type.borrow().conforms_to(&parent_method.return_type) {
                                     errors.push(SemanticError::SignatureMismatch(format!(
                                         "El tipo de retorno de {}.{} ({}) no conforma al del padre ({})", 
                                         type_decl.name, method.name, 
                                         method_info.return_type.borrow().name, 
                                         parent_method.return_type.borrow().name
                                     )));
                                }
                                
                                // Chequear contravarianza de argumentos (Base <= Impl)
                                for (i, (_, param_type)) in method_info.params.iter().enumerate() {
                                    let (_, parent_param_type) = &parent_method.params[i];
                                    if !parent_param_type.borrow().conforms_to(param_type) {
                                         errors.push(SemanticError::SignatureMismatch(format!(
                                             "El argumento {} de {}.{} ({}) no conforma al del padre ({}) (se requiere contravarianza)", 
                                             i, type_decl.name, method.name, 
                                             param_type.borrow().name, 
                                             parent_param_type.borrow().name
                                         )));
                                    }
                                }
                             }
                         }
                         parent = p.borrow().parent.clone();
                     }
                     current_type_rc.borrow_mut().methods.insert(method.name.clone(), method_info);
                 }
             }
         }
         
         if let Declaration::Protocol(proto) = decl {
             let current_type_rc = context.get_type(&proto.name).unwrap();
             for method in &proto.methods {
                  let mut params = Vec::new();
                 for p in &method.params {
                      let p_type = match &p.type_annotation {
                        Some(ann) => match context.resolve_type(ann) { Ok(t) => t, Err(e) => { errors.push(e); context.get_type("Object").unwrap() } },
                        None => context.get_type("Object").unwrap(),
                      };
                      params.push((p.name.clone(), p_type));
                 }
                 let ret_type = match context.resolve_type(&method.return_type) { Ok(t) => t, Err(e) => { errors.push(e); context.get_type("Object").unwrap() } };
                 
                 let method_info = MethodInfo {
                     name: method.name.clone(),
                     params,
                     return_type: ret_type,
                 };
                 current_type_rc.borrow_mut().methods.insert(method.name.clone(), method_info);
             }
         }
    }
    
    if !errors.is_empty() { return Err(errors); }

    // --- Pasada 4: Chequear Cuerpos (Funciones y Métodos) ---
    
    // Chequear Expresión Global
    {
        let mut checker = TypeChecker::new(&context);
        if let Err(mut body_errors) = checker.infer_type(&program.expr) {
            errors.append(&mut body_errors);
        }
    }
    
    // Chequear Funciones Globales
    for decl in &program.declarations {
         if let Declaration::Function(func) = decl {
             let mut checker = TypeChecker::new(&context);
             // Agregar parámetros al scope
             let (params_types, _) = context.functions.get(&func.name).unwrap();
             for (i, p_decl) in func.params.iter().enumerate() {
                 let t = params_types[i].clone();
                 checker.define(p_decl.name.clone(), t);
             }
             
             if let Err(mut body_errors) = checker.infer_type(&func.body) {
                errors.append(&mut body_errors);
             }
         }
    }

    // Chequear Macros (def) 
    for decl in &program.declarations {
         if let Declaration::Macro(macro_decl) = decl {
             let mut checker = TypeChecker::new(&context);
             let (params_types, _) = context.functions.get(&macro_decl.name).unwrap();
             for (i, p) in macro_decl.params.iter().enumerate() {
                 let pname = match p {
                     MacroParam::Normal { name, .. }
                     | MacroParam::Symbolic { name, .. }
                     | MacroParam::Placeholder { name, .. }
                     | MacroParam::Body { name, .. } => name,
                 };
                 let t = params_types[i].clone();
                 checker.define(pname.clone(), t);
             }

             if let Err(mut body_errors) = checker.infer_type(&macro_decl.body) {
                errors.append(&mut body_errors);
             }
         }
    }
    
    // Chequear Cuerpos de Tipos (Métodos y Atributos)
    for decl in &program.declarations {
        if let Declaration::Type(type_decl) = decl {
            let type_rc = context.get_type(&type_decl.name).unwrap();
            
            // 1. Inicialización de Atributos
            // Los atributos pueden usar argumentos del constructor en su inicialización
            for attr in &type_decl.attributes {
                 let mut checker = TypeChecker::new(&context);
                 // Añadir parámetros del constructor al scope
                 for (name, ty) in &type_rc.borrow().params {
                     checker.define(name.clone(), ty.clone());
                 }
                 
                 checker.current_type = Some(type_rc.clone());

                 let init_type = match checker.infer_type(&attr.init) {
                     Ok(t) => t,
                     Err(mut e) => { errors.append(&mut e); context.get_type("Object").unwrap() }
                 };
                 
                 let attr_type = type_rc.borrow().attributes.get(&attr.name).unwrap().clone();
                 if !conforms_to(init_type.clone(), attr_type.clone()) {
                      errors.push(SemanticError::TypeMismatch{ expected: attr_type.borrow().name.clone(), found: init_type.borrow().name.clone(), pos: attr.init.pos });
                 }
            }

            // 2. Cuerpos de Métodos
            for method in &type_decl.methods {
                let mut checker = TypeChecker::new(&context);
                let method_info = type_rc.borrow().get_method(&method.name).unwrap();
                
                // Definir parámetros del método en el scope
                for (name, ty) in &method_info.params {
                    checker.define(name.clone(), ty.clone());
                }
                
                checker.current_type = Some(type_rc.clone()); // 'self' disponible
                
                let body_type = match checker.infer_type(&method.body) {
                     Ok(t) => t,
                     Err(mut e) => { errors.append(&mut e); context.get_type("Object").unwrap() }
                 };
                 
                 if !conforms_to(body_type.clone(), method_info.return_type.clone()) {
                      errors.push(SemanticError::TypeMismatch{ expected: method_info.return_type.borrow().name.clone(), found: body_type.borrow().name.clone(), pos: method.body.pos });
                 }
            }
        }
    }

    if !errors.is_empty() { return Err(errors); }

    Ok(context)
}
