use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use crate::errors::SemanticError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeKind {
    Basic,      // Tipos primitivos y clases definidas por usuario
    Protocol,   // Protocolos
}

#[derive(Debug, Clone)]
pub struct Type {
    pub name: String,
    pub kind: TypeKind,
    pub parent: Option<Rc<RefCell<Type>>>,
    pub methods: HashMap<String, MethodInfo>,
    pub attributes: HashMap<String, Rc<RefCell<Type>>>,
    pub params: Vec<(String, Rc<RefCell<Type>>)>, // Constructor params
}

#[derive(Debug, Clone)]
pub struct MethodInfo {
    pub name: String,
    pub params: Vec<(String, Rc<RefCell<Type>>)>,
    pub return_type: Rc<RefCell<Type>>,
}

impl Type {
    pub fn new(name: &str, kind: TypeKind, parent: Option<Rc<RefCell<Type>>>) -> Self {
        Type {
            name: name.to_string(),
            kind,
            parent,
            methods: HashMap::new(),
            attributes: HashMap::new(),
            params: Vec::new(),
        }
    }

    /// Verifica si este tipo conforma al tipo `other`.
    /// 
    /// - Si `other` es una CLASE: `self` debe ser subclase de `other` (herencia).
    /// - Si `other` es un PROTOCOLO: `self` debe implementar todos los métodos de `other`.
    pub fn conforms_to(&self, other: &Rc<RefCell<Type>>) -> bool {
        let other = other.borrow();
        
        // Caso 1: Mismo tipo
        if self.name == other.name {
            return true;
        }

        // Caso 2: El otro es Object (todo hereda de Object implícitamente)
        if other.name == "Object" {
            return true;
        }

        match other.kind {
            TypeKind::Basic => {
                // Herencia: subir por la cadena de padres
                if let Some(parent) = &self.parent {
                    return parent.borrow().conforms_to(&Rc::new(RefCell::new(other.clone())));
                }
                false
            }
            TypeKind::Protocol => {
                // Implementación estructural: verificar que tenemos todos los métodos del protocolo
                for (method_name, method_info) in &other.methods {
                    if let Some(my_method) = self.get_method(method_name) {
                        // Verificar firma: cantidad de argumentos
                        if my_method.params.len() != method_info.params.len() {
                            return false;
                        }
                        
                        // Argumentos contravariantes (mi argumento debe ser >= argumento del proto)
                        // y Retorno covariante (mi retorno debe ser <= retorno del proto)
                        // Por simplicidad inicial: pedimos tipos exactos o conformidad básica
                        
                        // Check return type conformance
                        if !my_method.return_type.borrow().conforms_to(&method_info.return_type) {
                            return false;
                        }

                        // Check params conformance
                        for (i, (_, param_type)) in my_method.params.iter().enumerate() {
                            let (_, expected_param_type) = &method_info.params[i];
                            // Contravariance: expected param conforms to actual param
                            if !expected_param_type.borrow().conforms_to(param_type) {
                                return false;
                            }
                        }
                    } else {
                        return false; // Método faltante
                    }
                }
                true
            }
        }
    }

    pub fn get_method(&self, name: &str) -> Option<MethodInfo> {
        if let Some(method) = self.methods.get(name) {
            return Some(method.clone());
        }
        
        if let Some(parent) = &self.parent {
            return parent.borrow().get_method(name);
        }

        None
    }

    pub fn define_method(&mut self, name: String, params: Vec<(String, Rc<RefCell<Type>>)>, return_type: Rc<RefCell<Type>>) {
        self.methods.insert(name.clone(), MethodInfo { name, params, return_type });
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Type {}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

// Factory helpers
pub struct TypeFactory;

impl TypeFactory {
    pub fn object() -> Rc<RefCell<Type>> {
        Rc::new(RefCell::new(Type::new("Object", TypeKind::Basic, None)))
    }

    pub fn number(object: Rc<RefCell<Type>>) -> Rc<RefCell<Type>> {
        Rc::new(RefCell::new(Type::new("Number", TypeKind::Basic, Some(object))))
    }
    
    pub fn string(object: Rc<RefCell<Type>>) -> Rc<RefCell<Type>> {
        Rc::new(RefCell::new(Type::new("String", TypeKind::Basic, Some(object))))
    }

    pub fn boolean(object: Rc<RefCell<Type>>) -> Rc<RefCell<Type>> {
        Rc::new(RefCell::new(Type::new("Boolean", TypeKind::Basic, Some(object))))
    }
}

pub fn lowest_common_ancestor(type_a: Rc<RefCell<Type>>, type_b: Rc<RefCell<Type>>) -> Result<Rc<RefCell<Type>>, Vec<SemanticError>> {
    if type_a.borrow().conforms_to(&type_b) { return Ok(type_b); }
    if type_b.borrow().conforms_to(&type_a) { return Ok(type_a); }
    
    // Climb up A
    let mut curr_a = type_a;
    loop {
         let parent = curr_a.borrow().parent.clone();
         if let Some(p) = parent {
             if type_b.borrow().conforms_to(&p) {
                 return Ok(p);
             }
             curr_a = p;
         } else {
             break;
         }
    }
    
    Ok(curr_a) // Should be Object
}
