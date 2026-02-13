use crate::ast::nodes::*;
use crate::errors::SemanticError;
use super::types::{Type, TypeKind, TypeFactory};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

/// Contexto global del análisis semántico
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
    
    /// Obtiene la firma de una función (incluyendo built-ins)
    pub fn get_function(&self, name: &str) -> Option<(Vec<Rc<RefCell<Type>>>, Rc<RefCell<Type>>)> {
        // Funciones de la librería estándar
        if name == "print" {
            let obj = self.types.get("Object").unwrap().clone();
            let void = self.types.get("Object").unwrap().clone();
            return Some((vec![obj], void));
        }
        
        // Funciones matemáticas unarias
        if ["sin", "cos", "sqrt", "exp"].contains(&name) {
             let num = self.types.get("Number").unwrap().clone();
             return Some((vec![num.clone()], num));
        }

        if name == "rand" {
             let num = self.types.get("Number").unwrap().clone();
             return Some((vec![], num));
        }

        self.functions.get(name).cloned()
    }

    /// Resuelve una anotación de tipo al tipo real
    pub fn resolve_type(&self, annotation: &TypeAnnotation) -> Result<Rc<RefCell<Type>>, SemanticError> {
        match annotation {
            TypeAnnotation::Name(name) => self.get_type(name),
            TypeAnnotation::Iterable(inner) => {
                 let inner_type = self.resolve_type(inner)?;
                 let name = format!("Iterable<{}>", inner_type.borrow().name);
                 
                 // Crear 'al vuelo' el protocolo Iterable<T>
                 let iter = Rc::new(RefCell::new(Type::new(&name, TypeKind::Protocol, None)));
                 
                 let bool_type = self.get_type("Boolean")?;
                 iter.borrow_mut().define_method("next".to_string(), vec![], bool_type);
                 iter.borrow_mut().define_method("current".to_string(), vec![], inner_type);
                 
                 Ok(iter)
            },
            TypeAnnotation::Vector(inner) => {
                // Sistema de tipos genéricos para Vector<T>
                let inner_type = self.resolve_type(inner)?;
                let name = format!("Vector<{}>", inner_type.borrow().name);
                
                // Crear tipo Vector<T> al vuelo con métodos del protocolo Iterable
                let vec_type = Rc::new(RefCell::new(Type::new(&name, TypeKind::Basic, None)));
                
                let num_type = self.get_type("Number")?;
                let bool_type = self.get_type("Boolean")?;
                
                // Métodos del vector (implementa protocolo Iterable<T>)
                vec_type.borrow_mut().define_method("size".to_string(), vec![], num_type.clone());
                vec_type.borrow_mut().define_method("next".to_string(), vec![], bool_type);
                vec_type.borrow_mut().define_method("get_current".to_string(), vec![], inner_type.clone());
                
                // Almacenar el tipo de elemento como metadata para indexing
                vec_type.borrow_mut().define_attribute("__element_type".to_string(), inner_type);
                
                Ok(vec_type)
            },
            TypeAnnotation::Function { params, return_type } => {
                for p in params { self.resolve_type(p)?; }
                self.resolve_type(return_type)?;
                self.get_type("Object") // Tratar tipo función como Object
            }
        }
    }
}

/// Verifica si type_a conforma a type_b
pub fn conforms_to(type_a: Rc<RefCell<Type>>, type_b: Rc<RefCell<Type>>) -> bool {
    type_a.borrow().conforms_to(&type_b)
}
