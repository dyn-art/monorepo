use crate::attrs::AttrsIntervals;

use super::{glyph::GlyphToken, Token};
use std::ops::Range;

/// Represents spaces or punctuation between words.
#[derive(Debug, Clone)]
pub struct WordSeparatorToken {
    /// The glyph token used as a separator.
    token: GlyphToken,
}

impl WordSeparatorToken {
    pub fn from_text(text: &str, range: Range<usize>, attrs_intervals: &AttrsIntervals) -> Self {
        Self {
            token: GlyphToken::new(range)
        }
    }
}

impl Token for WordSeparatorToken {
    fn get_range(&self) -> &Range<usize> {
        &self.token.get_range()
    }
}
