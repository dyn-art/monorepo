use glam::Vec2;

use super::{glyph::GlyphToken, ShapeBuffer, ShapeToken};
use crate::{attrs::Attrs, fonts_cache::FontsCache};
use std::ops::Range;

/// Represents spaces or punctuation between words.
#[derive(Debug, Clone)]
pub struct WordSeparatorToken {
    range: Range<usize>,
    /// Glyph tokens that make up the word separator.
    tokens: Vec<GlyphToken>,
    /// Cached advance after applying the layout.
    advance: Vec2,
}

impl WordSeparatorToken {
    pub fn from_text(
        text: &str,
        range: Range<usize>,
        attrs: &Attrs,
        shape_buffer: &mut ShapeBuffer,
        fonts_cache: &mut FontsCache,
    ) -> Self {
        let mut tokens: Vec<GlyphToken> = Vec::with_capacity(range.len());

        log::info!(
            "WordSeparatorToken for text: '{}' ({:?})",
            &text[range.clone()],
            range
        );

        if let Some(font) = fonts_cache.get_font_by_attrs(attrs) {
            let (glyphs, buffer) = font.shape_text_with_fallback(
                text,
                range.clone(),
                shape_buffer.buffer.take().unwrap_or_default(),
                fonts_cache,
            );
            shape_buffer.buffer = Some(buffer);
            tokens.extend(glyphs.into_iter().map(|glyph| GlyphToken::new(glyph)));
        }

        return Self {
            range,
            tokens,
            advance: Vec2::default(),
        };
    }
}

impl ShapeToken for WordSeparatorToken {
    fn get_range(&self) -> &Range<usize> {
        &self.range
    }
}
