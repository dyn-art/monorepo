pub mod attrs;
pub mod font;
pub mod fonts_cache;
pub mod glyph;
pub mod path_builder;
pub mod token;

use attrs::{Attrs, AttrsInterval};
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

impl AttributedString {
    pub fn new(text: String, attrs_intervals: Vec<AttrsInterval>, bbox: Vec2) -> Self {
        let mut attrs_intervals = Lapper::new(attrs_intervals);
        attrs_intervals.divide_overlaps_with(|overlaps| {
            let mut merged_attrs = Attrs::new();
            for &attrs in overlaps.iter() {
                merged_attrs.merge(attrs.clone());
            }
            return merged_attrs;
        });

        Self {
            text,
            token_stream: Vec::new(),
            attrs_intervals,
            bbox,
        }
    }

    pub fn tokenize(&mut self) {
        // TODO
    }
}
