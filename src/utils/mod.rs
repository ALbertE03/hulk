/// represented by a start and end byte offset.
pub type Span = (usize, usize);

/// A wrapper that attaches a span to a node.
#[derive(Debug, Clone)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn new(node: T, span: Span) -> Self {
        Self { node, span }
    }
}
