use super::ShapeToken;
use crate::glyph::Glyph;
use std::ops::Range;
use tiny_skia_path::Transform;

/// Represents an individual glyph.
#[derive(Debug, Clone)]
pub struct GlyphToken {
    glyph: Glyph,
    /// Transform after applying the layout.
    transform: Transform,
}

impl GlyphToken {
    pub fn new(glyph: Glyph) -> Self {
        Self {
            glyph,
            transform: Transform::default(),
        }
    }

    pub fn get_glyph(&self) -> &Glyph {
        &self.glyph
    }

    pub fn get_transform(&self) -> &Transform {
        &self.transform
    }

    pub fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }
}

impl ShapeToken for GlyphToken {
    fn get_range(&self) -> &Range<usize> {
        &self.glyph.range
    }
}
