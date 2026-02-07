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
    pub params: Vec<Param>,      // For constructor arguments
    pub parent: Option<TypeInit>, // inheritance with args
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
    // --- Primitives ---
    Number(f64),
    String(String),
    Boolean(bool),
    Identifier(String),
    
    // --- Binary Operations ---
    Binary(Box<Spanned<Expr>>, Op, Box<Spanned<Expr>>),
    
    // --- Unary Operations ---
    Unary(UnOp, Box<Spanned<Expr>>),

    // --- Control Flow ---
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
    
    // --- Blocks & Scoping ---
    Block(Vec<Spanned<Expr>>), 
    
    Let {
        bindings: Vec<(String, Option<TypeAnnotation>, Spanned<Expr>)>,
        body: Box<Spanned<Expr>>,
    },
    
    Assignment {
        target: String, 
        value: Box<Spanned<Expr>>,
    },

    // --- Functions & Types ---
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

    // --- Type Checks ---
    Is(Box<Spanned<Expr>>, String),
    As(Box<Spanned<Expr>>, String),

    // --- Vectors ---
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

    // --- Mathematical Built-ins ---
    Sqrt(Box<Spanned<Expr>>),
    Sin(Box<Spanned<Expr>>),
    Cos(Box<Spanned<Expr>>),
    Exp(Box<Spanned<Expr>>),
    Log(Box<Spanned<Expr>>, Box<Spanned<Expr>>),
    Rand,
    PI,
    E,
    
    /// Error node for error recovery
    Error,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Add, Sub, Mul, Div, Mod, Pow,
    Eq, Neq, Lt, Gt, Le, Ge,
    And, Or,
    // String concat
    Concat, // @
    ConcatSpace, // @@
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnOp {
    Neg, // -
    Not, // !
}
