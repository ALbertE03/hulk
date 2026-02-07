#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Keywords
    Function,
    Let,
    If,
    Else,
    While,
    For,
    Type,
    New,
    Inherits,
    Protocol,
    Extends,
    Is,
    As,
    Elif,
    Base,
    Print, 
    True,
    False,
    In, // for 'for x in ...'

    // Identifiers
    Identifier(String),

    // Literals
    Number(f64),
    StringLiteral(String),

    // Operators
    // Arithmetic
    Plus,       // +
    Minus,      // -
    Star,       // *
    Slash,      // /
    Percent,    // %
    Power,      // ^ 
    
    // Comparison
    Equal,          // ==
    NotEqual,       // !=
    LessThan,       // <
    GreaterThan,    // >
    LessThanEq,     // <=
    GreaterThanEq,  // >=

    // Logical
    And,    // &
    Or,     // |
    Not,    // !

    // String
    Concat,         // @
    ConcatSpace,    // @@

    // Assignment / Definition
    Assign,         // =
    DestructAssign, // :=
    FuncArrow,      // => 
    TypeArrow,      // ->
    
    // Punctuation
    LParen,     // (
    RParen,     // )
    LBrace,     // {
    RBrace,     // }
    LBracket,   // [
    RBracket,   // ]
    Comma,      // ,
    Colon,      // :
    Dot,        // .
    Semicolon,  // ;

    // Special
    EOF,
    Unknown(char),
}
