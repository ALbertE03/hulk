
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
}

impl MacroExpansionContext {
    pub fn new() -> Self {
        MacroExpansionContext {
            macros: HashMap::new(),
            substitutions: HashMap::new(),
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
            // Llamadas a macros se convierten en expansión
            Expr::Call { func, args } => {
                if let Some(macro_decl) = self.macros.get(&func).cloned() {
                    // Expandir la macro
                    self.expand_macro_call(&macro_decl, args, pos)
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
                let expanded_cases = cases
                    .into_iter()
                    .map(|case| MatchCase {
                        pattern: case.pattern, // Patterns no se expanden
                        expr: self.expand_expr(case.expr),
                    })
                    .collect();

                Expr::Match {
                    expr: Box::new(self.expand_expr(*expr)),
                    cases: expanded_cases,
                    default: default.map(|d| Box::new(self.expand_expr(*d))),
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

        // Procesar parámetros
        let mut _body_arg: Option<Spanned<Expr>> = None;

        for (i, param) in macro_decl.params.iter().enumerate() {
            match param {
                MacroParam::Normal { name: _, .. } => {
                    // Argumento normal: sustituir nombre por expresión
                    if let Some(_arg) = args.get(i) {
                        // TODO: Aquí se debería hacer tree substitution real
                        // Por ahora solo guardamos el nombre
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

                MacroParam::Body { name: _, .. } => {
                    // Body argument: capturar expresión completa
                    // Típicamente el último argumento
                    if let Some(arg) = args.last() {
                        _body_arg = Some(arg.clone());
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

        expanded_body.node
    }

    /// Sanitiza variables en el body de una macro (renombra para evitar captura)
    pub fn sanitize_expr(&self, expr: &Spanned<Expr>) -> Spanned<Expr> {
        // TODO: Implementar renombrado real usando gensym
        // Por ahora retornar clon sin cambios
        expr.clone()
    }

    /// Aplica sustituciones a una expresión
    fn apply_substitutions(&self, expr: Spanned<Expr>) -> Spanned<Expr> {
        let pos = expr.pos;
        let node = match expr.node {
            Expr::Identifier(name) => {
                if let Some(substitution) = self.substitutions.get(&name) {
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
                        // Nota: No sustituimos nombres de variables definidas en let
                        // porque son locales a la macro/bloque. 
                        // Solo sustituimos en la inicialización
                        (name, ty, self.apply_substitutions(init))
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
