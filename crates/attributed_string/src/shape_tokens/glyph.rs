use super::ShapeToken;
use crate::glyph::Glyph;
use dyn_utils::units::abs::Abs;
use std::ops::Range;
use tiny_skia_path::Transform;

/// Represents an individual glyph.
#[derive(Debug, Clone)]
pub struct GlyphToken {
    glyph: Glyph,
    pub layout: GlyphLayout,
}

impl GlyphToken {
    pub fn new(glyph: Glyph, font_size: Abs) -> Self {
        let x_advance = glyph.x_advance;
        let y_advance = glyph.y_advance;

        Self {
            glyph,
            layout: GlyphLayout::new(x_advance.at(font_size), y_advance.at(font_size)),
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

    fn x_advance(&self) -> Abs {
        self.layout.x_advance
    }

    fn y_advance(&self) -> Abs {
        self.layout.y_advance
    }
}

#[derive(Debug, Clone)]
pub struct GlyphLayout {
    /// Cached transform after applying the layout.
    // TODO: Should a glyph have multipe transforms? Like one for layout on line, spacing, .. level
    // which are then combined when outlining?
    pub transform: Transform,
    /// Cached advance in horizontal direction after applying layout.
    pub x_advance: Abs,
    /// Cached advance in vertical direction after applying layout.
    pub y_advance: Abs,
}

impl GlyphLayout {
    pub fn new(x_advance: Abs, y_advance: Abs) -> Self {
        Self {
            transform: Transform::default(),
            x_advance,
            y_advance,
        }
    }
}
