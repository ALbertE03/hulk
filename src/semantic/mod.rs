pub mod types;
pub mod scope;

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
        // First check standard library functions if we had any
        if name == "print" {
            let obj = self.types.get("Object").unwrap().clone();
            let void = self.types.get("Object").unwrap().clone(); // Assuming print returns Object/Void
            return Some((vec![obj], void));
        }
        // Math functions
        if ["sin", "cos", "sqrt", "log", "exp"].contains(&name) {
             let num = self.types.get("Number").unwrap().clone();
             return Some((vec![num.clone()], num)); // Unary
        }
         if name == "log" { // Binary log(base, val) handled maybe? AST says Log(base, val)
             // AST has Log(base, val) which is binary.
             // But user might call `log(x)` ... wait, Context `get_function` is by name.
             // If builtin names collide, we need to check args count. Context returns ONE signature.
             // Assume standard lib is fixed
         }

        if name == "rand" {
             let num = self.types.get("Number").unwrap().clone();
             return Some((vec![], num));
        }

        self.functions.get(name).cloned()
    }

    // Resolve TypeAnnotation to actual Type
    pub fn resolve_type(&self, annotation: &TypeAnnotation) -> Result<Rc<RefCell<Type>>, SemanticError> {
        match annotation {
            TypeAnnotation::Name(name) => self.get_type(name),
            TypeAnnotation::Iterable(inner) => {
                 // Simplification: Check if inner type exists
                 let _ = self.resolve_type(inner)?;
                 // In a real implementation we would return a parameterized type
                 self.get_type("Object") 
            },
            TypeAnnotation::Function { params, return_type } => {
                for p in params { self.resolve_type(p)?; }
                self.resolve_type(return_type)?;
                self.get_type("Object") // Treat function type as Object
            }
        }
    }
}

pub fn check_program(program: &Program) -> Result<Context, Vec<SemanticError>> {
    let mut context = Context::new();
    let mut errors = Vec::new();

    // --- Pass 1: Collect Type and Protocol Names ---
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

    // --- Pass 2: Set Hierarchy (Parents) ---
    for decl in &program.declarations {
        match decl {
            Declaration::Type(type_decl) => {
                if let Some(parent_init) = &type_decl.parent {
                    match context.get_type(&parent_init.name) {
                        Ok(parent_type) => {
                             if parent_type.borrow().kind == TypeKind::Protocol {
                                 errors.push(SemanticError::GenericError(format!("Type {} cannot inherit from protocol {}", type_decl.name, parent_init.name)));
                             } else {
                                  match context.get_type(&type_decl.name) {
                                      Ok(child) => child.borrow_mut().parent = Some(parent_type.clone()),
                                      Err(_) => {} // Should not happen
                                  }
                             }
                        },
                        Err(e) => errors.push(e),
                    }
                } else {
                    // Default inherits Object (except Object itself)
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
                                   errors.push(SemanticError::GenericError(format!("Protocol {} cannot inherit from basic type {}", proto_decl.name, parent_name)));
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
    
    
// Check for cycles
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

    // --- Pass 3: Collect Features (Methods/Attributes) & Global Functions ---
    
    // 3a. Global Functions
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
                None => context.get_type("Object").unwrap(), // Default return Object? 
            };
            
            if let Err(e) = context.define_function(&func_decl.name, params, ret_type) {
                errors.push(e);
            }
        }
    }
    
    // 3b. Type Features
    for decl in &program.declarations {
         if let Declaration::Type(type_decl) = decl {
             let current_type_rc = context.get_type(&type_decl.name).unwrap();
             
             // Constructor Params
             let mut ctor_params = Vec::new();
             for p in &type_decl.params {
                  let p_type = match &p.type_annotation {
                    Some(ann) => match context.resolve_type(ann) { Ok(t) => t, Err(e) => { errors.push(e); context.get_type("Object").unwrap() } },
                    None => context.get_type("Object").unwrap(),
                  };
                  ctor_params.push((p.name.clone(), p_type));
             }
             current_type_rc.borrow_mut().params = ctor_params;

             // Attributes
             for attr in &type_decl.attributes {
                 let attr_type = match &attr.type_annotation {
                     Some(ann) => match context.resolve_type(ann) {
                         Ok(t) => t,
                         Err(e) => { errors.push(e); context.get_type("Object").unwrap() }
                     },
                     None => context.get_type("Object").unwrap(),
                 };
                 // Add attribute to type
                 if current_type_rc.borrow().attributes.contains_key(&attr.name) {
                     errors.push(SemanticError::AttributeDefined(format!("{}.{}", type_decl.name, attr.name)));
                 } else {
                     current_type_rc.borrow_mut().attributes.insert(attr.name.clone(), attr_type);
                 }
             }
             
             // Methods
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
                     // Check override
                     let mut parent = current_type_rc.borrow().parent.clone();
                     while let Some(p) = parent {
                         if let Some(parent_method) = p.borrow().methods.get(&method.name) {
                             if parent_method.params.len() != method_info.params.len() {
                                 errors.push(SemanticError::SignatureMismatch(format!("{}.{} overrides with different arg count", type_decl.name, method.name)));
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

    // --- Pass 4: Check Bodies (Functions & Methods) ---
    
    // Check Global Expression
    {
        // Wrap scope in Rc for sharing
        let scope = Rc::new(Scope::new());
        let mut checker = BodyChecker::new(&context, scope);
        if let Err(mut body_errors) = checker.check_expr(&program.expr) {
            errors.append(&mut body_errors);
        }
    }
    
    // Check Global Functions
    for decl in &program.declarations {
         if let Declaration::Function(func) = decl {
             let scope = Rc::new(Scope::new());
             // Add params to scope
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
    
    // TODO: Check Type Methods bodies and Attribute initializations
    // Requires visiting checking TypeDecl bodies again and setting `current_type`

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
    
    // Return resolved type on success
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
                    // Constant?
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
                   } else {
                        if matches!(op, Op::Eq | Op::Neq) {
                             Ok(self.context.get_type("Boolean").unwrap())
                        } else {
                             Err(vec![SemanticError::TypeMismatch { expected: "Math/Logic operands".to_string(), found: format!("{} {:?}", l.borrow().name, op) } ])
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
                    // Assuming let bindings see previous bindings in same block
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
                } else {
                    Err(vec![SemanticError::FunctionNotFound(name.clone(), args.len())])
                }
            },
             Expr::If{cond, then_expr, else_expr} => {
                 let t_cond = self.check_expr(cond)?;
                 if t_cond.borrow().name != "Boolean" {
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
             Expr::While { cond, body } => {
                  let t_cond = self.check_expr(cond)?;
                  if t_cond.borrow().name != "Boolean" {
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
             
            _ => Ok(self.context.get_type("Object").unwrap()), 
        }
    }
}


fn conforms_to(type_a: Rc<RefCell<Type>>, type_b: Rc<RefCell<Type>>) -> bool {
    type_a.borrow().conforms_to(&type_b)
}
