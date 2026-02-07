pub mod types;
pub mod scope;

#[cfg(test)]
mod tests;

use crate::ast::nodes::*;
use crate::utils::Spanned;
use crate::errors::SemanticError;
use types::{Type, TypeKind, MethodInfo, TypeFactory, lowest_common_ancestor};
use scope::Scope;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::cell::RefCell;

pub struct Context {
    pub types: HashMap<String, Rc<RefCell<Type>>>,
    pub functions: HashMap<String, (Vec<Rc<RefCell<Type>>>, Rc<RefCell<Type>>)>,
}

impl Context {
    pub fn new() -> Self {
        let mut types = HashMap::new();
        let object = TypeFactory::object();
        types.insert("Object".to_string(), object.clone());
        types.insert("Number".to_string(), TypeFactory::number(object.clone()));
        types.insert("Boolean".to_string(), TypeFactory::boolean(object.clone()));
        types.insert("String".to_string(), TypeFactory::string(object.clone()));
        
        Context { 
            types,
            functions: HashMap::new(),
        }
    }

    pub fn get_type(&self, name: &str) -> Result<Rc<RefCell<Type>>, SemanticError> {
        self.types.get(name)
            .cloned()
            .ok_or(SemanticError::TypeNotFound(name.to_string()))
    }
    
    pub fn create_type(&mut self, name: &str) -> Result<Rc<RefCell<Type>>, SemanticError> {
        if self.types.contains_key(name) {
             return Err(SemanticError::TypeDefined(name.to_string()));
        }
        let t = Rc::new(RefCell::new(Type::new(name, TypeKind::Basic, None)));
        self.types.insert(name.to_string(), t.clone());
        Ok(t)
    }

    pub fn define_function(&mut self, name: &str, params: Vec<Rc<RefCell<Type>>>, ret: Rc<RefCell<Type>>) -> Result<(), SemanticError> {
        if self.functions.contains_key(name) {
            return Err(SemanticError::FunctionDefined(name.to_string()));
        }
        self.functions.insert(name.to_string(), (params, ret));
        Ok(())
    }
    
    pub fn get_function(&self, name: &str) -> Option<(Vec<Rc<RefCell<Type>>>, Rc<RefCell<Type>>)> {
        // Primero chequear funciones de la librería estándar si las hay
        if name == "print" {
            let obj = self.types.get("Object").unwrap().clone();
            let void = self.types.get("Object").unwrap().clone(); // Asumiendo que print retorna Object/Void
            return Some((vec![obj], void));
        }
        // Funciones matemáticas
        if ["sin", "cos", "sqrt", "log", "exp"].contains(&name) {
             let num = self.types.get("Number").unwrap().clone();
             return Some((vec![num.clone()], num)); // Unario
        }
         if name == "log" { // Binary log(base, val) handled maybe? AST says Log(base, val)
             // AST tiene Log(base, val) que es binario.
             // Pero el usuario podría llamar `log(x)` ... espera, Context `get_function` es por nombre.
             // Si nombres builtin colisionan, necesitamos chequear cantidad de args. Context retorna UNA firma.
             // Asumimos que la lib estándar esta fija
         }

        if name == "rand" {
             let num = self.types.get("Number").unwrap().clone();
             return Some((vec![], num));
        }

        self.functions.get(name).cloned()
    }

    // Resolver TypeAnnotation al Type real
    pub fn resolve_type(&self, annotation: &TypeAnnotation) -> Result<Rc<RefCell<Type>>, SemanticError> {
        match annotation {
            TypeAnnotation::Name(name) => self.get_type(name),
            TypeAnnotation::Iterable(inner) => {
                 let inner_type = self.resolve_type(inner)?;
                 let name = format!("Iterable<{}>", inner_type.borrow().name);
                 
                 // Crear 'al vuelo' el protocolo Iterable<T>
                 // Tiene: next() -> Boolean, current() -> T
                 let iter = Rc::new(RefCell::new(Type::new(&name, TypeKind::Protocol, None)));
                 
                 let bool_type = self.get_type("Boolean")?;
                 iter.borrow_mut().define_method("next".to_string(), vec![], bool_type);
                 iter.borrow_mut().define_method("current".to_string(), vec![], inner_type);
                 
                 Ok(iter)
            },
            TypeAnnotation::Function { params, return_type } => {
                for p in params { self.resolve_type(p)?; }
                self.resolve_type(return_type)?;
                self.get_type("Object") // Tratar tipo función como Object
            }
        }
    }
}

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
        // Envolver scope en Rc para compartir
        let scope = Rc::new(Scope::new());
        let mut checker = BodyChecker::new(&context, scope);
        if let Err(mut body_errors) = checker.check_expr(&program.expr) {
            errors.append(&mut body_errors);
        }
    }
    
    // Chequear Funciones Globales
    for decl in &program.declarations {
         if let Declaration::Function(func) = decl {
             let scope = Rc::new(Scope::new());
             // Agregar parámetros al scope
             let (params_types, _) = context.functions.get(&func.name).unwrap();
             for (i, p_decl) in func.params.iter().enumerate() {
                 let t = params_types[i].clone();
                 scope.define_variable(p_decl.name.clone(), t);
             }
             
             let mut checker = BodyChecker::new(&context, scope);
             if let Err(mut body_errors) = checker.check_expr(&func.body) {
                errors.append(&mut body_errors);
             }
         }
    }

    // Chequear Macros (def) – treat as functions for body checking
    for decl in &program.declarations {
         if let Declaration::Macro(macro_decl) = decl {
             let scope = Rc::new(Scope::new());
             let (params_types, _) = context.functions.get(&macro_decl.name).unwrap();
             for (i, p) in macro_decl.params.iter().enumerate() {
                 let pname = match p {
                     MacroParam::Normal { name, .. }
                     | MacroParam::Symbolic { name, .. }
                     | MacroParam::Placeholder { name, .. }
                     | MacroParam::Body { name, .. } => name,
                 };
                 let t = params_types[i].clone();
                 scope.define_variable(pname.clone(), t);
             }

             let mut checker = BodyChecker::new(&context, scope);
             if let Err(mut body_errors) = checker.check_expr(&macro_decl.body) {
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
                 let scope = Rc::new(Scope::new());
                 // Añadir parámetros del constructor al scope
                 for (name, ty) in &type_rc.borrow().params {
                     scope.define_variable(name.clone(), ty.clone());
                 }
                 
                 let mut checker = BodyChecker::new(&context, scope);
                 
                 checker.current_type = Some(type_rc.clone());

                 let init_type = match checker.check_expr(&attr.init) {
                     Ok(t) => t,
                     Err(mut e) => { errors.append(&mut e); context.get_type("Object").unwrap() }
                 };
                 
                 let attr_type = type_rc.borrow().attributes.get(&attr.name).unwrap().clone();
                 if !conforms_to(init_type.clone(), attr_type.clone()) {
                      errors.push(SemanticError::TypeMismatch{ expected: attr_type.borrow().name.clone(), found: init_type.borrow().name.clone() });
                 }
            }

            // 2. Cuerpos de Métodos
            for method in &type_decl.methods {
                let scope = Rc::new(Scope::new());
                let method_info = type_rc.borrow().get_method(&method.name).unwrap();
                
                // Definir parámetros del método en el scope
                for (name, ty) in &method_info.params {
                    scope.define_variable(name.clone(), ty.clone());
                }
                
                let mut checker = BodyChecker::new(&context, scope);
                checker.current_type = Some(type_rc.clone()); // 'self' disponible
                
                let body_type = match checker.check_expr(&method.body) {
                     Ok(t) => t,
                     Err(mut e) => { errors.append(&mut e); context.get_type("Object").unwrap() }
                 };
                 
                 if !conforms_to(body_type.clone(), method_info.return_type.clone()) {
                      errors.push(SemanticError::TypeMismatch{ expected: method_info.return_type.borrow().name.clone(), found: body_type.borrow().name.clone() });
                 }
            }
        }
    }

    if !errors.is_empty() { return Err(errors); }

    Ok(context)
}

struct BodyChecker<'a> {
    context: &'a Context,
    scope: Rc<Scope>,
    current_type: Option<Rc<RefCell<Type>>>,
}

impl<'a> BodyChecker<'a> {
    fn new(context: &'a Context, scope: Rc<Scope>) -> Self {
        BodyChecker { context, scope, current_type: None }
    }
    
    // Retorna el tipo resuelto si tiene éxito
    fn check_expr(&mut self, expr_spanned: &Spanned<Expr>) -> Result<Rc<RefCell<Type>>, Vec<SemanticError>> {
        match &expr_spanned.node {
            Expr::Number(_) => Ok(self.context.get_type("Number").unwrap()),
            Expr::String(_) => Ok(self.context.get_type("String").unwrap()),
            Expr::Boolean(_) => Ok(self.context.get_type("Boolean").unwrap()),
            Expr::Identifier(name) => {
                if name == "self" {
                    if let Some(t) = &self.current_type {
                        Ok(t.clone())
                    } else {
                        Err(vec![SemanticError::SelfReference])
                    }
                } else if let Some(t) = self.scope.find_variable(name) {
                    Ok(t)
                } else {
                    // ¿Constante?
                     if name == "PI" || name == "E" {
                          Ok(self.context.get_type("Number").unwrap())
                     } else {
                          Err(vec![SemanticError::VariableNotFound(name.clone())])
                     }
                }
            },
            Expr::Binary(left, op, right) => {
               let t_left = self.check_expr(left);
               let t_right = self.check_expr(right);
               
               if t_left.is_ok() && t_right.is_ok() {
                   let l = t_left.unwrap();
                   let r = t_right.unwrap();
                   
                   if l.borrow().name == "Number" && r.borrow().name == "Number" {
                        match op {
                            Op::Add | Op::Sub | Op::Mul | Op::Div | Op::Mod | Op::Pow => Ok(self.context.get_type("Number").unwrap()),
                            Op::Eq | Op::Neq | Op::Lt | Op::Gt | Op::Le | Op::Ge => Ok(self.context.get_type("Boolean").unwrap()),
                             _ => Err(vec![SemanticError::OperationNotDefined(format!("{:?}", op), "Number".to_string())])
                        }
                   } else if l.borrow().name == "Boolean" && r.borrow().name == "Boolean" {
                        match op {
                            Op::And | Op::Or => Ok(self.context.get_type("Boolean").unwrap()),
                             Op::Eq | Op::Neq => Ok(self.context.get_type("Boolean").unwrap()),
                            _ => Err(vec![SemanticError::OperationNotDefined(format!("{:?}", op), "Boolean".to_string())])
                        }
                   } else if matches!(op, Op::Concat | Op::ConcatSpace) {
                        // @ and @@ work with any types – codegen converts to string at runtime
                        Ok(self.context.get_type("String").unwrap())
                   } else if l.borrow().name == "Object" || r.borrow().name == "Object" {
                        // When one side is Object (unknown type at compile time),
                        // allow the operation and infer the result from the known side.
                        match op {
                            Op::Add | Op::Sub | Op::Mul | Op::Div | Op::Mod | Op::Pow => Ok(self.context.get_type("Number").unwrap()),
                            Op::Eq | Op::Neq | Op::Lt | Op::Gt | Op::Le | Op::Ge => Ok(self.context.get_type("Boolean").unwrap()),
                            Op::And | Op::Or => Ok(self.context.get_type("Boolean").unwrap()),
                            _ => Ok(self.context.get_type("Object").unwrap()),
                        }
                   } else {
                        if matches!(op, Op::Eq | Op::Neq) {
                             Ok(self.context.get_type("Boolean").unwrap())
                        } else {
                             Err(vec![SemanticError::GenericError(format!("Operation {:?} is not defined for types {} and {}", op, l.borrow().name, r.borrow().name))])
                        }
                   }
               } else {
                   let mut errs = Vec::new();
                   if let Err(mut e) = t_left { errs.append(&mut e); }
                   if let Err(mut e) = t_right { errs.append(&mut e); }
                   Err(errs)
               }
            },
            Expr::Unary(op, operand) => {
                 let t_op = self.check_expr(operand)?;
                 if t_op.borrow().name == "Number" && matches!(op, UnOp::Neg) {
                      Ok(t_op)
                 } else if t_op.borrow().name == "Boolean" && matches!(op, UnOp::Not) {
                      Ok(t_op)
                 } else {
                       Err(vec![SemanticError::OperationNotDefined(format!("{:?}", op), t_op.borrow().name.clone())])
                 }
            },
            Expr::Let { bindings, body } => {
                let current_scope = Rc::new(Scope::new_child(self.scope.clone()));
                for (name, type_ann, expr_span) in bindings {
                   
                    let mut temp_checker = BodyChecker::new(self.context, current_scope.clone());
                    let t_expr = temp_checker.check_expr(expr_span)?;
                    
                    if let Some(ann) = type_ann {
                         let t_ann = self.context.resolve_type(ann).map_err(|e| vec![e])?;
                         if !conforms_to(t_expr.clone(), t_ann.clone()) {
                               return Err(vec![SemanticError::TypeMismatch { expected: t_ann.borrow().name.clone(), found: t_expr.borrow().name.clone() }]);
                         }
                    }
                    current_scope.define_variable(name.clone(), t_expr);
                }
                
                let mut body_checker = BodyChecker::new(self.context, current_scope);
                body_checker.check_expr(body)
            },
            Expr::Block(exprs) => {
                 let mut last_type = self.context.get_type("Object").unwrap(); 
                 for e in exprs {
                     last_type = self.check_expr(e)?;
                 }
                 Ok(last_type)
            },
            Expr::Call { func: name, args } => {
                if let Some((param_types, ret_type)) = self.context.get_function(name) {
                     if args.len() != param_types.len() {
                          return Err(vec![SemanticError::ArgumentCountMismatch(name.clone(), param_types.len(), args.len())]);
                     }
                     for (i, arg_expr) in args.iter().enumerate() {
                          let arg_type = self.check_expr(arg_expr)?;
                          if !conforms_to(arg_type.clone(), param_types[i].clone()) {
                               return Err(vec![SemanticError::TypeMismatch{ expected: param_types[i].borrow().name.clone(), found: arg_type.borrow().name.clone() }]);
                          }
                     }
                     Ok(ret_type)
                } else if self.scope.find_variable(name).is_some() {
                    // Variable exists in scope — could be a lambda; check args but return Object
                    for arg_expr in args {
                        let _ = self.check_expr(arg_expr)?;
                    }
                    Ok(self.context.get_type("Object").unwrap())
                } else {
                    Err(vec![SemanticError::FunctionNotFound(name.clone(), args.len())])
                }
            },
             Expr::If{cond, then_expr, else_expr} => {
                 let t_cond = self.check_expr(cond)?;
                 // Accept Boolean or Object (Object may be Boolean at runtime)
                 if t_cond.borrow().name != "Boolean" && t_cond.borrow().name != "Object" {
                     return Err(vec![SemanticError::TypeMismatch{ expected: "Boolean".to_string(), found: t_cond.borrow().name.clone() }]);
                 }
                 let t_then = self.check_expr(then_expr)?;
                 let t_else = self.check_expr(else_expr)?;
                 
                 lowest_common_ancestor(t_then, t_else)
             },
             Expr::PI | Expr::E => Ok(self.context.get_type("Number").unwrap()),
             Expr::Rand => Ok(self.context.get_type("Number").unwrap()),
             Expr::Sin(e) | Expr::Cos(e) | Expr::Sqrt(e) | Expr::Exp(e) => {
                 let t = self.check_expr(e)?;
                 if t.borrow().name != "Number" { return Err(vec![SemanticError::TypeMismatch{expected: "Number".into(), found: t.borrow().name.clone()}]); }
                 Ok(self.context.get_type("Number").unwrap())
             },
             Expr::Log(base, val) => {
                 let t1 = self.check_expr(base)?;
                 let t2 = self.check_expr(val)?;
                 if t1.borrow().name != "Number" { return Err(vec![SemanticError::TypeMismatch{expected: "Number".into(), found: t1.borrow().name.clone()}]); }
                 if t2.borrow().name != "Number" { return Err(vec![SemanticError::TypeMismatch{expected: "Number".into(), found: t2.borrow().name.clone()}]); }
                 Ok(self.context.get_type("Number").unwrap())
             },
             Expr::Assignment { target, value } => {
                 if let Some(t_var) = self.scope.find_variable(target) {
                      let t_val = self.check_expr(value)?;
                      if !conforms_to(t_val.clone(), t_var.clone()) {
                          return Err(vec![SemanticError::TypeMismatch{ expected: t_var.borrow().name.clone(), found: t_val.borrow().name.clone() }]);
                      }
                      Ok(t_val)
                 } else {
                     Err(vec![SemanticError::VariableNotFound(target.clone())])
                 }
             },
             Expr::AttributeAssignment { obj, attribute, value } => {
                 let t_obj = self.check_expr(obj)?;
                 let t_val = self.check_expr(value)?;
                 // Buscar atributo en la jerarquía de tipos
                 let mut curr = Some(t_obj.clone());
                 while let Some(c) = curr {
                     if let Some(attr_type) = c.borrow().attributes.get(attribute) {
                         if !conforms_to(t_val.clone(), attr_type.clone()) {
                             return Err(vec![SemanticError::TypeMismatch{ expected: attr_type.borrow().name.clone(), found: t_val.borrow().name.clone() }]);
                         }
                         return Ok(t_val);
                     }
                     curr = c.borrow().parent.clone();
                 }
                 // Atributo no encontrado en info de tipo — permitir como Object (fallback)
                 Ok(t_val)
             },
             Expr::While { cond, body } => {
                  let t_cond = self.check_expr(cond)?;
                  // Accept Boolean or Object (Object may be Boolean at runtime)
                  if t_cond.borrow().name != "Boolean" && t_cond.borrow().name != "Object" {
                      return Err(vec![SemanticError::TypeMismatch{ expected: "Boolean".to_string(), found: t_cond.borrow().name.clone() }]);
                  }
                  let _ = self.check_expr(body)?;
                  Ok(self.context.get_type("Object").unwrap())
             },
             Expr::Instantiation { ty, args } => {
                 let type_rc = self.context.get_type(ty).map_err(|e| vec![e])?;
                 let params = type_rc.borrow().params.clone();
                 if args.len() != params.len() {
                      return Err(vec![SemanticError::ArgumentCountMismatch(ty.clone(), params.len(), args.len())]);
                 }
                 for (i, arg_expr) in args.iter().enumerate() {
                      let arg_type = self.check_expr(arg_expr)?;
                      if !conforms_to(arg_type.clone(), params[i].1.clone()) {
                           return Err(vec![SemanticError::TypeMismatch{ expected: params[i].1.borrow().name.clone(), found: arg_type.borrow().name.clone() }]);
                      }
                 }
                 Ok(type_rc)
             },
             Expr::MethodCall { obj, method, args } => {
                 let t_obj = self.check_expr(obj)?;
                 // Find method in hierarchy
                 let mut curr = Some(t_obj.clone());
                 let mut found_method = None;
                 while let Some(c) = curr {
                     if let Some(m) = c.borrow().methods.get(method) {
                         found_method = Some(m.clone());
                         break;
                     }
                     curr = c.borrow().parent.clone();
                 }
                 
                 if let Some(m) = found_method {
                      if args.len() != m.params.len() {
                           return Err(vec![SemanticError::ArgumentCountMismatch(method.clone(), m.params.len(), args.len())]);
                      }
                      for (i, arg_expr) in args.iter().enumerate() {
                           let arg_type = self.check_expr(arg_expr)?;
                           if !conforms_to(arg_type.clone(), m.params[i].1.clone()) {
                                return Err(vec![SemanticError::TypeMismatch{ expected: m.params[i].1.borrow().name.clone(), found: arg_type.borrow().name.clone() }]);
                           }
                      }
                      Ok(m.return_type.clone())
                 } else {
                     Err(vec![SemanticError::MethodNotFound(format!("Method {} not found in {}", method, t_obj.borrow().name))])
                 }
             },
             Expr::AttributeAccess { obj, attribute } => {
                 let t_obj = self.check_expr(obj)?;
                 // Look for attribute in the type hierarchy
                 let mut curr = Some(t_obj.clone());
                 while let Some(c) = curr {
                     if let Some(attr_type) = c.borrow().attributes.get(attribute) {
                         return Ok(attr_type.clone());
                     }
                     curr = c.borrow().parent.clone();
                 }
                 // Attribute not found in type info — return Object as fallback
                 Ok(self.context.get_type("Object").unwrap())
             },
             Expr::Is(expr, _type_name) => {
                 let _ = self.check_expr(expr)?;
                 Ok(self.context.get_type("Boolean").unwrap())
             },
             Expr::As(expr, type_name) => {
                 let _ = self.check_expr(expr)?;
                 // Return the target type if known, otherwise Object
                 Ok(self.context.get_type(type_name).unwrap_or_else(|_| self.context.get_type("Object").unwrap()))
             },
             Expr::VectorLiteral(elems) => {
                 for e in elems {
                     let _ = self.check_expr(e)?;
                 }
                 Ok(self.context.get_type("Object").unwrap())
             },
             Expr::VectorGenerator { expr, var, iterable } => {
                 let _ = self.check_expr(iterable)?;
                 let gen_scope = Rc::new(Scope::new_child(self.scope.clone()));
                 gen_scope.define_variable(var.clone(), self.context.get_type("Object").unwrap());
                 let mut gen_checker = BodyChecker::new(self.context, gen_scope);
                 let _ = gen_checker.check_expr(expr)?;
                 Ok(self.context.get_type("Object").unwrap())
             },
             Expr::Indexing { obj, index } => {
                 let _ = self.check_expr(obj)?;
                 let t_idx = self.check_expr(index)?;
                 if t_idx.borrow().name != "Number" {
                     return Err(vec![SemanticError::TypeMismatch{ expected: "Number".to_string(), found: t_idx.borrow().name.clone() }]);
                 }
                 Ok(self.context.get_type("Object").unwrap())
             },
             Expr::For { var, iterable, body } => {
                 let _ = self.check_expr(iterable)?;
                 let for_scope = Rc::new(Scope::new_child(self.scope.clone()));
                 for_scope.define_variable(var.clone(), self.context.get_type("Object").unwrap());
                 let mut for_checker = BodyChecker::new(self.context, for_scope);
                 let _ = for_checker.check_expr(body)?;
                 Ok(self.context.get_type("Object").unwrap())
             },
             Expr::Lambda { params, return_type: _, body } => {
                 let lambda_scope = Rc::new(Scope::new_child(self.scope.clone()));
                 for p in params {
                     let p_type = if let Some(ann) = &p.type_annotation {
                         self.context.resolve_type(ann).unwrap_or_else(|_| self.context.get_type("Object").unwrap())
                     } else {
                         self.context.get_type("Object").unwrap()
                     };
                     lambda_scope.define_variable(p.name.clone(), p_type);
                 }
                 let mut lambda_checker = BodyChecker::new(self.context, lambda_scope);
                 let _ = lambda_checker.check_expr(body)?;
                 Ok(self.context.get_type("Object").unwrap())
             },
             Expr::Match { expr, cases, default } => {
                 let _ = self.check_expr(expr)?;
                 for case in cases {
                     let _ = self.check_expr(&case.expr)?;
                 }
                 if let Some(d) = default {
                     let _ = self.check_expr(d)?;
                 }
                 Ok(self.context.get_type("Object").unwrap())
             },
             Expr::BaseCall { args } => {
                 for a in args {
                     let _ = self.check_expr(a)?;
                 }
                 Ok(self.context.get_type("Object").unwrap())
             },
             
            _ => Ok(self.context.get_type("Object").unwrap()), 
        }
    }
}


fn conforms_to(type_a: Rc<RefCell<Type>>, type_b: Rc<RefCell<Type>>) -> bool {
    type_a.borrow().conforms_to(&type_b)
}
