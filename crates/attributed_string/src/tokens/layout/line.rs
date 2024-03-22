use super::{LayoutToken, SpanIndex};
use std::ops::Range;

/// Represents a line of text, composed of multiple spans for simplicity.
#[derive(Debug, Clone)]
pub struct LineToken {
    pub range: Range<usize>,
    pub span_indicies: Vec<SpanIndex>,
    // /// Alignment of the line within its container.
    // pub alignment: TextAlignment,
}

impl LayoutToken for LineToken {
    fn get_range(&self) -> &Range<usize> {
        &self.range
    }

    fn get_span_indices(&self) -> &Vec<SpanIndex> {
        &self.span_indicies
    }
}
