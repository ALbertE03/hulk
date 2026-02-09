use crate::ast::nodes::{Declaration, Expr, ExprVisitor, MacroDecl, MacroParam, Pattern, Program};
use crate::utils::{Position, Spanned};
use std::collections::HashMap;
use super::visitors::{MacroExpansionVisitor, SanitizationVisitor, SubstitutionVisitor};
use super::utils::gensym;

/// Contexto de expansión de macro
pub struct MacroExpansionContext {
    /// Macros disponibles (nombre -> declaración)
    pub macros: HashMap<String, MacroDecl>,
    /// Sustituciones activas (nombre original -> nombre generado)
    pub substitutions: HashMap<String, String>,
    /// Expression substitutions for Normal macro params (name -> expr)
    pub expr_substitutions: HashMap<String, Spanned<Expr>>,
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
        let mut visitor = MacroExpansionVisitor::new(self);
        visitor.visit_expr(expr)
    }

    /// Expande una llamada a macro
    pub fn expand_macro_call(
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
        let mut visitor = SanitizationVisitor::new(self);
        visitor.visit_expr(expr.clone())
    }

    pub fn sanitize_pattern(&self, pattern: &Pattern, scope: &mut HashMap<String, String>) -> Pattern {
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
        let mut visitor = SubstitutionVisitor::new(&self.substitutions, &self.expr_substitutions);
        visitor.visit_expr(expr)
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
