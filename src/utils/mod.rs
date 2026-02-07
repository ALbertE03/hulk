use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

#[derive(Debug, Clone)]
pub struct Spanned<T> {
    pub node: T,
    pub pos: Position,
}

impl<T> Spanned<T> {
    pub fn new(node: T, pos: Position) -> Self {
        Self { node, pos }
    }
}
