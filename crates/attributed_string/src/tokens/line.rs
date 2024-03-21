use super::{span::SpanToken, Token};
use std::ops::Range;

/// Represents a line of text, composed of multiple spans for simplicity.
#[derive(Debug, Clone)]
pub struct LineToken {
    pub range: Range<usize>,
    /// Span tokens that make up the line.
    pub tokens: Vec<SpanToken>,
    // /// Alignment of the line within its container.
    // pub alignment: TextAlignment,
}

impl Token for LineToken {
    fn get_range(&self) -> &Range<usize> {
        &self.range
    }
}
