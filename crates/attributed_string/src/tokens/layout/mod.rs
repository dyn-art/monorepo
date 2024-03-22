pub mod line;
pub mod paragraph;

use self::{line::LineToken, paragraph::ParagraphToken};
use std::ops::Range;

pub trait LayoutToken {
    /// Byte range in the original text marking the token's start and stop indices.
    /// Inclusive start, exclusive of stop (start <= x < end).
    fn get_range(&self) -> &Range<usize>;
    /// References to the spans and thus glyphs in the token.
    fn get_span_indices(&self) -> &Vec<SpanIndex>;
}

#[derive(Debug, Clone)]
pub struct SpanIndex {
    index: usize,
    range: Range<usize>,
}

#[derive(Debug, Clone)]
pub enum LayoutTokenVariant {
    Line(LineToken),
    Paragraph(ParagraphToken),
}
