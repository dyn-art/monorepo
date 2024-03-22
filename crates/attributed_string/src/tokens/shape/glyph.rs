use super::ShapeToken;
use crate::{
    attrs::Attrs,
    fonts_cache::FontsCache,
    glyph::{self, Glyph},
};
use glam::Vec2;
use std::ops::Range;

/// Represents an individual glyph.
#[derive(Debug, Clone)]
pub struct GlyphToken {
    glyph: Glyph,
    transform: Vec2,
}

impl GlyphToken {
    pub fn new(glyph: Glyph) -> Self {
        // log::info!("GlyphToken for range: {:?}", glyph.range);
        Self {
            glyph,
            transform: Vec2::default(),
        }
    }
}

impl ShapeToken for GlyphToken {
    fn get_range(&self) -> &Range<usize> {
        &self.glyph.range
    }
}
