use glam::Vec2;
use rustybuzz::ttf_parser::GlyphId;
use std::ops::Range;

pub struct Glyph {
    pub font_id: fontdb::ID,
    pub glyph_id: GlyphId,
    pub range: Range<usize>,
    pub advance: Vec2,
    pub offset: Vec2,
    pub ascent: f32,
    pub descent: f32,
}

pub fn shape_text() -> Vec<Glyph> {
    Vec::new()
}

pub fn shape_text_with_font() -> Option<Vec<Glyph>> {
    None
}
