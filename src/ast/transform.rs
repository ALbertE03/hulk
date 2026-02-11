// Módulo para transformaciones del AST
use crate::ast::nodes::*;
use crate::semantic::Context;
use crate::utils::{Spanned, Position};
use std::collections::HashMap;

/// Transforma el AST para soportar implicit functor implementation
pub fn transform_implicit_functors(program: &mut Program, _context: &Context) {
    let mut transformer = ImplicitFunctorTransformer::new();
    
    // Identificar funciones disponibles del AST
    for decl in &program.declarations {
        if let Declaration::Function(fd) = decl {
            transformer.register_function(&fd.name, &fd.params, fd.return_type.as_ref());
        }
    }
    
    // Identificar protocolos con invoke del AST
    for decl in &program.declarations {
        if let Declaration::Protocol(pd) = decl {
            if pd.methods.iter().any(|m| m.name == "invoke") {
                // Guardar la info del protocolo
                let invoke_method = pd.methods.iter().find(|m| m.name == "invoke").unwrap();
                transformer.functor_protocols.insert(
                    pd.name.clone(),
                    (invoke_method.params.clone(), Some(invoke_method.return_type.clone()))
                );
            }
        }
    }
    
    // Transformar la expresión principal
    transformer.transform_expr(&mut program.expr);
    
    // Agregar los wrappers generados a las declaraciones
    for wrapper_decl in transformer.generated_wrappers.into_values() {
        program.declarations.push(Declaration::Type(wrapper_decl));
    }
}

struct ImplicitFunctorTransformer {
    generated_wrappers: HashMap<String, TypeDecl>,
    next_wrapper_id: usize,
    functions: HashMap<String, (Vec<Param>, Option<TypeAnnotation>)>,
    functor_protocols: HashMap<String, (Vec<Param>, Option<TypeAnnotation>)>,
}

impl ImplicitFunctorTransformer {
    fn new() -> Self {
        Self {
            generated_wrappers: HashMap::new(),
            next_wrapper_id: 0,
            functions: HashMap::new(),
            functor_protocols: HashMap::new(),
        }
    }
    
    fn register_function(&mut self, name: &str, params: &[Param], ret_type: Option<&TypeAnnotation>) {
        self.functions.insert(name.to_string(), (params.to_vec(), ret_type.cloned()));
    }
    
    fn transform_expr(&mut self, expr: &mut Spanned<Expr>) {
        match &mut expr.node {
            Expr::Call { func: _, args } => {
                // Transformar argumentos primero (bottom-up)
                for arg in args.iter_mut() {
                    self.transform_expr(arg);
                }
                
                // Verificar cada argumento para ver si es una función que debe convertirse
                // Clonar la info necesaria para evitar borrow checker issues
                let protocol_names: Vec<(String, Vec<Param>, Option<TypeAnnotation>)> = 
                    self.functor_protocols.iter()
                        .map(|(name, (params, ret))| (name.clone(), params.clone(), ret.clone()))
                        .collect();
                
                for arg in args.iter_mut() {
                    if let Expr::Identifier(func_name) = &arg.node {
                        // ¿Es una función conocida?
                        if self.functions.contains_key(func_name) {
                            // Buscar si algún protocolo functor coincide con la firma
                            for (protocol_name, params, ret) in &protocol_names {
                                let wrapper_name = self.generate_wrapper(
                                    func_name,
                                    protocol_name,
                                    &(params.clone(), ret.clone())
                                );
                                
                                // Reemplazar por instanciación
                                arg.node = Expr::Instantiation {
                                    ty: wrapper_name,
                                    args: vec![],
                                };
                                break; // Solo generar un wrapper por función
                            }
                        }
                    }
                }
            }
            Expr::Let { bindings, body } => {
                for (_name, type_ann, init_expr) in bindings.iter_mut() {
                    // Si la anotación de tipo es un protocolo functor y la expresión es un identificador
                    if let Some(TypeAnnotation::Name(type_name)) = type_ann {
                        // Clonar para evitar borrow issues
                        let protocol_sig = self.functor_protocols.get(type_name).cloned();
                        if let Some(sig) = protocol_sig {
                            if let Expr::Identifier(func_name) = &init_expr.node {
                                if self.functions.contains_key(func_name) {
                                    let wrapper_name = self.generate_wrapper(
                                        func_name,
                                        type_name,
                                        &sig
                                    );
                                    init_expr.node = Expr::Instantiation {
                                        ty: wrapper_name,
                                        args: vec![],
                                    };
                                }
                            }
                        }
                    }
                    self.transform_expr(init_expr);
                }
                self.transform_expr(body);
            }
            Expr::If { cond, then_expr, else_expr } => {
                self.transform_expr(cond);
                self.transform_expr(then_expr);
                self.transform_expr(else_expr);
            }
            Expr::While { cond, body } => {
                self.transform_expr(cond);
                self.transform_expr(body);
            }
            Expr::For { var: _, iterable, body } => {
                self.transform_expr(iterable);
                self.transform_expr(body);
            }
            Expr::Block(exprs) => {
                for e in exprs.iter_mut() {
                    self.transform_expr(e);
                }
            }
            Expr::MethodCall { obj, method: _, args } => {
                self.transform_expr(obj);
                for arg in args.iter_mut() {
                    self.transform_expr(arg);
                }
            }
            Expr::Assignment { target: _, value } => {
                self.transform_expr(value);
            }
            Expr::Binary(left, _op, right) => {
                self.transform_expr(left);
                self.transform_expr(right);
            }
            Expr::Unary(_op, operand) => {
                self.transform_expr(operand);
            }
            Expr::Instantiation { ty: _, args } => {
                for arg in args.iter_mut() {
                    self.transform_expr(arg);
                }
            }
            Expr::Indexing { obj, index } => {
                self.transform_expr(obj);
                self.transform_expr(index);
            }
            Expr::As(e, _) => {
                self.transform_expr(e);
            }
            Expr::Is(e, _) => {
                self.transform_expr(e);
            }
            Expr::BaseCall { args } => {
                for arg in args.iter_mut() {
                    self.transform_expr(arg);
                }
            }
            _ => {}
        }
    }
    
    fn generate_wrapper(
        &mut self,
        func_name: &str,
        protocol_name: &str,
        protocol_sig: &(Vec<Param>, Option<TypeAnnotation>)
    ) -> String {
        // Verificar si ya generamos un wrapper para esta función+protocolo
        let key = format!("{}_{}", func_name, protocol_name);
        if let Some(wrapper_decl) = self.generated_wrappers.get(&key) {
            return wrapper_decl.name.clone();
        }
        
        // Generar nombre único para el wrapper
        let wrapper_name = format!("_{}Wrapper{}", 
            func_name.chars().next().unwrap().to_uppercase().collect::<String>() 
                + &func_name[1..], 
            self.next_wrapper_id
        );
        self.next_wrapper_id += 1;
        
        // Usar la firma del protocolo
        let (invoke_params, invoke_ret_type) = protocol_sig;
        
        // Crear el método invoke que llama a la función original
        let invoke_body = Spanned {
            node: Expr::Call {
                func: func_name.to_string(),
                args: invoke_params.iter().map(|p| Spanned {
                    node: Expr::Identifier(p.name.clone()),
                    pos: Position { line: 0, column: 0 },
                }).collect(),
            },
            pos: Position { line: 0, column: 0 },
        };
        
        let method = FunctionDecl {
            name: "invoke".to_string(),
            params: invoke_params.clone(),
            return_type: invoke_ret_type.clone(),
            body: invoke_body,
        };
        
        // Crear el TypeDecl para el wrapper
        let wrapper_decl = TypeDecl {
            name: wrapper_name.clone(),
            params: vec![],
            parent: None,
            attributes: vec![],
            methods: vec![method],
        };
        
        self.generated_wrappers.insert(key, wrapper_decl);
        wrapper_name
    }
}
