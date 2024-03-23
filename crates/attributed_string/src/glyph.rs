use crate::font::FontId;
use glam::Vec2;
use rustybuzz::ttf_parser::GlyphId;
use std::ops::Range;

#[derive(Debug, Default, Clone)]
pub struct Glyph {
    pub font_id: FontId,
    pub glyph_id: GlyphId,
    /// Position in bytes in the original string.
    pub range: Range<usize>,
    /// A size in physical units.
    ///
    /// It's different from advance in that it's not affected by letter spacing, ..
    pub size: Vec2,
    /// An advance in physical units.
    pub advance: Vec2,
    /// An offset in physical units.
    pub offset: Vec2,
    /// An ascent in physical units.
    pub ascent: f32,
    /// An descent in physical units.
    pub descent: f32,
}

impl Glyph {
    pub fn height(&self) -> f32 {
        self.ascent - self.descent
    }
}
