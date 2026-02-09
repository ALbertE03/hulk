use crate::utils::Spanned;

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub declarations: Vec<Declaration>,
    pub expr: Spanned<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Declaration {
    Function(FunctionDecl),
    Type(TypeDecl),
    Protocol(ProtocolDecl),
    Macro(MacroDecl),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeAnnotation {
    Name(String),
    Function {
        params: Vec<TypeAnnotation>,
        return_type: Box<TypeAnnotation>,
    },
    Iterable(Box<TypeAnnotation>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDecl {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Option<TypeAnnotation>,
    pub body: Spanned<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeDecl {
    pub name: String,
    pub params: Vec<Param>,      // Para argumentos del constructor
    pub parent: Option<TypeInit>, // Herencia con argumentos
    pub attributes: Vec<Attribute>,
    pub methods: Vec<FunctionDecl>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProtocolDecl {
    pub name: String,
    pub parent: Option<String>,
    pub methods: Vec<MethodSignature>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub name: String,
    pub type_annotation: Option<TypeAnnotation>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeInit {
    pub name: String,
    pub args: Vec<Spanned<Expr>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Attribute {
    pub name: String,
    pub type_annotation: Option<TypeAnnotation>,
    pub init: Spanned<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MethodSignature {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: TypeAnnotation,
}

// --- Macros ---

#[derive(Debug, Clone, PartialEq)]
pub struct MacroDecl {
    pub name: String,
    pub params: Vec<MacroParam>,
    pub return_type: Option<TypeAnnotation>,
    pub body: Spanned<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MacroParam {
    // Parámetro normal: se expande como expresión
    Normal { name: String, type_annotation: TypeAnnotation },
    // Parámetro simbólico (@): se pasa el nombre de la variable
    Symbolic { name: String, type_annotation: TypeAnnotation },
    // Placeholder ($): introduce nueva variable en el scope
    Placeholder { name: String, type_annotation: TypeAnnotation },
    // Body (*): captura bloque de expresiones
    Body { name: String, type_annotation: TypeAnnotation },
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchCase {
    pub pattern: Pattern,
    pub expr: Spanned<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    // Literal: 0, 1, "hello", true
    Literal(Expr),
    // Variable: x, y (captura cualquier valor)
    Variable { name: String, type_annotation: Option<TypeAnnotation> },
    // Binaria: (x + y), (a * b)
    Binary {
        left: Box<Pattern>,
        op: Op,
        right: Box<Pattern>,
    },
    // Unaria: -x, !y
    Unary {
        op: UnOp,
        operand: Box<Pattern>,
    },
    // Llamada: f(x, y)
    Call {
        func: String,
        args: Vec<Pattern>,
    },
    // Wildcard: _ (ignora valor)
    Wildcard,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    // --- Primitivos ---
    Number(f64),
    String(String),
    Boolean(bool),
    Identifier(String),
    
    // --- Operaciones binarias ---
    Binary(Box<Spanned<Expr>>, Op, Box<Spanned<Expr>>),
    
    // --- Operaciones unarias ---
    Unary(UnOp, Box<Spanned<Expr>>),

    // --- Flujo de control ---
    If {
        cond: Box<Spanned<Expr>>,
        then_expr: Box<Spanned<Expr>>,
        else_expr: Box<Spanned<Expr>>, 
    },
    While {
        cond: Box<Spanned<Expr>>,
        body: Box<Spanned<Expr>>,
    },
    For {
        var: String,
        iterable: Box<Spanned<Expr>>,
        body: Box<Spanned<Expr>>,
    },
    
    // --- Bloques y ámbito ---
    Block(Vec<Spanned<Expr>>), 
    
    Let {
        bindings: Vec<(String, Option<TypeAnnotation>, Spanned<Expr>)>,
        body: Box<Spanned<Expr>>,
    },
    
    Assignment {
        target: String, 
        value: Box<Spanned<Expr>>,
    },

    /// Asignación a atributo: self.attr := expr
    AttributeAssignment {
        obj: Box<Spanned<Expr>>,
        attribute: String,
        value: Box<Spanned<Expr>>,
    },

    // --- Funciones y tipos ---
    Call {
        func: String,
        args: Vec<Spanned<Expr>>,
    },
    BaseCall {
        args: Vec<Spanned<Expr>>,
    },
    MethodCall {
        obj: Box<Spanned<Expr>>,
        method: String,
        args: Vec<Spanned<Expr>>,
    },
    AttributeAccess {
        obj: Box<Spanned<Expr>>,
        attribute: String,
    },
    Instantiation {
        ty: String,
        args: Vec<Spanned<Expr>>,
    },

    // --- Lambdas ---
    Lambda {
        params: Vec<Param>,
        return_type: Option<TypeAnnotation>,
        body: Box<Spanned<Expr>>,
    },

    // --- Comprobaciones de tipo ---
    Is(Box<Spanned<Expr>>, String),
    As(Box<Spanned<Expr>>, String),

    // --- Vectores ---
    VectorLiteral(Vec<Spanned<Expr>>),
    VectorGenerator {
        expr: Box<Spanned<Expr>>,
        var: String,
        iterable: Box<Spanned<Expr>>,
    },
    Indexing {
        obj: Box<Spanned<Expr>>,
        index: Box<Spanned<Expr>>,
    },

    // --- Funciones matemáticas integradas ---
    Sqrt(Box<Spanned<Expr>>),
    Sin(Box<Spanned<Expr>>),
    Cos(Box<Spanned<Expr>>),
    Exp(Box<Spanned<Expr>>),
    Log(Box<Spanned<Expr>>, Box<Spanned<Expr>>),
    Rand,
    PI,
    E,
    
    // --- Pattern Matching (para macros) ---
    Match {
        expr: Box<Spanned<Expr>>,
        cases: Vec<MatchCase>,
        default: Option<Box<Spanned<Expr>>>,
    },
    
    /// Nodo de error para recuperación de errores
    Error,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Add, Sub, Mul, Div, Mod, Pow,
    Eq, Neq, Lt, Gt, Le, Ge,
    And, Or,
    // Concatenación de cadenas
    Concat, // @
    ConcatSpace, // @@
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnOp {
    Neg, // - (negativo)
    Not, // ! (negación lógica)
}

// --- Visitor Pattern ---

pub trait ExprVisitor {
    /// Visita un nodo de expresión y retorna la expresión transformada
    fn visit_expr(&mut self, expr: Spanned<Expr>) -> Spanned<Expr> {
        let pos = expr.pos;
        let node = match expr.node {
            Expr::Number(_) | Expr::String(_) | Expr::Boolean(_) | 
            Expr::Rand | Expr::PI | Expr::E | Expr::Error => expr.node,
            
            Expr::Identifier(name) => self.visit_identifier(name, pos),
            
            Expr::Binary(left, op, right) => self.visit_binary(*left, op, *right, pos),
            Expr::Unary(op, operand) => self.visit_unary(op, *operand, pos),
            
            Expr::If { cond, then_expr, else_expr } => 
                self.visit_if(*cond, *then_expr, *else_expr, pos),
            
            Expr::While { cond, body } => self.visit_while(*cond, *body, pos),
            
            Expr::For { var, iterable, body } => 
                self.visit_for(var, *iterable, *body, pos),
            
            Expr::Block(exprs) => self.visit_block(exprs, pos),
            
            Expr::Let { bindings, body } => self.visit_let(bindings, *body, pos),
            
            Expr::Assignment { target, value } => 
                self.visit_assignment(target, *value, pos),
            
            Expr::AttributeAssignment { obj, attribute, value } => 
                self.visit_attribute_assignment(*obj, attribute, *value, pos),
            
            Expr::Call { func, args } => self.visit_call(func, args, pos),
            
            Expr::BaseCall { args } => self.visit_base_call(args, pos),
            
            Expr::MethodCall { obj, method, args } => 
                self.visit_method_call(*obj, method, args, pos),
            
            Expr::AttributeAccess { obj, attribute } => 
                self.visit_attribute_access(*obj, attribute, pos),
            
            Expr::Instantiation { ty, args } => 
                self.visit_instantiation(ty, args, pos),
            
            Expr::Lambda { params, return_type, body } => 
                self.visit_lambda(params, return_type, *body, pos),
            
            Expr::Is(expr, ty) => self.visit_is(*expr, ty, pos),
            Expr::As(expr, ty) => self.visit_as(*expr, ty, pos),
            
            Expr::VectorLiteral(elements) => self.visit_vector_literal(elements, pos),
            
            Expr::VectorGenerator { expr, var, iterable } => 
                self.visit_vector_generator(*expr, var, *iterable, pos),
            
            Expr::Indexing { obj, index } => self.visit_indexing(*obj, *index, pos),
            
            Expr::Sqrt(e) => self.visit_sqrt(*e, pos),
            Expr::Sin(e) => self.visit_sin(*e, pos),
            Expr::Cos(e) => self.visit_cos(*e, pos),
            Expr::Exp(e) => self.visit_exp(*e, pos),
            Expr::Log(base, x) => self.visit_log(*base, *x, pos),
            
            Expr::Match { expr, cases, default } => 
                self.visit_match(*expr, cases, default, pos),
        };
        
        Spanned::new(node, pos)
    }
    
    // Métodos específicos para cada tipo de nodo
    // Por defecto, realizan una visita recursiva estándar
    
    fn visit_identifier(&mut self, name: String, _pos: crate::utils::Position) -> Expr {
        Expr::Identifier(name)
    }
    
    fn visit_binary(&mut self, left: Spanned<Expr>, op: Op, right: Spanned<Expr>, _pos: crate::utils::Position) -> Expr {
        Expr::Binary(
            Box::new(self.visit_expr(left)),
            op,
            Box::new(self.visit_expr(right)),
        )
    }
    
    fn visit_unary(&mut self, op: UnOp, operand: Spanned<Expr>, _pos: crate::utils::Position) -> Expr {
        Expr::Unary(op, Box::new(self.visit_expr(operand)))
    }
    
    fn visit_if(&mut self, cond: Spanned<Expr>, then_expr: Spanned<Expr>, 
                else_expr: Spanned<Expr>, _pos: crate::utils::Position) -> Expr {
        Expr::If {
            cond: Box::new(self.visit_expr(cond)),
            then_expr: Box::new(self.visit_expr(then_expr)),
            else_expr: Box::new(self.visit_expr(else_expr)),
        }
    }
    
    fn visit_while(&mut self, cond: Spanned<Expr>, body: Spanned<Expr>, _pos: crate::utils::Position) -> Expr {
        Expr::While {
            cond: Box::new(self.visit_expr(cond)),
            body: Box::new(self.visit_expr(body)),
        }
    }
    
    fn visit_for(&mut self, var: String, iterable: Spanned<Expr>, 
                 body: Spanned<Expr>, _pos: crate::utils::Position) -> Expr {
        Expr::For {
            var,
            iterable: Box::new(self.visit_expr(iterable)),
            body: Box::new(self.visit_expr(body)),
        }
    }
    
    fn visit_block(&mut self, exprs: Vec<Spanned<Expr>>, _pos: crate::utils::Position) -> Expr {
        Expr::Block(exprs.into_iter().map(|e| self.visit_expr(e)).collect())
    }
    
    fn visit_let(&mut self, bindings: Vec<(String, Option<TypeAnnotation>, Spanned<Expr>)>, 
                 body: Spanned<Expr>, _pos: crate::utils::Position) -> Expr {
        let new_bindings = bindings
            .into_iter()
            .map(|(name, ty, init)| (name, ty, self.visit_expr(init)))
            .collect();
        Expr::Let {
            bindings: new_bindings,
            body: Box::new(self.visit_expr(body)),
        }
    }
    
    fn visit_assignment(&mut self, target: String, value: Spanned<Expr>, _pos: crate::utils::Position) -> Expr {
        Expr::Assignment {
            target,
            value: Box::new(self.visit_expr(value)),
        }
    }
    
    fn visit_attribute_assignment(&mut self, obj: Spanned<Expr>, attribute: String, 
                                  value: Spanned<Expr>, _pos: crate::utils::Position) -> Expr {
        Expr::AttributeAssignment {
            obj: Box::new(self.visit_expr(obj)),
            attribute,
            value: Box::new(self.visit_expr(value)),
        }
    }
    
    fn visit_call(&mut self, func: String, args: Vec<Spanned<Expr>>, _pos: crate::utils::Position) -> Expr {
        Expr::Call {
            func,
            args: args.into_iter().map(|a| self.visit_expr(a)).collect(),
        }
    }
    
    fn visit_base_call(&mut self, args: Vec<Spanned<Expr>>, _pos: crate::utils::Position) -> Expr {
        Expr::BaseCall {
            args: args.into_iter().map(|a| self.visit_expr(a)).collect(),
        }
    }
    
    fn visit_method_call(&mut self, obj: Spanned<Expr>, method: String, 
                        args: Vec<Spanned<Expr>>, _pos: crate::utils::Position) -> Expr {
        Expr::MethodCall {
            obj: Box::new(self.visit_expr(obj)),
            method,
            args: args.into_iter().map(|a| self.visit_expr(a)).collect(),
        }
    }
    
    fn visit_attribute_access(&mut self, obj: Spanned<Expr>, attribute: String, _pos: crate::utils::Position) -> Expr {
        Expr::AttributeAccess {
            obj: Box::new(self.visit_expr(obj)),
            attribute,
        }
    }
    
    fn visit_instantiation(&mut self, ty: String, args: Vec<Spanned<Expr>>, _pos: crate::utils::Position) -> Expr {
        Expr::Instantiation {
            ty,
            args: args.into_iter().map(|a| self.visit_expr(a)).collect(),
        }
    }
    
    fn visit_lambda(&mut self, params: Vec<Param>, return_type: Option<TypeAnnotation>, 
                   body: Spanned<Expr>, _pos: crate::utils::Position) -> Expr {
        Expr::Lambda {
            params,
            return_type,
            body: Box::new(self.visit_expr(body)),
        }
    }
    
    fn visit_is(&mut self, expr: Spanned<Expr>, ty: String, _pos: crate::utils::Position) -> Expr {
        Expr::Is(Box::new(self.visit_expr(expr)), ty)
    }
    
    fn visit_as(&mut self, expr: Spanned<Expr>, ty: String, _pos: crate::utils::Position) -> Expr {
        Expr::As(Box::new(self.visit_expr(expr)), ty)
    }
    
    fn visit_vector_literal(&mut self, elements: Vec<Spanned<Expr>>, _pos: crate::utils::Position) -> Expr {
        Expr::VectorLiteral(elements.into_iter().map(|e| self.visit_expr(e)).collect())
    }
    
    fn visit_vector_generator(&mut self, expr: Spanned<Expr>, var: String, 
                             iterable: Spanned<Expr>, _pos: crate::utils::Position) -> Expr {
        Expr::VectorGenerator {
            expr: Box::new(self.visit_expr(expr)),
            var,
            iterable: Box::new(self.visit_expr(iterable)),
        }
    }
    
    fn visit_indexing(&mut self, obj: Spanned<Expr>, index: Spanned<Expr>, _pos: crate::utils::Position) -> Expr {
        Expr::Indexing {
            obj: Box::new(self.visit_expr(obj)),
            index: Box::new(self.visit_expr(index)),
        }
    }
    
    fn visit_sqrt(&mut self, expr: Spanned<Expr>, _pos: crate::utils::Position) -> Expr {
        Expr::Sqrt(Box::new(self.visit_expr(expr)))
    }
    
    fn visit_sin(&mut self, expr: Spanned<Expr>, _pos: crate::utils::Position) -> Expr {
        Expr::Sin(Box::new(self.visit_expr(expr)))
    }
    
    fn visit_cos(&mut self, expr: Spanned<Expr>, _pos: crate::utils::Position) -> Expr {
        Expr::Cos(Box::new(self.visit_expr(expr)))
    }
    
    fn visit_exp(&mut self, expr: Spanned<Expr>, _pos: crate::utils::Position) -> Expr {
        Expr::Exp(Box::new(self.visit_expr(expr)))
    }
    
    fn visit_log(&mut self, base: Spanned<Expr>, x: Spanned<Expr>, _pos: crate::utils::Position) -> Expr {
        Expr::Log(
            Box::new(self.visit_expr(base)),
            Box::new(self.visit_expr(x)),
        )
    }
    
    fn visit_match(&mut self, expr: Spanned<Expr>, cases: Vec<MatchCase>, 
                  default: Option<Box<Spanned<Expr>>>, _pos: crate::utils::Position) -> Expr {
        Expr::Match {
            expr: Box::new(self.visit_expr(expr)),
            cases: cases.into_iter().map(|case| MatchCase {
                pattern: case.pattern,
                expr: self.visit_expr(case.expr),
            }).collect(),
            default: default.map(|d| Box::new(self.visit_expr(*d))),
        }
    }
}
