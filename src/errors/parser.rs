use crate::utils::Position;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum ParseError {
    UnexpectedToken {
        expected: String,
        found: String,
        pos: Position,
    },
    UnexpectedEOF(Position),
    InvalidExpression(Position),
    Lex(crate::errors::LexError),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken { expected, found, pos } => {
                write!(f, "Expected '{}', found '{}' at {}", expected, found, pos)
            }
            ParseError::UnexpectedEOF(pos) => write!(f, "Unexpected end of file at {}", pos),
            ParseError::InvalidExpression(pos) => write!(f, "Invalid expression at {}", pos),
            ParseError::Lex(err) => write!(f, "{}", err),
        }
    }
}
