use crate::utils::Spanned;

#[derive(Debug, Clone)]
pub struct Program {
    pub declarations: Vec<Declaration>,
    pub expr: Spanned<Expr>,
}

#[derive(Debug, Clone)]
pub enum Declaration {
    Function(FunctionDecl),
    Type(TypeDecl),
    Protocol(ProtocolDecl),
}

#[derive(Debug, Clone)]
pub struct FunctionDecl {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Option<String>,
    pub body: Spanned<Expr>,
}

#[derive(Debug, Clone)]
pub struct TypeDecl {
    pub name: String,
    pub params: Vec<Param>,      // For constructor arguments
    pub parent: Option<TypeInit>, // inheritance with args
    pub attributes: Vec<Attribute>,
    pub methods: Vec<FunctionDecl>,
}

#[derive(Debug, Clone)]
pub struct ProtocolDecl {
    pub name: String,
    pub parent: Option<String>,
    pub methods: Vec<MethodSignature>,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub type_annotation: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TypeInit {
    pub name: String,
    pub args: Vec<Spanned<Expr>>,
}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: String,
    pub type_annotation: Option<String>,
    pub init: Spanned<Expr>,
}

#[derive(Debug, Clone)]
pub struct MethodSignature {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: String,
}

#[derive(Debug, Clone)]
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
        bindings: Vec<(String, Option<String>, Spanned<Expr>)>,
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
    MethodCall {
        obj: Box<Spanned<Expr>>,
        method: String,
        args: Vec<Spanned<Expr>>,
    },
    Instantiation {
        ty: String,
        args: Vec<Spanned<Expr>>,
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
    
    /// Error node for error recovery
    Error,
}

#[derive(Debug, Clone)]
pub enum Op {
    Add, Sub, Mul, Div, Mod, Pow,
    Eq, Neq, Lt, Gt, Le, Ge,
    And, Or,
    // String concat
    Concat, // @
    ConcatSpace, // @@
}

#[derive(Debug, Clone)]
pub enum UnOp {
    Neg, // -
    Not, // !
}
