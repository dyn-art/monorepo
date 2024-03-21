use super::{line::LineToken, Token};
use std::ops::Range;

/// Encapsulates a paragraph, potentially containing multiple lines with varied directionality and alignment.
#[derive(Debug, Clone)]
pub struct ParagraphToken {
    pub range: Range<usize>,
    /// Line tokens that make up the paragraph.
    pub lines: Vec<LineToken>,
    /// Indentation level for the paragraph, useful for blockquotes or nested lists.
    pub indent_level: u32,
    /// Spacing before the paragraph.
    pub space_before: f32,
    /// Spacing after the paragraph.
    pub space_after: f32,
}

impl Token for ParagraphToken {
    fn get_range(&self) -> &Range<usize> {
        &self.range
    }
}
