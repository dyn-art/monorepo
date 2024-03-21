use crate::attrs::AttrsIntervals;

use super::{glyph::GlyphToken, Token};
use glam::Vec2;
use std::ops::Range;

/// Groups glyphs into a continuous fragment of text, typically a word or number.
#[derive(Debug, Clone)]
pub struct TextFragmentToken {
    range: Range<usize>,
    /// Glyph tokens that make up the word.
    tokens: Vec<GlyphToken>,
    /// Cached advance after applying the layout.
    advance: Vec2,
}

impl TextFragmentToken {
    pub fn from_text(text: &str, range: Range<usize>, attrs_intervals: &AttrsIntervals) -> Self {
        Self {
            range,
            advance: Vec2::default(),
            tokens: Vec::new(), // TODO
        }
    }
}

impl Token for TextFragmentToken {
    fn get_range(&self) -> &Range<usize> {
        &self.range
    }
}
