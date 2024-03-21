pub mod attrs;
pub mod font;
pub mod fonts_cache;
pub mod glyph;
pub mod path_builder;
pub mod token;

use attrs::Attrs;
use glam::Vec2;
use rust_lapper::Lapper;
use token::Token;

#[derive(Debug, Clone)]
struct AttributedString {
    text: String,
    token_stream: Vec<Token>,
    attrs_intervals: Lapper<usize, Attrs>,
    bbox: Vec2,
}
