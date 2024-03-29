use super::ShapeToken;
use crate::glyph::Glyph;
use dyn_utils::units::em::Em;
use std::ops::Range;
use tiny_skia_path::Transform;

/// Represents an individual glyph.
#[derive(Debug, Clone)]
pub struct GlyphToken {
    glyph: Glyph,
    /// Cached transform after applying the layout.
    pub transform: Transform,
    /// Cached advance in horizontal direction after applying layout relative the font size.
    pub x_advance: Em,
    /// Cached advance in vertical direction after applying layout relative the font size.
    pub y_advance: Em,
}

impl GlyphToken {
    pub fn new(glyph: Glyph) -> Self {
        let x_advance = glyph.x_advance;
        let y_advance = glyph.y_advance;

        Self {
            glyph,
            transform: Transform::default(),
            x_advance,
            y_advance,
        }
    }

    pub fn get_glyph(&self) -> &Glyph {
        &self.glyph
    }
}

impl ShapeToken for GlyphToken {
    fn get_range(&self) -> &Range<usize> {
        &self.glyph.range
    }
}
