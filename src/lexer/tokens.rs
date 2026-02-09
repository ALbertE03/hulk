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
    Def,        // para definir macros
    Match,      // para pattern matching
    Case,       // para casos de match
    Default,    // para caso default en match

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
    Dollar,     // $ (para variable placeholders en macros)
    // @ se tokeniza como Concat, no At
    // At se usa solo conceptualmente en parsing de macros

    // Especial
    EOF,
    Unknown(char),
}
