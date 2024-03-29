use super::ShapeToken;
use crate::glyph::Glyph;
use dyn_utils::units::abs::Abs;
use std::ops::Range;
use tiny_skia_path::Transform;

/// Represents an individual glyph.
#[derive(Debug, Clone)]
pub struct GlyphToken {
    glyph: Glyph,
    /// Cached transform after applying the layout.
    pub transform: Transform,
    /// Cached advance in horizontal direction after applying layout.
    pub x_advance: Abs,
    /// Cached advance in vertical direction after applying layout.
    pub y_advance: Abs,
}

impl GlyphToken {
    pub fn new(glyph: Glyph, font_size: Abs) -> Self {
        let x_advance = glyph.x_advance;
        let y_advance = glyph.y_advance;

        Self {
            glyph,
            transform: Transform::default(),
            x_advance: x_advance.at(font_size),
            y_advance: y_advance.at(font_size),
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

    fn get_width(&self) -> Abs {
        self.x_advance
    }

    fn get_height(&self) -> Abs {
        self.y_advance
    }
}
