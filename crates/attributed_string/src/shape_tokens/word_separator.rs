use super::{glyph::GlyphToken, ShapeBuffer, ShapeToken};
use crate::{attrs::Attrs, shape::shape_text_with_fallback};
use dyn_fonts_book::FontsBook;
use dyn_utils::units::{abs::Abs, em::Em};
use std::ops::Range;

/// Represents spaces or punctuation between words.
#[derive(Debug, Clone)]
pub struct WordSeparatorToken {
    range: Range<usize>,
    /// Glyph tokens that make up the word separator.
    tokens: Vec<GlyphToken>,
}

impl WordSeparatorToken {
    pub fn from_text(
        text: &str,
        range: Range<usize>,
        attrs: &Attrs,
        shape_buffer: &mut ShapeBuffer,
        fonts_book: &mut FontsBook,
    ) -> Self {
        let mut tokens: Vec<GlyphToken> = Vec::with_capacity(range.len());

        log::info!(
            "WordSeparatorToken for text: '{}' ({:?})",
            &text[range.clone()],
            range
        );

        if let Some(font) = fonts_book.get_font_by_info(attrs.get_font_info()) {
            let (glyphs, buffer) = shape_text_with_fallback(
                text,
                range.clone(),
                shape_buffer.buffer.take().unwrap_or_default(),
                &font,
                fonts_book,
            );
            shape_buffer.buffer = Some(buffer);
            tokens.extend(
                glyphs
                    .into_iter()
                    .map(|glyph| GlyphToken::new(glyph, attrs.get_font_size())),
            );
        }

        return Self { range, tokens };
    }

    pub fn get_tokens(&self) -> &Vec<GlyphToken> {
        &self.tokens
    }

    pub(crate) fn get_tokens_mut(&mut self) -> &mut Vec<GlyphToken> {
        &mut self.tokens
    }

    pub fn x_advance(&self) -> Abs {
        self.tokens
            .iter()
            .fold(Abs::zero(), |acc, token| acc + token.x_advance)
    }
}

impl ShapeToken for WordSeparatorToken {
    fn get_range(&self) -> &Range<usize> {
        &self.range
    }
}
