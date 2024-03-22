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
    /// Cached transform after applying the layout.
    transform: Vec2,
    /// Cached bounding box after applying the layout.
    bbox: Vec2,
}

impl GlyphToken {
    pub fn new(glyph: Glyph) -> Self {
        // log::info!("GlyphToken for range: {:?}", glyph.range);
        Self {
            glyph,
            transform: Vec2::default(),
            bbox: Vec2::default(),
        }
    }

    pub fn get_glyph(&self) -> &Glyph {
        &self.glyph
    }

    pub fn get_transform(&self) -> &Vec2 {
        &self.transform
    }

    pub fn set_transform(&mut self, transform: Vec2) {
        self.transform = transform;
    }
}

impl ShapeToken for GlyphToken {
    fn get_range(&self) -> &Range<usize> {
        &self.glyph.range
    }
}