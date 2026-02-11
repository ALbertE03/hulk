use crate::ast::nodes::{Expr, ExprVisitor, MatchCase, TypeAnnotation, Param};
use crate::utils::{Position, Spanned};
use std::collections::HashMap;
use super::context::MacroExpansionContext;
use super::utils::gensym;

/// Visitor para aplicar sustituciones de variables y expresiones
pub struct SubstitutionVisitor<'a> {
    substitutions: &'a HashMap<String, String>,
    expr_substitutions: &'a HashMap<String, Spanned<Expr>>,
}

impl<'a> SubstitutionVisitor<'a> {
    pub fn new(
        substitutions: &'a HashMap<String, String>,
        expr_substitutions: &'a HashMap<String, Spanned<Expr>>,
    ) -> Self {
        SubstitutionVisitor {
            substitutions,
            expr_substitutions,
        }
    }
}

impl<'a> ExprVisitor for SubstitutionVisitor<'a> {
    fn visit_identifier(&mut self, name: String, _pos: Position) -> Expr {
        // Primero buscar en sustituciones de expresiones (parámetros normales)
        if let Some(expr_sub) = self.expr_substitutions.get(&name) {
            return expr_sub.node.clone();
        }
        
        // Luego en sustituciones de nombres (parámetros simbólicos/placeholder)
        if let Some(substitution) = self.substitutions.get(&name) {
            Expr::Identifier(substitution.clone())
        } else {
            Expr::Identifier(name)
        }
    }
    
    fn visit_assignment(&mut self, target: String, value: Spanned<Expr>, _pos: Position) -> Expr {
        let new_target = if let Some(sub) = self.substitutions.get(&target) {
            sub.clone()
        } else {
            target
        };
        Expr::Assignment {
            target: new_target,
            value: Box::new(self.visit_expr(value)),
        }
    }
    
    fn visit_let(&mut self, bindings: Vec<(String, Option<TypeAnnotation>, Spanned<Expr>)>, 
                 body: Spanned<Expr>, _pos: Position) -> Expr {
        let new_bindings = bindings
            .into_iter()
            .map(|(name, ty, init)| {
                let new_name = if let Some(sub) = self.substitutions.get(&name) {
                    sub.clone()
                } else {
                    name
                };
                (new_name, ty, self.visit_expr(init))
            })
            .collect();
        Expr::Let {
            bindings: new_bindings,
            body: Box::new(self.visit_expr(body)),
        }
    }
}

/// Visitor para sanitizar variables (renombrar para evitar captura)
pub struct SanitizationVisitor<'a> {
    context: &'a MacroExpansionContext,
    scope: HashMap<String, String>,
}

impl<'a> SanitizationVisitor<'a> {
    pub fn new(context: &'a MacroExpansionContext) -> Self {
        SanitizationVisitor {
            context,
            scope: HashMap::new(),
        }
    }
    
    pub fn with_scope(context: &'a MacroExpansionContext, scope: HashMap<String, String>) -> Self {
        SanitizationVisitor {
            context,
            scope,
        }
    }
}

impl<'a> ExprVisitor for SanitizationVisitor<'a> {
    fn visit_identifier(&mut self, name: String, _pos: Position) -> Expr {
        if let Some(new_name) = self.scope.get(&name) {
            Expr::Identifier(new_name.clone())
        } else {
            Expr::Identifier(name)
        }
    }
    
    fn visit_let(&mut self, bindings: Vec<(String, Option<TypeAnnotation>, Spanned<Expr>)>, 
                 body: Spanned<Expr>, _pos: Position) -> Expr {
        let mut inner_scope = self.scope.clone();
        let mut new_bindings = Vec::new();

        for (name, ty, init) in bindings {
            let new_init = self.visit_expr(init);
            
            let new_name = if let Some(placeholder_target) = self.context.substitutions.get(&name) {
                placeholder_target.clone()
            } else {
                gensym(&name)
            };
            
            inner_scope.insert(name, new_name.clone());
            new_bindings.push((new_name, ty, new_init));
        }

        let mut inner_visitor = SanitizationVisitor::with_scope(self.context, inner_scope);
        let new_body = inner_visitor.visit_expr(body);
        
        Expr::Let {
            bindings: new_bindings,
            body: Box::new(new_body),
        }
    }
    
    fn visit_for(&mut self, var: String, iterable: Spanned<Expr>, 
                 body: Spanned<Expr>, _pos: Position) -> Expr {
        let new_iterable = self.visit_expr(iterable);
        
        let mut inner_scope = self.scope.clone();
        let new_var = gensym(&var);
        inner_scope.insert(var, new_var.clone());
        
        let mut inner_visitor = SanitizationVisitor::with_scope(self.context, inner_scope);
        let new_body = inner_visitor.visit_expr(body);
        
        Expr::For {
            var: new_var,
            iterable: Box::new(new_iterable),
            body: Box::new(new_body),
        }
    }
    
    fn visit_vector_generator(&mut self, expr: Spanned<Expr>, var: String, 
                             iterable: Spanned<Expr>, _pos: Position) -> Expr {
        let new_iterable = self.visit_expr(iterable);
        
        let mut inner_scope = self.scope.clone();
        let new_var = gensym(&var);
        inner_scope.insert(var, new_var.clone());
        
        let mut inner_visitor = SanitizationVisitor::with_scope(self.context, inner_scope);
        let new_expr = inner_visitor.visit_expr(expr);
        
        Expr::VectorGenerator {
            expr: Box::new(new_expr),
            var: new_var,
            iterable: Box::new(new_iterable),
        }
    }
    
    fn visit_lambda(&mut self, params: Vec<Param>, return_type: Option<TypeAnnotation>, 
                   body: Spanned<Expr>, _pos: Position) -> Expr {
        let mut inner_scope = self.scope.clone();
        let new_params = params.iter().map(|p| {
            let new_name = gensym(&p.name);
            inner_scope.insert(p.name.clone(), new_name.clone());
            Param {
                name: new_name,
                type_annotation: p.type_annotation.clone(),
            }
        }).collect();
        
        let mut inner_visitor = SanitizationVisitor::with_scope(self.context, inner_scope);
        let new_body = inner_visitor.visit_expr(body);
        
        Expr::Lambda {
            params: new_params,
            return_type,
            body: Box::new(new_body),
        }
    }
    
    fn visit_match(&mut self, expr: Spanned<Expr>, cases: Vec<MatchCase>, 
                  default: Option<Box<Spanned<Expr>>>, _pos: Position) -> Expr {
        let new_expr = self.visit_expr(expr);
        
        let new_cases = cases.into_iter().map(|case| {
            let mut inner_scope = self.scope.clone();
            let new_pattern = self.context.sanitize_pattern(&case.pattern, &mut inner_scope);
            let mut inner_visitor = SanitizationVisitor::with_scope(self.context, inner_scope);
            let new_case_expr = inner_visitor.visit_expr(case.expr);
            
            MatchCase {
                pattern: new_pattern,
                expr: new_case_expr,
            }
        }).collect();
        
        let new_default = default.map(|d| Box::new(self.visit_expr(*d)));
        
        Expr::Match {
            expr: Box::new(new_expr),
            cases: new_cases,
            default: new_default,
        }
    }
    
    fn visit_assignment(&mut self, target: String, value: Spanned<Expr>, _pos: Position) -> Expr {
        let new_target = if let Some(new_name) = self.scope.get(&target) {
            new_name.clone()
        } else {
            target
        };
        Expr::Assignment {
            target: new_target,
            value: Box::new(self.visit_expr(value)),
        }
    }
}

/// Visitor para expandir macros
pub struct MacroExpansionVisitor<'a> {
    pub context: &'a mut MacroExpansionContext,
}

impl<'a> MacroExpansionVisitor<'a> {
    pub fn new(context: &'a mut MacroExpansionContext) -> Self {
        MacroExpansionVisitor { context }
    }
}

impl<'a> ExprVisitor for MacroExpansionVisitor<'a> {
    fn visit_identifier(&mut self, name: String, _pos: Position) -> Expr {
        // Aplicar sustituciones si existen
        if let Some(expr_sub) = self.context.expr_substitutions.get(&name) {
            // Expandir recursivamente la sustitución
            return self.visit_expr(expr_sub.clone()).node;
        } else if let Some(substitution) = self.context.substitutions.get(&name) {
            Expr::Identifier(substitution.clone())
        } else {
            Expr::Identifier(name)
        }
    }
    
    fn visit_call(&mut self, func: String, args: Vec<Spanned<Expr>>, pos: Position) -> Expr {
        // Expandir argumentos primero
        let expanded_args: Vec<Spanned<Expr>> = args.into_iter()
            .map(|a| self.visit_expr(a))
            .collect();
        
        // Verificar si es una llamada a macro
        if let Some(macro_decl) = self.context.macros.get(&func).cloned() {
            let result = self.context.expand_macro_call(&macro_decl, expanded_args, pos);
            // Expandir recursivamente el resultado
            return self.visit_expr(Spanned::new(result, pos)).node;
        }
        
        // Llamada normal
        Expr::Call {
            func,
            args: expanded_args,
        }
    }
    
    fn visit_match(&mut self, expr: Spanned<Expr>, cases: Vec<MatchCase>, 
                  default: Option<Box<Spanned<Expr>>>, _pos: Position) -> Expr {
        let expanded_expr = self.visit_expr(expr);
        
        // Intentar Pattern Matching en tiempo de compilación
        for case in &cases {
            if let Some(bindings) = self.context.pattern_match(&case.pattern, &expanded_expr) {
                // Guardar contexto
                let old_substitutions = self.context.substitutions.clone();
                let old_expr_substitutions = self.context.expr_substitutions.clone();

                // Inyectar bindings
                self.context.expr_substitutions.extend(bindings);
                
                // Expandir el cuerpo del caso
                let result = self.visit_expr(case.expr.clone());

                // Restaurar contexto
                self.context.substitutions = old_substitutions;
                self.context.expr_substitutions = old_expr_substitutions;

                return result.node;
            }
        }
        
        // Si hay default y no hubo match, expandirlo
        if let Some(def) = default {
            return self.visit_expr(*def).node;
        }

        // Fallback: generar nodo Match para runtime
        let expanded_cases = cases.into_iter()
            .map(|case| MatchCase {
                pattern: case.pattern,
                expr: self.visit_expr(case.expr),
            })
            .collect();

        Expr::Match {
            expr: Box::new(expanded_expr),
            cases: expanded_cases,
            default: None,
        }
    }
}
