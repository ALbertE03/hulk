
#[cfg(test)]
mod tests;

use crate::ast::nodes::{Declaration, Expr, MacroDecl, MacroParam, MatchCase, Pattern, Program};
use crate::utils::{Position, Spanned};
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Contador global para generar nombres únicos
#[allow(dead_code)]
static GENSYM_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Genera un nombre único para variables sanitizadas
/// Si el prefijo es válido (no empieza con _), genera nombre sin _ al inicio
/// Si es inválido, retorna el prefijo original sin modificar
#[allow(dead_code)]
fn gensym(prefix: &str) -> String {
    // Si el prefijo empieza con _, es inválido - retornar sin generar
    if prefix.starts_with('_') {
        return prefix.to_string();
    }
    
    // Para prefijos válidos, generar con $$ (nunca con _ al inicio)
    let count = GENSYM_COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("{}$${}", prefix, count)
}

/// Contexto de expansión de macro
pub struct MacroExpansionContext {
    /// Macros disponibles (nombre -> declaración)
    macros: HashMap<String, MacroDecl>,
    /// Sustituciones activas (nombre original -> nombre generado)
    substitutions: HashMap<String, String>,
    /// Expression substitutions for Normal macro params (name -> expr)
    expr_substitutions: HashMap<String, Spanned<Expr>>,
}

impl MacroExpansionContext {
    pub fn new() -> Self {
        MacroExpansionContext {
            macros: HashMap::new(),
            substitutions: HashMap::new(),
            expr_substitutions: HashMap::new(),
        }
    }

    /// Registra una macro en el contexto
    pub fn register_macro(&mut self, decl: MacroDecl) {
        self.macros.insert(decl.name.clone(), decl);
    }

    /// Expande todas las macros en un programa
    pub fn expand_program(&mut self, program: Program) -> Program {
        // Fase 1: Registrar todas las macros
        let mut non_macro_decls = Vec::new();
        for decl in program.declarations {
            match decl {
                Declaration::Macro(macro_decl) => {
                    self.register_macro(macro_decl);
                }
                other => non_macro_decls.push(other),
            }
        }

        // Fase 2: Expandir expresiones en el cuerpo principal
        let expanded_expr = self.expand_expr(program.expr);

        // Fase 3: Expandir expresiones dentro de las funciones
        let fully_expanded_decls = non_macro_decls.into_iter().map(|decl| {
            if let Declaration::Function(mut func_decl) = decl {
                func_decl.body = self.expand_expr(func_decl.body);
                Declaration::Function(func_decl)
            } else {
                decl
            }
        }).collect();

        Program {
            declarations: fully_expanded_decls,
            expr: expanded_expr,
        }
    }

    /// Expande macros en una expresión
    fn expand_expr(&mut self, expr: Spanned<Expr>) -> Spanned<Expr> {
        let pos = expr.pos;
        let expanded_node = match expr.node {
            // Identifier: aplicar sustituciones si existen
            Expr::Identifier(name) => {
                if let Some(expr_sub) = self.expr_substitutions.get(&name) {
                    // Hemos encontrado una sustitución (parámetro normal o match binding)
                    // Expandimos la sustitución recursivamente por si contiene más macros
                    return self.expand_expr(expr_sub.clone());
                } else if let Some(substitution) = self.substitutions.get(&name) {
                    Expr::Identifier(substitution.clone())
                } else {
                    Expr::Identifier(name)
                }
            }

            // Llamadas a macros se convierten en expansión
            Expr::Call { func, args } => {
                if let Some(macro_decl) = self.macros.get(&func).cloned() {
                    // Expand arguments first, then expand the macro call
                    let expanded_args: Vec<Spanned<Expr>> = args.into_iter().map(|a| self.expand_expr(a)).collect();
                    let result = self.expand_macro_call(&macro_decl, expanded_args, pos);
                    // Recursively expand the result (for nested macro calls in the body)
                    return self.expand_expr(Spanned::new(result, pos));
                } else {
                    // Llamada normal, expandir argumentos recursivamente
                    Expr::Call {
                        func,
                        args: args.into_iter().map(|a| self.expand_expr(a)).collect(),
                    }
                }
            }

            // Let: expandir bindings y body
            Expr::Let { bindings, body } => {
                let expanded_bindings = bindings
                    .into_iter()
                    .map(|(name, ty, init)| (name, ty, self.expand_expr(init)))
                    .collect();
                Expr::Let {
                    bindings: expanded_bindings,
                    body: Box::new(self.expand_expr(*body)),
                }
            }

            // If: expandir condición, then y else
            Expr::If {
                cond,
                then_expr,
                else_expr,
            } => Expr::If {
                cond: Box::new(self.expand_expr(*cond)),
                then_expr: Box::new(self.expand_expr(*then_expr)),
                else_expr: Box::new(self.expand_expr(*else_expr)),
            },

            // While: expandir condición y body
            Expr::While { cond, body } => Expr::While {
                cond: Box::new(self.expand_expr(*cond)),
                body: Box::new(self.expand_expr(*body)),
            },

            // For: expandir iterable y body
            Expr::For { var, iterable, body } => Expr::For {
                var,
                iterable: Box::new(self.expand_expr(*iterable)),
                body: Box::new(self.expand_expr(*body)),
            },

            // Block: expandir cada expresión
            Expr::Block(exprs) => {
                Expr::Block(exprs.into_iter().map(|e| self.expand_expr(e)).collect())
            }

            // Binary: expandir operandos
            Expr::Binary(left, op, right) => Expr::Binary(
                Box::new(self.expand_expr(*left)),
                op,
                Box::new(self.expand_expr(*right)),
            ),

            // Unary: expandir operando
            Expr::Unary(op, operand) => Expr::Unary(op, Box::new(self.expand_expr(*operand))),

            // Match: expandir expr, cases y default
            Expr::Match {
                expr,
                cases,
                default,
            } => {
                let expanded_expr_node = self.expand_expr(*expr);
                
                // Intentar Pattern Matching en tiempo de compilación
                // Esto es crucial para macros que inspeccionan la estructura AST
                for case in &cases {
                    if let Some(bindings) = self.pattern_match(&case.pattern, &expanded_expr_node) {
                        // ¡Coincidencia encontrada!
                        let old_substitutions = self.substitutions.clone();
                        let old_expr_substitutions = self.expr_substitutions.clone();

                        // Inyectar bindings en el contexto de sustitución
                        self.expr_substitutions.extend(bindings);
                        
                        // Expandir el cuerpo del caso con los nuevos bindings
                        let result = self.expand_expr(case.expr.clone());

                        // Restaurar contexto
                        self.substitutions = old_substitutions;
                        self.expr_substitutions = old_expr_substitutions;

                        return Spanned::new(result.node, pos);
                    }
                }
                
                // Si ninguna coincidencia estructural (AST) funciona, conservamos el nodo Match
                // para que se resuelva en tiempo de ejecución (si es un match de tipos válido).
                // Pero si hay un bloque default, y estamos en el contexto de macro expansión 
                // donde el match falló, ¿quizás debamos expandir el default?
                // La semántica de HULK macros sugiere que el match se "resuelve".
                // Si no hay match estructural, y hay default, tomamos el default.
                if let Some(def) = default {
                     // Nota: Esto asume que el match era estructural y falló.
                     // Si era un match de tipos runtime que casualmente no coincidió estructuralmente con AST...
                     // Es ambigüo. Pero dado el use case de "simplify", el default actúa como fallback.
                     return self.expand_expr(*def);
                }

                // Fallback: generar nodo Match para runtime
                let expanded_cases = cases
                    .into_iter()
                    .map(|case| MatchCase {
                        pattern: case.pattern, // Patterns no se expanden
                        expr: self.expand_expr(case.expr),
                    })
                    .collect();

                Expr::Match {
                    expr: Box::new(expanded_expr_node),
                    cases: expanded_cases,
                    default: None, // El default ya se habría tomado arriba si existiera
                }
            }

            // MethodCall: expandir objeto y argumentos
            Expr::MethodCall { obj, method, args } => Expr::MethodCall {
                obj: Box::new(self.expand_expr(*obj)),
                method,
                args: args.into_iter().map(|a| self.expand_expr(a)).collect(),
            },

            // AttributeAccess: expandir objeto
            Expr::AttributeAccess { obj, attribute } => Expr::AttributeAccess {
                obj: Box::new(self.expand_expr(*obj)),
                attribute,
            },

            // Instantiation: expandir argumentos
            Expr::Instantiation { ty, args } => Expr::Instantiation {
                ty,
                args: args.into_iter().map(|a| self.expand_expr(a)).collect(),
            },

            // Assignment: expandir value
            Expr::Assignment { target, value } => Expr::Assignment {
                target,
                value: Box::new(self.expand_expr(*value)),
            },

            // AttributeAssignment: expandir obj y value
            Expr::AttributeAssignment { obj, attribute, value } => Expr::AttributeAssignment {
                obj: Box::new(self.expand_expr(*obj)),
                attribute,
                value: Box::new(self.expand_expr(*value)),
            },

            // Lambda: expandir body
            Expr::Lambda {
                params,
                return_type,
                body,
            } => Expr::Lambda {
                params,
                return_type,
                body: Box::new(self.expand_expr(*body)),
            },

            // VectorLiteral: expandir elementos
            Expr::VectorLiteral(elements) => {
                Expr::VectorLiteral(elements.into_iter().map(|e| self.expand_expr(e)).collect())
            }

            // VectorGenerator: expandir expr e iterable
            Expr::VectorGenerator {
                expr,
                var,
                iterable,
            } => Expr::VectorGenerator {
                expr: Box::new(self.expand_expr(*expr)),
                var,
                iterable: Box::new(self.expand_expr(*iterable)),
            },

            // Indexing: expandir objeto e índice
            Expr::Indexing { obj, index } => Expr::Indexing {
                obj: Box::new(self.expand_expr(*obj)),
                index: Box::new(self.expand_expr(*index)),
            },

            // Is/As: expandir expresión
            Expr::Is(e, ty) => Expr::Is(Box::new(self.expand_expr(*e)), ty),
            Expr::As(e, ty) => Expr::As(Box::new(self.expand_expr(*e)), ty),

            // Funciones matemáticas: expandir argumentos
            Expr::Sqrt(e) => Expr::Sqrt(Box::new(self.expand_expr(*e))),
            Expr::Sin(e) => Expr::Sin(Box::new(self.expand_expr(*e))),
            Expr::Cos(e) => Expr::Cos(Box::new(self.expand_expr(*e))),
            Expr::Exp(e) => Expr::Exp(Box::new(self.expand_expr(*e))),
            Expr::Log(base, x) => Expr::Log(
                Box::new(self.expand_expr(*base)),
                Box::new(self.expand_expr(*x)),
            ),

            // Primitivos y terminales: no expandir
            other => other,
        };

        Spanned::new(expanded_node, pos)
    }

    /// Expande una llamada a macro
    fn expand_macro_call(
        &mut self,
        macro_decl: &MacroDecl,
        args: Vec<Spanned<Expr>>,
        _call_pos: Position,
    ) -> Expr {
        // Crear nuevo scope para sustituciones
        let old_substitutions = self.substitutions.clone();
        let old_expr_substitutions = self.expr_substitutions.clone();

        // Procesar parámetros
        let mut _body_arg: Option<Spanned<Expr>> = None;

        for (i, param) in macro_decl.params.iter().enumerate() {
            match param {
                MacroParam::Normal { name, .. } => {
                    // Argumento normal: sustituir nombre por expresión
                    if let Some(arg) = args.get(i) {
                        self.expr_substitutions.insert(name.clone(), arg.clone());
                    }
                }

                MacroParam::Symbolic { name, .. } => {
                    // Argumento simbólico: extraer nombre de variable
                    if let Some(arg) = args.get(i) {
                        if let Expr::Identifier(var_name) = &arg.node {
                            self.substitutions.insert(name.clone(), var_name.clone());
                        }
                    }
                }

                MacroParam::Placeholder { name, .. } => {
                    // Placeholder: sustituir por nombre del argumento
                    if let Some(arg) = args.get(i) {
                        if let Expr::Identifier(var_name) = &arg.node {
                            self.substitutions.insert(name.clone(), var_name.clone());
                        }
                    }
                }

                MacroParam::Body { name, .. } => {
                    // Body argument: capturar expresión completa
                    // Típicamente el último argumento
                    if let Some(arg) = args.last() {
                        self.expr_substitutions.insert(name.clone(), arg.clone());
                    }
                }
            }
        }

        // Clonar y sanitizar body de la macro
        let mut expanded_body = self.sanitize_expr(&macro_decl.body);

        // Aplicar sustituciones
        expanded_body = self.apply_substitutions(expanded_body);

        // Restaurar sustituciones anteriores
        self.substitutions = old_substitutions;
        self.expr_substitutions = old_expr_substitutions;

        expanded_body.node
    }

    /// Sanitiza variables en el body de una macro (renombra para evitar captura)
    pub fn sanitize_expr(&self, expr: &Spanned<Expr>) -> Spanned<Expr> {
        let mut scope = HashMap::new();
        self.sanitize_expr_with_scope(expr, &mut scope)
    }

    fn sanitize_expr_with_scope(&self, expr: &Spanned<Expr>, scope: &mut HashMap<String, String>) -> Spanned<Expr> {
        let pos = expr.pos;
        let node = match &expr.node {
            Expr::Let { bindings, body } => {
                let mut inner_scope = scope.clone();
                let mut new_bindings = Vec::new();

                for (name, ty, init) in bindings {
                    // Init se sanitiza con el scope ACTUAL (antes de añadir la nueva variable)
                    let new_init = self.sanitize_expr_with_scope(init, &mut inner_scope);

                    // Generar nuevo nombre para la variable
                    let new_name = gensym(name);
                    inner_scope.insert(name.clone(), new_name.clone());

                    new_bindings.push((new_name, ty.clone(), new_init));
                }

                let new_body = self.sanitize_expr_with_scope(body, &mut inner_scope);
                
                Expr::Let {
                    bindings: new_bindings,
                    body: Box::new(new_body),
                }
            },
            
            Expr::For { var, iterable, body } => {
                let new_iterable = self.sanitize_expr_with_scope(iterable, scope);
                
                let mut inner_scope = scope.clone();
                let new_var = gensym(var);
                inner_scope.insert(var.clone(), new_var.clone());
                
                let new_body = self.sanitize_expr_with_scope(body, &mut inner_scope);
                
                Expr::For {
                    var: new_var,
                    iterable: Box::new(new_iterable),
                    body: Box::new(new_body),
                }
            },
            
            Expr::VectorGenerator { expr, var, iterable } => {
                let new_iterable = self.sanitize_expr_with_scope(iterable, scope);
                
                let mut inner_scope = scope.clone();
                let new_var = gensym(var);
                inner_scope.insert(var.clone(), new_var.clone());
                
                let new_expr = self.sanitize_expr_with_scope(expr, &mut inner_scope);
                
                Expr::VectorGenerator {
                    expr: Box::new(new_expr),
                    var: new_var,
                    iterable: Box::new(new_iterable),
                }
            },
            
            Expr::Lambda { params, return_type, body } => {
                let mut inner_scope = scope.clone();
                let new_params = params.iter().map(|p| {
                     let new_name = gensym(&p.name);
                     inner_scope.insert(p.name.clone(), new_name.clone());
                     crate::ast::nodes::Param {
                         name: new_name,
                         type_annotation: p.type_annotation.clone(),
                     }
                }).collect();
                
                let new_body = self.sanitize_expr_with_scope(body, &mut inner_scope);
                
                Expr::Lambda {
                    params: new_params,
                    return_type: return_type.clone(),
                    body: Box::new(new_body),
                }
            },
            
            Expr::Match { expr, cases, default } => {
                let new_expr = self.sanitize_expr_with_scope(expr, scope);
                
                let new_cases = cases.iter().map(|case| {
                    let mut inner_scope = scope.clone();
                    let new_pattern = self.sanitize_pattern(&case.pattern, &mut inner_scope);
                    let new_case_expr = self.sanitize_expr_with_scope(&case.expr, &mut inner_scope);
                    
                    MatchCase {
                        pattern: new_pattern,
                        expr: new_case_expr,
                    }
                }).collect();
                
                let new_default = default.as_ref().map(|d| Box::new(self.sanitize_expr_with_scope(d, scope)));
                
                Expr::Match {
                    expr: Box::new(new_expr),
                    cases: new_cases,
                    default: new_default,
                }
            },

            Expr::Identifier(name) => {
                if let Some(new_name) = scope.get(name) {
                    Expr::Identifier(new_name.clone())
                } else {
                    Expr::Identifier(name.clone())
                }
            },
            
            // Recursive delegation for other nodes (standard traversal)
            Expr::Block(exprs) => Expr::Block(exprs.iter().map(|e| self.sanitize_expr_with_scope(e, scope)).collect()),
            Expr::If { cond, then_expr, else_expr } => Expr::If {
                cond: Box::new(self.sanitize_expr_with_scope(cond, scope)),
                then_expr: Box::new(self.sanitize_expr_with_scope(then_expr, scope)),
                else_expr: Box::new(self.sanitize_expr_with_scope(else_expr, scope)),
            },
            Expr::While { cond, body } => Expr::While {
                cond: Box::new(self.sanitize_expr_with_scope(cond, scope)),
                body: Box::new(self.sanitize_expr_with_scope(body, scope)),
            },
            Expr::Binary(left, op, right) => Expr::Binary(
                Box::new(self.sanitize_expr_with_scope(left, scope)),
                op.clone(),
                Box::new(self.sanitize_expr_with_scope(right, scope)),
            ),
            Expr::Unary(op, operand) => Expr::Unary(
                op.clone(),
                Box::new(self.sanitize_expr_with_scope(operand, scope)),
            ),
            Expr::Call { func, args } => Expr::Call {
                func: func.clone(),
                args: args.iter().map(|a| self.sanitize_expr_with_scope(a, scope)).collect(),
            },
            Expr::MethodCall { obj, method, args } => Expr::MethodCall {
                obj: Box::new(self.sanitize_expr_with_scope(obj, scope)),
                method: method.clone(),
                args: args.iter().map(|a| self.sanitize_expr_with_scope(a, scope)).collect(),
            },
             Expr::Assignment { target, value } => {
                let new_target = if let Some(new_name) = scope.get(target) {
                    new_name.clone()
                } else {
                    target.clone()
                };
                Expr::Assignment {
                    target: new_target,
                    value: Box::new(self.sanitize_expr_with_scope(value, scope)),
                }
            },
            Expr::AttributeAssignment { obj, attribute, value } => Expr::AttributeAssignment {
                obj: Box::new(self.sanitize_expr_with_scope(obj, scope)),
                attribute: attribute.clone(),
                value: Box::new(self.sanitize_expr_with_scope(value, scope)),
            },
            Expr::AttributeAccess { obj, attribute } => Expr::AttributeAccess {
                obj: Box::new(self.sanitize_expr_with_scope(obj, scope)),
                attribute: attribute.clone(),
            },
             Expr::Instantiation { ty, args } => Expr::Instantiation {
                ty: ty.clone(),
                args: args.iter().map(|a| self.sanitize_expr_with_scope(a, scope)).collect(),
            },
            Expr::VectorLiteral(elements) => Expr::VectorLiteral(
                elements.iter().map(|e| self.sanitize_expr_with_scope(e, scope)).collect()
            ),
            Expr::Indexing { obj, index } => Expr::Indexing {
                obj: Box::new(self.sanitize_expr_with_scope(obj, scope)),
                index: Box::new(self.sanitize_expr_with_scope(index, scope)),
            },
            Expr::Is(e, ty) => Expr::Is(Box::new(self.sanitize_expr_with_scope(e, scope)), ty.clone()),
            Expr::As(e, ty) => Expr::As(Box::new(self.sanitize_expr_with_scope(e, scope)), ty.clone()),
            Expr::Sqrt(e) => Expr::Sqrt(Box::new(self.sanitize_expr_with_scope(e, scope))),
            Expr::Sin(e) => Expr::Sin(Box::new(self.sanitize_expr_with_scope(e, scope))),
            Expr::Cos(e) => Expr::Cos(Box::new(self.sanitize_expr_with_scope(e, scope))),
            Expr::Exp(e) => Expr::Exp(Box::new(self.sanitize_expr_with_scope(e, scope))),
            Expr::Log(base, x) => Expr::Log(
                Box::new(self.sanitize_expr_with_scope(base, scope)),
                Box::new(self.sanitize_expr_with_scope(x, scope)),
            ),
            
            // Terminales y otros
            other => other.clone(),
        };
        Spanned::new(node, pos)
    }

    fn sanitize_pattern(&self, pattern: &Pattern, scope: &mut HashMap<String, String>) -> Pattern {
        match pattern {
            Pattern::Variable { name, type_annotation } => {
                let new_name = gensym(name);
                scope.insert(name.clone(), new_name.clone());
                Pattern::Variable { name: new_name, type_annotation: type_annotation.clone() }
            },
            Pattern::Binary { left, op, right } => Pattern::Binary {
                left: Box::new(self.sanitize_pattern(left, scope)),
                op: op.clone(),
                right: Box::new(self.sanitize_pattern(right, scope)),
            },
            Pattern::Unary { op, operand } => Pattern::Unary {
                op: op.clone(),
                operand: Box::new(self.sanitize_pattern(operand, scope)),
            },
            Pattern::Call { func, args } => Pattern::Call {
                func: func.clone(),
                args: args.iter().map(|p| self.sanitize_pattern(p, scope)).collect(),
            },
            other => other.clone(),
        }
    }

    /// Aplica sustituciones a una expresión
    fn apply_substitutions(&self, expr: Spanned<Expr>) -> Spanned<Expr> {
        let pos = expr.pos;
        let node = match expr.node {
            Expr::Identifier(name) => {
                if let Some(expr_sub) = self.expr_substitutions.get(&name) {
                    // Normal param: replace with full expression
                    return expr_sub.clone();
                } else if let Some(substitution) = self.substitutions.get(&name) {
                    Expr::Identifier(substitution.clone())
                } else {
                    Expr::Identifier(name)
                }
            }
            
            // Asignación: sustituir tanto target como value
            Expr::Assignment { target, value } => {
                let new_target = if let Some(sub) = self.substitutions.get(&target) {
                    sub.clone()
                } else {
                    target
                };
                Expr::Assignment {
                    target: new_target,
                    value: Box::new(self.apply_substitutions(*value)),
                }
            }

            // Asignación a atributo: sustituir obj y value
            Expr::AttributeAssignment { obj, attribute, value } => {
                Expr::AttributeAssignment {
                    obj: Box::new(self.apply_substitutions(*obj)),
                    attribute,
                    value: Box::new(self.apply_substitutions(*value)),
                }
            }

            // Recursivamente aplicar a todas las subexpresiones
            Expr::Binary(left, op, right) => Expr::Binary(
                Box::new(self.apply_substitutions(*left)),
                op,
                Box::new(self.apply_substitutions(*right)),
            ),
            Expr::Unary(op, operand) => Expr::Unary(op, Box::new(self.apply_substitutions(*operand))),
            Expr::Call { func, args } => Expr::Call {
                func, // No sustituimos nombres de funciones por ahora
                args: args.into_iter().map(|a| self.apply_substitutions(a)).collect(),
            },
            Expr::Let { bindings, body } => {
                let substituted_bindings = bindings
                    .into_iter()
                    .map(|(name, ty, init)| {
                        // Revisar si el nombre de la variable está en las sustituciones
                        // Esto es crucial para parámetros de tipo placeholder ($param)
                        let new_name = if let Some(sub) = self.substitutions.get(&name) {
                            sub.clone()
                        } else {
                            name
                        };
                        (new_name, ty, self.apply_substitutions(init))
                    })
                    .collect();
                Expr::Let {
                    bindings: substituted_bindings,
                    body: Box::new(self.apply_substitutions(*body)),
                }
            }
            Expr::Block(exprs) => {
                Expr::Block(exprs.into_iter().map(|e| self.apply_substitutions(e)).collect())
            },
            Expr::If { cond, then_expr, else_expr } => Expr::If {
                cond: Box::new(self.apply_substitutions(*cond)),
                then_expr: Box::new(self.apply_substitutions(*then_expr)),
                else_expr: Box::new(self.apply_substitutions(*else_expr)),
            },
            Expr::While { cond, body } => Expr::While {
                cond: Box::new(self.apply_substitutions(*cond)),
                body: Box::new(self.apply_substitutions(*body)),
            },
            Expr::For { var, iterable, body } => Expr::For {
                var, // Variable local del for, no se sustituye
                iterable: Box::new(self.apply_substitutions(*iterable)),
                body: Box::new(self.apply_substitutions(*body)),
            },
            
            // Otros casos recursivos
            Expr::MethodCall { obj, method, args } => Expr::MethodCall {
                obj: Box::new(self.apply_substitutions(*obj)),
                method,
                args: args.into_iter().map(|a| self.apply_substitutions(a)).collect(),
            },
            Expr::AttributeAccess { obj, attribute } => Expr::AttributeAccess {
                obj: Box::new(self.apply_substitutions(*obj)),
                attribute,
            },
            Expr::Instantiation { ty, args } => Expr::Instantiation {
                ty,
                args: args.into_iter().map(|a| self.apply_substitutions(a)).collect(),
            },
            Expr::VectorLiteral(elements) => Expr::VectorLiteral(
                elements.into_iter().map(|e| self.apply_substitutions(e)).collect()
            ),
            Expr::VectorGenerator { expr, var, iterable } => Expr::VectorGenerator {
                expr: Box::new(self.apply_substitutions(*expr)),
                var,
                iterable: Box::new(self.apply_substitutions(*iterable)),
            },
            Expr::Indexing { obj, index } => Expr::Indexing {
                obj: Box::new(self.apply_substitutions(*obj)),
                index: Box::new(self.apply_substitutions(*index)),
            },
            Expr::Is(expr, ty) => Expr::Is(Box::new(self.apply_substitutions(*expr)), ty),
            Expr::As(expr, ty) => Expr::As(Box::new(self.apply_substitutions(*expr)), ty),
            Expr::Lambda { params, return_type, body } => Expr::Lambda {
                params,
                return_type,
                body: Box::new(self.apply_substitutions(*body)),
            },

            // Match: sustituir en la expresión discriminante y en las expresiones de los casos
            Expr::Match { expr, cases, default } => Expr::Match {
                expr: Box::new(self.apply_substitutions(*expr)),
                cases: cases.into_iter().map(|case| MatchCase {
                    pattern: case.pattern,
                    expr: self.apply_substitutions(case.expr),
                }).collect(),
                default: default.map(|d| Box::new(self.apply_substitutions(*d))),
            },
            
            // Math functions
            Expr::Sqrt(e) => Expr::Sqrt(Box::new(self.apply_substitutions(*e))),
            Expr::Sin(e) => Expr::Sin(Box::new(self.apply_substitutions(*e))),
            Expr::Cos(e) => Expr::Cos(Box::new(self.apply_substitutions(*e))),
            Expr::Exp(e) => Expr::Exp(Box::new(self.apply_substitutions(*e))),
            Expr::Log(base, x) => Expr::Log(
                Box::new(self.apply_substitutions(*base)),
                Box::new(self.apply_substitutions(*x)),
            ),
            
            other => other,
        };

        Spanned::new(node, pos)
    }

    /// Realiza pattern matching entre un patrón y una expresión
    pub fn pattern_match(
        &self,
        pattern: &Pattern,
        expr: &Spanned<Expr>,
    ) -> Option<HashMap<String, Spanned<Expr>>> {
        let mut bindings = HashMap::new();

        match (pattern, &expr.node) {
            // Literal: debe coincidir exactamente
            (Pattern::Literal(pat_expr), expr_node) => {
                if pat_expr == expr_node {
                    Some(bindings)
                } else {
                    None
                }
            }

            // Variable: siempre coincide, captura valor
            (Pattern::Variable { name, .. }, _) => {
                bindings.insert(name.clone(), expr.clone());
                Some(bindings)
            }

            // Wildcard: siempre coincide, no captura
            (Pattern::Wildcard, _) => Some(bindings),

            // Binary: debe ser expresión binaria con mismo operador
            (
                Pattern::Binary { left, op, right },
                Expr::Binary(expr_left, expr_op, expr_right),
            ) => {
                if op == expr_op {
                    // Hacer match recursivo
                    let left_bindings = self.pattern_match(left, expr_left)?;
                    let right_bindings = self.pattern_match(right, expr_right)?;

                    // Combinar bindings
                    bindings.extend(left_bindings);
                    bindings.extend(right_bindings);
                    Some(bindings)
                } else {
                    None
                }
            }

            // Unary: debe ser expresión unaria con mismo operador
            (
                Pattern::Unary { op, operand },
                Expr::Unary(expr_op, expr_operand),
            ) => {
                if op == expr_op {
                    self.pattern_match(operand, expr_operand)
                } else {
                    None
                }
            }

            // Call: debe ser llamada con mismo nombre de función
            (
                Pattern::Call { func, args: pattern_args },
                Expr::Call { func: expr_func, args: expr_args },
            ) => {
                // Verificar que el nombre de la función coincida
                if func != expr_func {
                    return None;
                }

                // Verificar que el número de argumentos coincida
                if pattern_args.len() != expr_args.len() {
                    return None;
                }

                // Hacer match de cada argumento
                for (pat_arg, expr_arg) in pattern_args.iter().zip(expr_args.iter()) {
                    let arg_bindings = self.pattern_match(pat_arg, expr_arg)?;
                    bindings.extend(arg_bindings);
                }

                Some(bindings)
            }

            // Casos incompatibles: no hay match
            _ => None,
        }
    }
}

/// Expande macros en un programa
pub fn expand_macros(program: Program) -> Program {
    let mut ctx = MacroExpansionContext::new();
    ctx.expand_program(program)
}
