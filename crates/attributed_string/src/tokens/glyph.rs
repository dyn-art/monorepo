use super::Token;
use crate::glyph::Glyph;
use glam::Vec2;
use std::ops::Range;

/// Represents an individual glyph.
#[derive(Debug, Clone)]
pub struct GlyphToken {
    range: Range<usize>, // TODO: Redundant to glyph range
    glyph: Option<Glyph>,
    transform: Vec2,
}

impl GlyphToken {
    pub fn new(range: Range<usize>) -> Self {
        Self {
            range,
            glyph: None,
            transform: Vec2::default(),
        }
    }
}

impl Token for GlyphToken {
    fn get_range(&self) -> &Range<usize> {
        &self.range
    }
}
