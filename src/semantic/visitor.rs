use crate::ast::nodes::*;
use crate::utils::Spanned;
use crate::errors::SemanticError;
use super::context::{Context, conforms_to};
use super::types::{Type, lowest_common_ancestor};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

/// Visitor para inferencia de tipos en el AST
pub struct TypeChecker<'a> {
    context: &'a Context,
    scope: Vec<HashMap<String, Rc<RefCell<Type>>>>,
    pub current_type: Option<Rc<RefCell<Type>>>,
}

impl<'a> TypeChecker<'a> {
    pub fn new(context: &'a Context) -> Self {
        TypeChecker { 
            context, 
            scope: vec![HashMap::new()],
            current_type: None,
        }
    }
    
    pub fn enter_scope(&mut self) {
        self.scope.push(HashMap::new());
    }
    
    pub fn exit_scope(&mut self) {
        self.scope.pop();
    }
    
    pub fn define(&mut self, name: String, ty: Rc<RefCell<Type>>) {
        if let Some(current) = self.scope.last_mut() {
            current.insert(name, ty);
        }
    }
    
    pub fn lookup(&self, name: &str) -> Option<Rc<RefCell<Type>>> {
        for scope in self.scope.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return Some(ty.clone());
            }
        }
        None
    }
    
    fn is_compatible(type_a: &Rc<RefCell<Type>>, type_b: &Rc<RefCell<Type>>) -> bool {
        type_a.borrow().conforms_to(type_b)
    }
    
    pub fn infer_type(&mut self, expr: &Spanned<Expr>) -> Result<Rc<RefCell<Type>>, Vec<SemanticError>> {
        match &expr.node {
            Expr::Number(_) => self.visit_number(expr.pos),
            Expr::String(_) => self.visit_string(expr.pos),
            Expr::Boolean(_) => self.visit_boolean(expr.pos),
            Expr::Identifier(name) => self.visit_identifier(name, expr.pos),
            Expr::Binary(left, op, right) => self.visit_binary(left, op, right, expr.pos),
            Expr::Unary(op, operand) => self.visit_unary(op, operand, expr.pos),
            Expr::Let { bindings, body } => self.visit_let(bindings, body, expr.pos),
            Expr::Block(exprs) => self.visit_block(exprs, expr.pos),
            Expr::Call { func, args } => self.visit_call(func, args, expr.pos),
            Expr::If { cond, then_expr, else_expr } => self.visit_if(cond, then_expr, else_expr, expr.pos),
            Expr::While { cond, body } => self.visit_while(cond, body, expr.pos),
            Expr::For { var, iterable, body } => self.visit_for(var, iterable, body, expr.pos),
            Expr::Lambda { params, return_type, body } => self.visit_lambda(params, return_type, body, expr.pos),
            Expr::Assignment { target, value } => self.visit_assignment(target, value, expr.pos),
            Expr::AttributeAssignment { obj, attribute, value } => self.visit_attribute_assignment(obj, attribute, value, expr.pos),
            Expr::Instantiation { ty, args } => self.visit_instantiation(ty, args, expr.pos),
            Expr::MethodCall { obj, method, args } => self.visit_method_call(obj, method, args, expr.pos),
            Expr::AttributeAccess { obj, attribute } => self.visit_attribute_access(obj, attribute, expr.pos),
            Expr::VectorLiteral(elems) => self.visit_vector_literal(elems, expr.pos),
            Expr::VectorGenerator { expr, var, iterable } => self.visit_vector_generator(expr, var, iterable, expr.pos),
            Expr::Indexing { obj, index } => self.visit_indexing(obj, index, expr.pos),
            Expr::Match { expr, cases, default } => self.visit_match(expr, cases, default, expr.pos),
            Expr::Is(e, ty) => self.visit_is(e, ty, expr.pos),
            Expr::As(e, ty) => self.visit_as(e, ty, expr.pos),
            Expr::PI => self.visit_pi(expr.pos),
            Expr::E => self.visit_e(expr.pos),
            Expr::Rand => self.visit_rand(expr.pos),
            Expr::Sin(e) => self.visit_sin(e, expr.pos),
            Expr::Cos(e) => self.visit_cos(e, expr.pos),
            Expr::Sqrt(e) => self.visit_sqrt(e, expr.pos),
            Expr::Exp(e) => self.visit_exp(e, expr.pos),
            Expr::Log(base, val) => self.visit_log(base, val, expr.pos),
            Expr::BaseCall { args } => self.visit_base_call(args, expr.pos),
            _ => Ok(self.context.get_type("Object").unwrap()),
        }
    }
    
 
    fn visit_number(&mut self, _pos: crate::utils::Position) -> Result<Rc<RefCell<Type>>, Vec<SemanticError>> {
        Ok(self.context.get_type("Number").unwrap())
    }
    
    fn visit_string(&mut self, _pos: crate::utils::Position) -> Result<Rc<RefCell<Type>>, Vec<SemanticError>> {
        Ok(self.context.get_type("String").unwrap())
    }
    
    fn visit_boolean(&mut self, _pos: crate::utils::Position) -> Result<Rc<RefCell<Type>>, Vec<SemanticError>> {
        Ok(self.context.get_type("Boolean").unwrap())
    }
    
    fn visit_identifier(&mut self, name: &str, _pos: crate::utils::Position) -> Result<Rc<RefCell<Type>>, Vec<SemanticError>> {
        if name == "self" {
            if let Some(t) = &self.current_type {
                Ok(t.clone())
            } else {
                Err(vec![SemanticError::SelfReference])
            }
        } else if let Some(t) = self.lookup(name) {
            Ok(t)
        } else if name == "PI" || name == "E" {
            Ok(self.context.get_type("Number").unwrap())
        } else {
            Err(vec![SemanticError::VariableNotFound(name.to_string())])
        }
    }
    
    fn visit_binary(&mut self, left: &Spanned<Expr>, op: &Op, right: &Spanned<Expr>, _pos: crate::utils::Position) -> Result<Rc<RefCell<Type>>, Vec<SemanticError>> {
        let t_left = self.infer_type(left);
        let t_right = self.infer_type(right);
        
        if t_left.is_ok() && t_right.is_ok() {
            let l = t_left.unwrap();
            let r = t_right.unwrap();
            
            if l.borrow().name == "Number" && r.borrow().name == "Number" {
                match op {
                    Op::Add | Op::Sub | Op::Mul | Op::Div | Op::Mod | Op::Pow => 
                        Ok(self.context.get_type("Number").unwrap()),
                    Op::Eq | Op::Neq | Op::Lt | Op::Gt | Op::Le | Op::Ge => 
                        Ok(self.context.get_type("Boolean").unwrap()),
                    _ => Err(vec![SemanticError::OperationNotDefined(format!("{:?}", op), "Number".to_string())])
                }
            } else if l.borrow().name == "Boolean" && r.borrow().name == "Boolean" {
                match op {
                    Op::And | Op::Or | Op::Eq | Op::Neq => 
                        Ok(self.context.get_type("Boolean").unwrap()),
                    _ => Err(vec![SemanticError::OperationNotDefined(format!("{:?}", op), "Boolean".to_string())])
                }
            } else if matches!(op, Op::Concat | Op::ConcatSpace) {
                Ok(self.context.get_type("String").unwrap())
            } else if l.borrow().name == "Object" || r.borrow().name == "Object" {
                match op {
                    Op::Add | Op::Sub | Op::Mul | Op::Div | Op::Mod | Op::Pow => 
                        Ok(self.context.get_type("Number").unwrap()),
                    Op::Eq | Op::Neq | Op::Lt | Op::Gt | Op::Le | Op::Ge | Op::And | Op::Or => 
                        Ok(self.context.get_type("Boolean").unwrap()),
                    _ => Ok(self.context.get_type("Object").unwrap()),
                }
            } else if matches!(op, Op::Eq | Op::Neq) {
                Ok(self.context.get_type("Boolean").unwrap())
            } else {
                Err(vec![SemanticError::GenericError(format!("Operation {:?} is not defined for types {} and {}", op, l.borrow().name, r.borrow().name))])
            }
        } else {
            let mut errs = Vec::new();
            if let Err(mut e) = t_left { errs.append(&mut e); }
            if let Err(mut e) = t_right { errs.append(&mut e); }
            Err(errs)
        }
    }
    
    fn visit_unary(&mut self, op: &UnOp, operand: &Spanned<Expr>, _pos: crate::utils::Position) -> Result<Rc<RefCell<Type>>, Vec<SemanticError>> {
        let t_op = self.infer_type(operand)?;
        if t_op.borrow().name == "Number" && matches!(op, UnOp::Neg) {
            Ok(t_op)
        } else if t_op.borrow().name == "Boolean" && matches!(op, UnOp::Not) {
            Ok(t_op)
        } else {
            Err(vec![SemanticError::OperationNotDefined(format!("{:?}", op), t_op.borrow().name.clone())])
        }
    }
    
    fn visit_let(&mut self, bindings: &[(String, Option<TypeAnnotation>, Spanned<Expr>)], body: &Spanned<Expr>, _pos: crate::utils::Position) -> Result<Rc<RefCell<Type>>, Vec<SemanticError>> {
        self.enter_scope();
        
        for (name, type_ann, expr_span) in bindings {
            let t_expr = self.infer_type(expr_span)?;
            
            if let Some(ann) = type_ann {
                let t_ann = self.context.resolve_type(ann).map_err(|e| vec![e])?;
                if !Self::is_compatible(&t_expr, &t_ann) {
                    self.exit_scope();
                    return Err(vec![SemanticError::TypeMismatch { 
                        expected: t_ann.borrow().name.clone(), 
                        found: t_expr.borrow().name.clone(), 
                        pos: expr_span.pos 
                    }]);
                }
            }
            self.define(name.clone(), t_expr);
        }
        
        let result = self.infer_type(body);
        self.exit_scope();
        result
    }
    
    fn visit_block(&mut self, exprs: &[Spanned<Expr>], _pos: crate::utils::Position) -> Result<Rc<RefCell<Type>>, Vec<SemanticError>> {
        let mut last_type = self.context.get_type("Object").unwrap(); 
        for e in exprs {
            last_type = self.infer_type(e)?;
        }
        Ok(last_type)
    }
    
    fn visit_call(&mut self, name: &str, args: &[Spanned<Expr>], _pos: crate::utils::Position) -> Result<Rc<RefCell<Type>>, Vec<SemanticError>> {
        if let Some((param_types, ret_type)) = self.context.get_function(name) {
            if args.len() != param_types.len() {
                return Err(vec![SemanticError::ArgumentCountMismatch(name.to_string(), param_types.len(), args.len())]);
            }
            
            for (i, arg_expr) in args.iter().enumerate() {
                let arg_type = self.infer_type(arg_expr)?;
                if !conforms_to(arg_type.clone(), param_types[i].clone()) {
                    return Err(vec![SemanticError::TypeMismatch{ 
                        expected: param_types[i].borrow().name.clone(), 
                        found: arg_type.borrow().name.clone(), 
                        pos: arg_expr.pos 
                    }]);
                }
            }
            Ok(ret_type)
        } else if self.lookup(name).is_some() {
            // Variable en scope - podría ser lambda
            for arg_expr in args {
                let _ = self.infer_type(arg_expr)?;
            }
            Ok(self.context.get_type("Object").unwrap())
        } else {
            Err(vec![SemanticError::FunctionNotFound(name.to_string(), args.len())])
        }
    }
    
    fn visit_if(&mut self, cond: &Spanned<Expr>, then_expr: &Spanned<Expr>, else_expr: &Spanned<Expr>, _pos: crate::utils::Position) -> Result<Rc<RefCell<Type>>, Vec<SemanticError>> {
        let t_cond = self.infer_type(cond)?;
        
        if t_cond.borrow().name != "Boolean" && t_cond.borrow().name != "Object" {
            return Err(vec![SemanticError::TypeMismatch{ 
                expected: "Boolean".to_string(), 
                found: t_cond.borrow().name.clone(), 
                pos: cond.pos 
            }]);
        }
        
        let t_then = self.infer_type(then_expr)?;
        let t_else = self.infer_type(else_expr)?;
        
        lowest_common_ancestor(t_then, t_else)
    }
    
    fn visit_while(&mut self, cond: &Spanned<Expr>, body: &Spanned<Expr>, _pos: crate::utils::Position) -> Result<Rc<RefCell<Type>>, Vec<SemanticError>> {
        let t_cond = self.infer_type(cond)?;
        
        if t_cond.borrow().name != "Boolean" && t_cond.borrow().name != "Object" {
            return Err(vec![SemanticError::TypeMismatch{ 
                expected: "Boolean".to_string(), 
                found: t_cond.borrow().name.clone(), 
                pos: cond.pos 
            }]);
        }
        
        let _ = self.infer_type(body)?;
        Ok(self.context.get_type("Object").unwrap())
    }
    
    fn visit_for(&mut self, var: &str, iterable: &Spanned<Expr>, body: &Spanned<Expr>, _pos: crate::utils::Position) -> Result<Rc<RefCell<Type>>, Vec<SemanticError>> {
        let _ = self.infer_type(iterable)?;
        
        self.enter_scope();
        self.define(var.to_string(), self.context.get_type("Object").unwrap());
        let result = self.infer_type(body);
        self.exit_scope();
        
        result?;
        Ok(self.context.get_type("Object").unwrap())
    }
    
    fn visit_lambda(&mut self, params: &[Param], _return_type: &Option<TypeAnnotation>, body: &Spanned<Expr>, _pos: crate::utils::Position) -> Result<Rc<RefCell<Type>>, Vec<SemanticError>> {
        self.enter_scope();
        
        for p in params {
            let p_type = if let Some(ann) = &p.type_annotation {
                self.context.resolve_type(ann).unwrap_or_else(|_| self.context.get_type("Object").unwrap())
            } else {
                self.context.get_type("Object").unwrap()
            };
            self.define(p.name.clone(), p_type);
        }
        
        let result = self.infer_type(body);
        self.exit_scope();
        
        result?;
        Ok(self.context.get_type("Object").unwrap())
    }
    
    fn visit_assignment(&mut self, target: &str, value: &Spanned<Expr>, _pos: crate::utils::Position) -> Result<Rc<RefCell<Type>>, Vec<SemanticError>> {
        if let Some(t_var) = self.lookup(target) {
            let t_val = self.infer_type(value)?;
            if !conforms_to(t_val.clone(), t_var.clone()) {
                return Err(vec![SemanticError::TypeMismatch{ 
                    expected: t_var.borrow().name.clone(), 
                    found: t_val.borrow().name.clone(), 
                    pos: value.pos 
                }]);
            }
            Ok(t_val)
        } else {
            Err(vec![SemanticError::VariableNotFound(target.to_string())])
        }
    }
    
    fn visit_attribute_assignment(&mut self, obj: &Spanned<Expr>, attribute: &str, value: &Spanned<Expr>, _pos: crate::utils::Position) -> Result<Rc<RefCell<Type>>, Vec<SemanticError>> {
        let t_obj = self.infer_type(obj)?;
        let t_val = self.infer_type(value)?;
        
        let mut curr = Some(t_obj.clone());
        while let Some(c) = curr {
            if let Some(attr_type) = c.borrow().attributes.get(attribute) {
                if !conforms_to(t_val.clone(), attr_type.clone()) {
                    return Err(vec![SemanticError::TypeMismatch{ 
                        expected: attr_type.borrow().name.clone(), 
                        found: t_val.borrow().name.clone(), 
                        pos: value.pos 
                    }]);
                }
                return Ok(t_val);
            }
            curr = c.borrow().parent.clone();
        }
        
        Ok(t_val)
    }
    
    fn visit_instantiation(&mut self, ty: &str, args: &[Spanned<Expr>], _pos: crate::utils::Position) -> Result<Rc<RefCell<Type>>, Vec<SemanticError>> {
        let type_rc = self.context.get_type(ty).map_err(|e| vec![e])?;
        let params = type_rc.borrow().params.clone();
        
        if args.len() != params.len() {
            return Err(vec![SemanticError::ArgumentCountMismatch(ty.to_string(), params.len(), args.len())]);
        }
        
        for (i, arg_expr) in args.iter().enumerate() {
            let arg_type = self.infer_type(arg_expr)?;
            if !conforms_to(arg_type.clone(), params[i].1.clone()) {
                return Err(vec![SemanticError::TypeMismatch{ 
                    expected: params[i].1.borrow().name.clone(), 
                    found: arg_type.borrow().name.clone(), 
                    pos: arg_expr.pos 
                }]);
            }
        }
        
        Ok(type_rc)
    }
    
    fn visit_method_call(&mut self, obj: &Spanned<Expr>, method: &str, args: &[Spanned<Expr>], _pos: crate::utils::Position) -> Result<Rc<RefCell<Type>>, Vec<SemanticError>> {
        let t_obj = self.infer_type(obj)?;
        
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
                return Err(vec![SemanticError::ArgumentCountMismatch(method.to_string(), m.params.len(), args.len())]);
            }
            
            for (i, arg_expr) in args.iter().enumerate() {
                let arg_type = self.infer_type(arg_expr)?;
                if !conforms_to(arg_type.clone(), m.params[i].1.clone()) {
                    return Err(vec![SemanticError::TypeMismatch{ 
                        expected: m.params[i].1.borrow().name.clone(), 
                        found: arg_type.borrow().name.clone(), 
                        pos: arg_expr.pos 
                    }]);
                }
            }
            
            Ok(m.return_type.clone())
        } else {
            Err(vec![SemanticError::MethodNotFound(format!("Method {} not found in {}", method, t_obj.borrow().name))])
        }
    }
    
    fn visit_attribute_access(&mut self, obj: &Spanned<Expr>, attribute: &str, _pos: crate::utils::Position) -> Result<Rc<RefCell<Type>>, Vec<SemanticError>> {
        let t_obj = self.infer_type(obj)?;
        
        let mut curr = Some(t_obj.clone());
        while let Some(c) = curr {
            if let Some(attr_type) = c.borrow().attributes.get(attribute) {
                return Ok(attr_type.clone());
            }
            curr = c.borrow().parent.clone();
        }
        
        Ok(self.context.get_type("Object").unwrap())
    }
    
    fn visit_vector_literal(&mut self, elems: &[Spanned<Expr>], _pos: crate::utils::Position) -> Result<Rc<RefCell<Type>>, Vec<SemanticError>> {
        for e in elems {
            let _ = self.infer_type(e)?;
        }
        Ok(self.context.get_type("Object").unwrap())
    }
    
    fn visit_vector_generator(&mut self, expr: &Spanned<Expr>, var: &str, iterable: &Spanned<Expr>, _pos: crate::utils::Position) -> Result<Rc<RefCell<Type>>, Vec<SemanticError>> {
        let _ = self.infer_type(iterable)?;
        
        self.enter_scope();
        self.define(var.to_string(), self.context.get_type("Object").unwrap());
        let result = self.infer_type(expr);
        self.exit_scope();
        
        result?;
        Ok(self.context.get_type("Object").unwrap())
    }
    
    fn visit_indexing(&mut self, obj: &Spanned<Expr>, index: &Spanned<Expr>, _pos: crate::utils::Position) -> Result<Rc<RefCell<Type>>, Vec<SemanticError>> {
        let _ = self.infer_type(obj)?;
        let t_idx = self.infer_type(index)?;
        
        if t_idx.borrow().name != "Number" {
            return Err(vec![SemanticError::TypeMismatch{ 
                expected: "Number".to_string(), 
                found: t_idx.borrow().name.clone(), 
                pos: index.pos 
            }]);
        }
        
        Ok(self.context.get_type("Object").unwrap())
    }
    
    fn visit_match(&mut self, expr: &Spanned<Expr>, cases: &[MatchCase], default: &Option<Box<Spanned<Expr>>>, _pos: crate::utils::Position) -> Result<Rc<RefCell<Type>>, Vec<SemanticError>> {
        let _ = self.infer_type(expr)?;
        
        for case in cases {
            let _ = self.infer_type(&case.expr)?;
        }
        
        if let Some(d) = default {
            let _ = self.infer_type(d)?;
        }
        
        Ok(self.context.get_type("Object").unwrap())
    }
    
    fn visit_is(&mut self, expr: &Spanned<Expr>, _ty: &str, _pos: crate::utils::Position) -> Result<Rc<RefCell<Type>>, Vec<SemanticError>> {
        let _ = self.infer_type(expr)?;
        Ok(self.context.get_type("Boolean").unwrap())
    }
    
    fn visit_as(&mut self, expr: &Spanned<Expr>, type_name: &str, _pos: crate::utils::Position) -> Result<Rc<RefCell<Type>>, Vec<SemanticError>> {
        let _ = self.infer_type(expr)?;
        Ok(self.context.get_type(type_name).unwrap_or_else(|_| self.context.get_type("Object").unwrap()))
    }
    
    fn visit_pi(&mut self, _pos: crate::utils::Position) -> Result<Rc<RefCell<Type>>, Vec<SemanticError>> {
        Ok(self.context.get_type("Number").unwrap())
    }
    
    fn visit_e(&mut self, _pos: crate::utils::Position) -> Result<Rc<RefCell<Type>>, Vec<SemanticError>> {
        Ok(self.context.get_type("Number").unwrap())
    }
    
    fn visit_rand(&mut self, _pos: crate::utils::Position) -> Result<Rc<RefCell<Type>>, Vec<SemanticError>> {
        Ok(self.context.get_type("Number").unwrap())
    }
    
    fn visit_sin(&mut self, e: &Spanned<Expr>, _pos: crate::utils::Position) -> Result<Rc<RefCell<Type>>, Vec<SemanticError>> {
        let t = self.infer_type(e)?;
        if t.borrow().name != "Number" {
            return Err(vec![SemanticError::TypeMismatch{
                expected: "Number".into(), 
                found: t.borrow().name.clone(), 
                pos: e.pos
            }]);
        }
        Ok(self.context.get_type("Number").unwrap())
    }
    
    fn visit_cos(&mut self, e: &Spanned<Expr>, _pos: crate::utils::Position) -> Result<Rc<RefCell<Type>>, Vec<SemanticError>> {
        let t = self.infer_type(e)?;
        if t.borrow().name != "Number" {
            return Err(vec![SemanticError::TypeMismatch{
                expected: "Number".into(), 
                found: t.borrow().name.clone(), 
                pos: e.pos
            }]);
        }
        Ok(self.context.get_type("Number").unwrap())
    }
    
    fn visit_sqrt(&mut self, e: &Spanned<Expr>, _pos: crate::utils::Position) -> Result<Rc<RefCell<Type>>, Vec<SemanticError>> {
        let t = self.infer_type(e)?;
        if t.borrow().name != "Number" {
            return Err(vec![SemanticError::TypeMismatch{
                expected: "Number".into(), 
                found: t.borrow().name.clone(), 
                pos: e.pos
            }]);
        }
        Ok(self.context.get_type("Number").unwrap())
    }
    
    fn visit_exp(&mut self, e: &Spanned<Expr>, _pos: crate::utils::Position) -> Result<Rc<RefCell<Type>>, Vec<SemanticError>> {
        let t = self.infer_type(e)?;
        if t.borrow().name != "Number" {
            return Err(vec![SemanticError::TypeMismatch{
                expected: "Number".into(), 
                found: t.borrow().name.clone(), 
                pos: e.pos
            }]);
        }
        Ok(self.context.get_type("Number").unwrap())
    }
    
    fn visit_log(&mut self, base: &Spanned<Expr>, val: &Spanned<Expr>, _pos: crate::utils::Position) -> Result<Rc<RefCell<Type>>, Vec<SemanticError>> {
        let t1 = self.infer_type(base)?;
        let t2 = self.infer_type(val)?;
        
        if t1.borrow().name != "Number" {
            return Err(vec![SemanticError::TypeMismatch{
                expected: "Number".into(), 
                found: t1.borrow().name.clone(), 
                pos: base.pos
            }]);
        }
        if t2.borrow().name != "Number" {
            return Err(vec![SemanticError::TypeMismatch{
                expected: "Number".into(), 
                found: t2.borrow().name.clone(), 
                pos: val.pos
            }]);
        }
        
        Ok(self.context.get_type("Number").unwrap())
    }
    
    fn visit_base_call(&mut self, args: &[Spanned<Expr>], _pos: crate::utils::Position) -> Result<Rc<RefCell<Type>>, Vec<SemanticError>> {
        for a in args {
            let _ = self.infer_type(a)?;
        }
        Ok(self.context.get_type("Object").unwrap())
    }
}
