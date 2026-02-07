#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Palabras clave
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
    In, // para 'for x in ...'

    // Identificadores
    Identifier(String),

    // Literales
    Number(f64),
    StringLiteral(String),

    // Operadores
    // Aritmética
    Plus,       // +
    Minus,      // -
    Star,       // *
    Slash,      // /
    Percent,    // %
    Power,      // ^ 
    
    // Comparación
    Equal,          // ==
    NotEqual,       // !=
    LessThan,       // <
    GreaterThan,    // >
    LessThanEq,     // <=
    GreaterThanEq,  // >=

    // Lógico
    And,    // &
    Or,     // |
    Not,    // !

    // Cadenas
    Concat,         // @
    ConcatSpace,    // @@

    // Asignación / Definición
    Assign,         // =
    DestructAssign, // :=
    FuncArrow,      // => 
    TypeArrow,      // ->
    
    // Puntuación
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

    // Especial
    EOF,
    Unknown(char),
}
