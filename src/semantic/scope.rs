use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use super::types::Type;

#[derive(Debug)]
pub struct Scope {
    pub parent: Option<Rc<Scope>>,
    pub variables: RefCell<HashMap<String, Rc<RefCell<Type>>>>,
    pub functions: RefCell<HashMap<String, (Vec<Rc<RefCell<Type>>>, Rc<RefCell<Type>>)>>, // (ParamTypes, ReturnType)
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            parent: None,
            variables: RefCell::new(HashMap::new()),
            functions: RefCell::new(HashMap::new()),
        }
    }

    pub fn new_child(parent: Rc<Scope>) -> Self {
        Scope {
            parent: Some(parent),
            variables: RefCell::new(HashMap::new()),
            functions: RefCell::new(HashMap::new()),
        }
    }

    /// Define una variable en el ámbito actual.
    /// Retorna `true` si se definió con éxito, `false` si ya existía.
    pub fn define_variable(&self, name: String, type_info: Rc<RefCell<Type>>) -> bool {
        let mut vars = self.variables.borrow_mut();
        if vars.contains_key(&name) {
            return false; // Ya existe en este scope
        }
        vars.insert(name, type_info);
        true
    }

    /// Busca una variable recursivamente en los ámbitos padres.
    pub fn find_variable(&self, name: &str) -> Option<Rc<RefCell<Type>>> {
        if let Some(t) = self.variables.borrow().get(name) {
            return Some(t.clone());
        }
        if let Some(parent) = &self.parent {
            return parent.find_variable(name);
        }
        None
    }

    pub fn is_defined(&self, name: &str) -> bool {
        self.find_variable(name).is_some()
    }
    
    pub fn define_function(&self, name: String, params: Vec<Rc<RefCell<Type>>>, return_type: Rc<RefCell<Type>>) -> bool {
        let mut funcs = self.functions.borrow_mut();
         if funcs.contains_key(&name) {
            return false;
        }
        funcs.insert(name, (params, return_type));
        true
    }
    
    pub fn find_function(&self, name: &str) -> Option<(Vec<Rc<RefCell<Type>>>, Rc<RefCell<Type>>)> {
        if let Some(f) = self.functions.borrow().get(name) {
            return Some(f.clone());
        }
        if let Some(parent) = &self.parent {
            return parent.find_function(name);
        }
        None
    }
}
