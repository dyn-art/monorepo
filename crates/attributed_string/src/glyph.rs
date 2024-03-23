use crate::font::FontId;
use glam::Vec2;
use rustybuzz::ttf_parser::GlyphId;
use std::ops::Range;

#[derive(Debug, Default, Clone)]
pub struct Glyph {
    pub font_id: FontId,
    pub glyph_id: GlyphId,
    pub range: Range<usize>,
    pub size: Vec2,
    pub advance: Vec2,
    pub offset: Vec2,
    pub ascent: f32,
    pub descent: f32,
}
