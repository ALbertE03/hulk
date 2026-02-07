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
