use crate::utils::Position;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum LexError {
    UnterminatedString(Position),
    UnterminatedBlockComment(Position),
    UnexpectedCharacter(char, Position),
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexError::UnterminatedString(pos) => write!(f, "Unterminated string at {}", pos),
            LexError::UnterminatedBlockComment(pos) => write!(f, "Unterminated block comment at {}", pos),
            LexError::UnexpectedCharacter(c, pos) => write!(f, "Unexpected character '{}' at {}", c, pos),
        }
    }
}
