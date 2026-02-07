use crate::ast::nodes::*;
use crate::utils::Spanned;
use std::collections::HashMap;

/// Interner para reutilizar strings idénticos y reducir uso de memoria.
pub struct StringInterner {
    map: HashMap<String, String>,
}

impl StringInterner {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// Retorna una referencia compartida al string, creándola si no existe.
    pub fn intern(&mut self, s: String) -> String {
        if let Some(existing) = self.map.get(&s) {
            existing.clone()
        } else {
            self.map.insert(s.clone(), s.clone());
            s
        }
    }
}

/// Entorno para constant propagation: mapea nombres de variables a sus valores constantes conocidos.
type ConstEnv = HashMap<String, Expr>;

/// Determina si una expresión es un valor constante (puede propagarse).
fn is_constant(expr: &Expr) -> bool {
    matches!(expr, Expr::Number(_) | Expr::Boolean(_) | Expr::String(_) | Expr::PI | Expr::E)
}

/// Recursively collects all variable names that appear as targets of `:=` assignments.
/// These variables are mutable and must NOT be constant-propagated.
fn collect_assigned_vars(expr: &Expr, out: &mut std::collections::HashSet<String>) {
    match expr {
        Expr::Assignment { target, value } => {
            out.insert(target.clone());
            collect_assigned_vars(&value.node, out);
        }
        Expr::AttributeAssignment { obj, value, .. } => {
            collect_assigned_vars(&obj.node, out);
            collect_assigned_vars(&value.node, out);
        }
        Expr::Block(exprs) => {
            for e in exprs { collect_assigned_vars(&e.node, out); }
        }
        Expr::Let { bindings, body } => {
            for (_, _, init) in bindings { collect_assigned_vars(&init.node, out); }
            collect_assigned_vars(&body.node, out);
        }
        Expr::If { cond, then_expr, else_expr } => {
            collect_assigned_vars(&cond.node, out);
            collect_assigned_vars(&then_expr.node, out);
            collect_assigned_vars(&else_expr.node, out);
        }
        Expr::While { cond, body } => {
            collect_assigned_vars(&cond.node, out);
            collect_assigned_vars(&body.node, out);
        }
        Expr::For { iterable, body, .. } => {
            collect_assigned_vars(&iterable.node, out);
            collect_assigned_vars(&body.node, out);
        }
        Expr::Binary(l, _, r) => {
            collect_assigned_vars(&l.node, out);
            collect_assigned_vars(&r.node, out);
        }
        Expr::Unary(_, e) => collect_assigned_vars(&e.node, out),
        Expr::Call { args, .. } | Expr::BaseCall { args } => {
            for a in args { collect_assigned_vars(&a.node, out); }
        }
        Expr::MethodCall { obj, args, .. } => {
            collect_assigned_vars(&obj.node, out);
            for a in args { collect_assigned_vars(&a.node, out); }
        }
        Expr::Lambda { body, .. } => collect_assigned_vars(&body.node, out),
        Expr::Match { expr: e, cases, default } => {
            collect_assigned_vars(&e.node, out);
            for c in cases { collect_assigned_vars(&c.expr.node, out); }
            if let Some(d) = default { collect_assigned_vars(&d.node, out); }
        }
        _ => {}
    }
}

/// Optimiza un programa completo aplicando constant folding, constant propagation, dead code elimination y string interning.
pub fn optimize_program(program: Program) -> Program {
    let mut interner = StringInterner::new();
    let env = HashMap::new();
    
    let declarations = program.declarations
        .into_iter()
        .map(|decl| optimize_declaration(decl, &mut interner, &env))
        .collect();
    
    let expr = optimize_expr(program.expr, &mut interner, &env);
    
    Program { declarations, expr }
}

/// Optimiza una declaración.
fn optimize_declaration(decl: Declaration, interner: &mut StringInterner, env: &ConstEnv) -> Declaration {
    match decl {
        Declaration::Function(func) => {
            Declaration::Function(FunctionDecl {
                name: interner.intern(func.name),
                params: func.params.into_iter().map(|p| optimize_param(p, interner)).collect(),
                return_type: func.return_type.map(|t| optimize_type_annotation(t, interner)),
                body: optimize_expr(func.body, interner, env),
            })
        }
        Declaration::Type(ty) => {
            Declaration::Type(TypeDecl {
                name: interner.intern(ty.name),
                params: ty.params.into_iter().map(|p| optimize_param(p, interner)).collect(),
                parent: ty.parent.map(|p| TypeInit {
                    name: interner.intern(p.name),
                    args: p.args.into_iter().map(|a| optimize_expr(a, interner, env)).collect(),
                }),
                attributes: ty.attributes.into_iter().map(|a| Attribute {
                    name: interner.intern(a.name),
                    type_annotation: a.type_annotation.map(|t| optimize_type_annotation(t, interner)),
                    init: optimize_expr(a.init, interner, env),
                }).collect(),
                methods: ty.methods.into_iter().map(|m| FunctionDecl {
                    name: interner.intern(m.name),
                    params: m.params.into_iter().map(|p| optimize_param(p, interner)).collect(),
                    return_type: m.return_type.map(|t| optimize_type_annotation(t, interner)),
                    body: optimize_expr(m.body, interner, env),
                }).collect(),
            })
        }
        Declaration::Protocol(proto) => {
            Declaration::Protocol(ProtocolDecl {
                name: interner.intern(proto.name),
                parent: proto.parent.map(|p| interner.intern(p)),
                methods: proto.methods.into_iter().map(|m| MethodSignature {
                    name: interner.intern(m.name),
                    params: m.params.into_iter().map(|p| optimize_param(p, interner)).collect(),
                    return_type: optimize_type_annotation(m.return_type, interner),
                }).collect(),
            })
        }
        Declaration::Macro(macro_decl) => {
            // Las macros se expanden antes de la optimización
            // Por ahora, simplemente pasarlas sin optimizar
            Declaration::Macro(macro_decl)
        }
    }
}

fn optimize_param(param: Param, interner: &mut StringInterner) -> Param {
    Param {
        name: interner.intern(param.name),
        type_annotation: param.type_annotation.map(|t| optimize_type_annotation(t, interner)),
    }
}

fn optimize_type_annotation(ty: TypeAnnotation, interner: &mut StringInterner) -> TypeAnnotation {
    match ty {
        TypeAnnotation::Name(s) => TypeAnnotation::Name(interner.intern(s)),
        TypeAnnotation::Function { params, return_type } => {
            TypeAnnotation::Function {
                params: params.into_iter().map(|p| optimize_type_annotation(p, interner)).collect(),
                return_type: Box::new(optimize_type_annotation(*return_type, interner)),
            }
        }
        TypeAnnotation::Iterable(inner) => {
            TypeAnnotation::Iterable(Box::new(optimize_type_annotation(*inner, interner)))
        }
    }
}

/// Optimiza una expresión aplicando constant folding y dead code elimination.
fn optimize_expr(expr: Spanned<Expr>, interner: &mut StringInterner, env: &ConstEnv) -> Spanned<Expr> {
    let pos = expr.pos.clone();
    let optimized_node = match expr.node {
        // CONSTANT PROPAGATION: reemplazar variable por su valor si es conocido
        Expr::Identifier(ref name) => {
            if let Some(value) = env.get(name) {
                // La variable tiene un valor constante conocido, propagarlo
                value.clone()
            } else {
                // Variable desconocida o no constante, mantenerla
                Expr::Identifier(interner.intern(name.clone()))
            }
        }
        
        // Constant folding para operaciones binarias
        Expr::Binary(left, op, right) => {
            let opt_left = optimize_expr(*left, interner, env);
            let opt_right = optimize_expr(*right, interner, env);
            
            match (&opt_left.node, &op, &opt_right.node) {
                // Operaciones aritméticas con números
                (Expr::Number(a), Op::Add, Expr::Number(b)) => Expr::Number(a + b),
                (Expr::Number(a), Op::Sub, Expr::Number(b)) => Expr::Number(a - b),
                (Expr::Number(a), Op::Mul, Expr::Number(b)) => Expr::Number(a * b),
                (Expr::Number(a), Op::Div, Expr::Number(b)) if *b != 0.0 => Expr::Number(a / b),
                (Expr::Number(a), Op::Mod, Expr::Number(b)) if *b != 0.0 => Expr::Number(a % b),
                (Expr::Number(a), Op::Pow, Expr::Number(b)) => Expr::Number(a.powf(*b)),
                
                // Comparaciones
                (Expr::Number(a), Op::Eq, Expr::Number(b)) => Expr::Boolean(a == b),
                (Expr::Number(a), Op::Neq, Expr::Number(b)) => Expr::Boolean(a != b),
                (Expr::Number(a), Op::Lt, Expr::Number(b)) => Expr::Boolean(a < b),
                (Expr::Number(a), Op::Gt, Expr::Number(b)) => Expr::Boolean(a > b),
                (Expr::Number(a), Op::Le, Expr::Number(b)) => Expr::Boolean(a <= b),
                (Expr::Number(a), Op::Ge, Expr::Number(b)) => Expr::Boolean(a >= b),
                
                // Booleanos
                (Expr::Boolean(a), Op::And, Expr::Boolean(b)) => Expr::Boolean(*a && *b),
                (Expr::Boolean(a), Op::Or, Expr::Boolean(b)) => Expr::Boolean(*a || *b),
                
                // Short-circuit booleano
                (Expr::Boolean(false), Op::And, _) => Expr::Boolean(false),
                (Expr::Boolean(true), Op::Or, _) => Expr::Boolean(true),
                (Expr::Boolean(true), Op::And, _) => opt_right.node,
                (Expr::Boolean(false), Op::Or, _) => opt_right.node,
                
                // Concatenación de strings
                (Expr::String(a), Op::Concat, Expr::String(b)) => {
                    Expr::String(interner.intern(format!("{}{}", a, b)))
                }
                (Expr::String(a), Op::ConcatSpace, Expr::String(b)) => {
                    Expr::String(interner.intern(format!("{} {}", a, b)))
                }
                
                // Simplificaciones algebraicas
                (_, Op::Add, Expr::Number(0.0)) => opt_left.node,
                (Expr::Number(0.0), Op::Add, _) => opt_right.node,
                (_, Op::Sub, Expr::Number(0.0)) => opt_left.node,
                (_, Op::Mul, Expr::Number(1.0)) => opt_left.node,
                (Expr::Number(1.0), Op::Mul, _) => opt_right.node,
                (_, Op::Mul, Expr::Number(0.0)) => Expr::Number(0.0),
                (Expr::Number(0.0), Op::Mul, _) => Expr::Number(0.0),
                (_, Op::Div, Expr::Number(1.0)) => opt_left.node,
                (_, Op::Pow, Expr::Number(1.0)) => opt_left.node,
                (_, Op::Pow, Expr::Number(0.0)) => Expr::Number(1.0),
                
                _ => Expr::Binary(Box::new(opt_left), op, Box::new(opt_right)),
            }
        }
        
        // Operaciones unarias
        Expr::Unary(op, inner) => {
            let opt_inner = optimize_expr(*inner, interner, env);
            match (&op, &opt_inner.node) {
                (UnOp::Neg, Expr::Number(n)) => Expr::Number(-n),
                (UnOp::Not, Expr::Boolean(b)) => Expr::Boolean(!b),
                _ => Expr::Unary(op, Box::new(opt_inner)),
            }
        }
        
        // Dead code elimination en if
        Expr::If { cond, then_expr, else_expr } => {
            let opt_cond = optimize_expr(*cond, interner, env);
            
            match &opt_cond.node {
                Expr::Boolean(true) => optimize_expr(*then_expr, interner, env).node,
                Expr::Boolean(false) => optimize_expr(*else_expr, interner, env).node,
                _ => Expr::If {
                    cond: Box::new(opt_cond),
                    then_expr: Box::new(optimize_expr(*then_expr, interner, env)),
                    else_expr: Box::new(optimize_expr(*else_expr, interner, env)),
                },
            }
        }
        
        // Dead code elimination en while
        Expr::While { cond, body } => {
            let opt_cond = optimize_expr(*cond, interner, env);
            
            match &opt_cond.node {
                Expr::Boolean(false) => Expr::Block(vec![]), // while(false) se elimina
                _ => Expr::While {
                    cond: Box::new(opt_cond),
                    body: Box::new(optimize_expr(*body, interner, env)),
                },
            }
        }
        
        // For
        Expr::For { var, iterable, body } => {
            Expr::For {
                var: interner.intern(var),
                iterable: Box::new(optimize_expr(*iterable, interner, env)),
                body: Box::new(optimize_expr(*body, interner, env)),
            }
        }
        
        // Bloques
        Expr::Block(exprs) => {
            Expr::Block(exprs.into_iter().map(|e| optimize_expr(e, interner, env)).collect())
        }
        
        // Let con CONSTANT PROPAGATION (respeta variables mutadas por :=)
        Expr::Let { bindings, body } => {
            // Primero, escanear el body para encontrar variables reasignadas con :=
            let mut mutated = std::collections::HashSet::new();
            collect_assigned_vars(&body.node, &mut mutated);

            let mut new_env = env.clone();
            let new_bindings: Vec<_> = bindings.into_iter().map(|(name, ty, init)| {
                // Optimizar la inicialización con el entorno actual
                let opt_init = optimize_expr(init, interner, &new_env);
                
                // Solo propagar si el valor es constante Y la variable no se reasigna
                if is_constant(&opt_init.node) && !mutated.contains(&name) {
                    new_env.insert(name.clone(), opt_init.node.clone());
                } else {
                    // Si la variable es mutada, removerla del env por si una
                    // definición exterior la había registrado
                    new_env.remove(&name);
                }
                
                (
                    interner.intern(name),
                    ty.map(|t| optimize_type_annotation(t, interner)),
                    opt_init,
                )
            }).collect();
            
            // Optimizar el cuerpo con el nuevo entorno que incluye las constantes
            Expr::Let {
                bindings: new_bindings,
                body: Box::new(optimize_expr(*body, interner, &new_env)),
            }
        }
        
        // Assignment
        Expr::Assignment { target, value } => {
            Expr::Assignment {
                target: interner.intern(target),
                value: Box::new(optimize_expr(*value, interner, env)),
            }
        }
        
        // AttributeAssignment
        Expr::AttributeAssignment { obj, attribute, value } => {
            Expr::AttributeAssignment {
                obj: Box::new(optimize_expr(*obj, interner, env)),
                attribute: interner.intern(attribute),
                value: Box::new(optimize_expr(*value, interner, env)),
            }
        }
        
        // Llamadas
        Expr::Call { func, args } => {
            Expr::Call {
                func: interner.intern(func),
                args: args.into_iter().map(|a| optimize_expr(a, interner, env)).collect(),
            }
        }
        
        Expr::BaseCall { args } => {
            Expr::BaseCall {
                args: args.into_iter().map(|a| optimize_expr(a, interner, env)).collect(),
            }
        }
        
        Expr::MethodCall { obj, method, args } => {
            Expr::MethodCall {
                obj: Box::new(optimize_expr(*obj, interner, env)),
                method: interner.intern(method),
                args: args.into_iter().map(|a| optimize_expr(a, interner, env)).collect(),
            }
        }
        
        Expr::AttributeAccess { obj, attribute } => {
            Expr::AttributeAccess {
                obj: Box::new(optimize_expr(*obj, interner, env)),
                attribute: interner.intern(attribute),
            }
        }
        
        Expr::Instantiation { ty, args } => {
            Expr::Instantiation {
                ty: interner.intern(ty),
                args: args.into_iter().map(|a| optimize_expr(a, interner, env)).collect(),
            }
        }
        
        Expr::Is(expr, ty) => {
            Expr::Is(
                Box::new(optimize_expr(*expr, interner, env)),
                interner.intern(ty),
            )
        }
        
        Expr::As(expr, ty) => {
            Expr::As(
                Box::new(optimize_expr(*expr, interner, env)),
                interner.intern(ty),
            )
        }
        
        // Vectores
        Expr::VectorLiteral(exprs) => {
            Expr::VectorLiteral(exprs.into_iter().map(|e| optimize_expr(e, interner, env)).collect())
        }
        
        Expr::VectorGenerator { expr, var, iterable } => {
            Expr::VectorGenerator {
                expr: Box::new(optimize_expr(*expr, interner, env)),
                var: interner.intern(var),
                iterable: Box::new(optimize_expr(*iterable, interner, env)),
            }
        }
        
        Expr::Indexing { obj, index } => {
            Expr::Indexing {
                obj: Box::new(optimize_expr(*obj, interner, env)),
                index: Box::new(optimize_expr(*index, interner, env)),
            }
        }
        
        // Lambda
        Expr::Lambda { params, return_type, body } => {
            Expr::Lambda {
                params: params.into_iter().map(|p| optimize_param(p, interner)).collect(),
                return_type: return_type.map(|t| optimize_type_annotation(t, interner)),
                body: Box::new(optimize_expr(*body, interner, env)),
            }
        }
        
        // Funciones matemáticas - constant folding
        Expr::Sqrt(inner) => {
            let opt = optimize_expr(*inner, interner, env);
            match &opt.node {
                Expr::Number(n) if *n >= 0.0 => Expr::Number(n.sqrt()),
                _ => Expr::Sqrt(Box::new(opt)),
            }
        }
        
        Expr::Sin(inner) => {
            let opt = optimize_expr(*inner, interner, env);
            match &opt.node {
                Expr::Number(n) => Expr::Number(n.sin()),
                _ => Expr::Sin(Box::new(opt)),
            }
        }
        
        Expr::Cos(inner) => {
            let opt = optimize_expr(*inner, interner, env);
            match &opt.node {
                Expr::Number(n) => Expr::Number(n.cos()),
                _ => Expr::Cos(Box::new(opt)),
            }
        }
        
        Expr::Exp(inner) => {
            let opt = optimize_expr(*inner, interner, env);
            match &opt.node {
                Expr::Number(n) => Expr::Number(n.exp()),
                _ => Expr::Exp(Box::new(opt)),
            }
        }
        
        Expr::Log(base, val) => {
            let opt_base = optimize_expr(*base, interner, env);
            let opt_val = optimize_expr(*val, interner, env);
            match (&opt_base.node, &opt_val.node) {
                (Expr::Number(b), Expr::Number(v)) if *b > 0.0 && *v > 0.0 => {
                    Expr::Number(v.log(*b))
                }
                _ => Expr::Log(Box::new(opt_base), Box::new(opt_val)),
            }
        }
        
        // String interning
        Expr::String(s) => Expr::String(interner.intern(s)),
        
        // Valores constantes (sin cambios)
        other => other,
    };
    
    Spanned::new(optimized_node, pos)
}
