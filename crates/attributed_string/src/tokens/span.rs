use super::shape::{
    glyph::GlyphToken, linebreak::LinebreakToken, text_fragment::TextFragmentToken,
    word_separator::WordSeparatorToken, ShapeBuffer, ShapeTokenVariant,
};
use crate::{attrs::Attrs, fonts_cache::FontsCache};
use glam::Vec2;
use std::ops::Range;
use unicode_linebreak::BreakClass;

/// A span of text with common attributes.
/// It is guranteed that a span only matches one attribute set.
#[derive(Debug, Clone)]
pub struct SpanToken {
    range: Range<usize>,
    attrs_index: usize,
    /// Shape tokens within the span, including glyphs, words, and separators.
    tokens: Vec<ShapeTokenVariant>,
    /// Bidi level for handling text directionality within the span.
    level: unicode_bidi::Level,
}

impl SpanToken {
    pub fn from_text(
        text: &str,
        range: Range<usize>,
        level: unicode_bidi::Level,
        attrs_index: usize,
        attrs: &Attrs,
        fonts_cache: &mut FontsCache,
    ) -> Self {
        let mut tokens: Vec<ShapeTokenVariant> = Vec::new();
        let span_text = &text[range.clone()];
        let mut shape_buffer = ShapeBuffer {
            buffer: Some(rustybuzz::UnicodeBuffer::new()),
        };

        log::info!("SpanToken for text: '{}' ({:?})", span_text, range);

        // Process each character for potential tokenization within the paragraph
        let mut start = range.start;
        for (index, _char) in span_text.char_indices() {
            let global_index = range.start + index; // Adjust index relative to the entire text
            let break_class = unicode_linebreak::break_property(_char as u32);

            match break_class {
                // Handle line break
                BreakClass::Mandatory
                | BreakClass::LineFeed
                | BreakClass::NextLine
                | BreakClass::CarriageReturn => {
                    // Add text fragment token
                    if start != global_index {
                        tokens.push(ShapeTokenVariant::TextFragment(
                            TextFragmentToken::from_text(
                                text,
                                start..global_index,
                                attrs,
                                &mut shape_buffer,
                                fonts_cache,
                            ),
                        ));
                    }

                    // Add line break token
                    tokens.push(ShapeTokenVariant::Linebreak(LinebreakToken::new(
                        global_index..global_index + 1,
                    )));
                    start = global_index + 1;
                }

                // Handle text segment separation
                BreakClass::Space | BreakClass::ZeroWidthSpace => {
                    // Add text fragment token
                    if start != global_index {
                        tokens.push(ShapeTokenVariant::TextFragment(
                            TextFragmentToken::from_text(
                                text,
                                start..global_index,
                                attrs,
                                &mut shape_buffer,
                                fonts_cache,
                            ),
                        ));
                    }

                    // Add word separator token
                    tokens.push(ShapeTokenVariant::WordSeparator(
                        WordSeparatorToken::from_text(
                            text,
                            global_index..global_index + 1,
                            attrs,
                            &mut shape_buffer,
                            fonts_cache,
                        ),
                    ));
                    start = global_index + 1;
                }
                _ => {}
            }
        }

        // Handle the last text fragment within the paragraph, if any
        if start < range.end {
            tokens.push(ShapeTokenVariant::TextFragment(
                TextFragmentToken::from_text(
                    text,
                    start..range.end,
                    attrs,
                    &mut shape_buffer,
                    fonts_cache,
                ),
            ));
        }

        return Self {
            range,
            attrs_index,
            tokens,
            level,
        };
    }

    #[inline]
    pub fn get_range(&self) -> &Range<usize> {
        &self.range
    }

    #[inline]
    pub fn get_tokens(&self) -> &Vec<ShapeTokenVariant> {
        &self.tokens
    }

    #[inline]
    pub fn get_level(&self) -> &unicode_bidi::Level {
        &self.level
    }

    pub fn get_bbox() -> Vec2 {
        // TODO
        Vec2::default()
    }

    pub fn get_transform() -> Vec2 {
        // TODO
        Vec2::default()
    }

    #[inline]
    pub fn iter_glyphs(&self) -> GlyphTokenRefIterator {
        GlyphTokenRefIterator::new(self)
    }
}

pub struct GlyphTokenRefIterator<'a> {
    span_token: &'a SpanToken,
    // Tracks the current position in the top-level tokens vector
    token_index: usize,
    // Tracks the position within the current ShapeTokenVariant's GlyphToken vector
    glyph_index: usize,
}

impl<'a> GlyphTokenRefIterator<'a> {
    pub fn new(span_token: &'a SpanToken) -> Self {
        Self {
            span_token,
            token_index: 0,
            glyph_index: 0,
        }
    }
}

impl<'a> Iterator for GlyphTokenRefIterator<'a> {
    type Item = &'a GlyphToken;

    fn next(&mut self) -> Option<Self::Item> {
        while self.token_index < self.span_token.tokens.len() {
            match &self.span_token.tokens[self.token_index] {
                // Move to next token after yielding a Glyph
                ShapeTokenVariant::Glyph(glyph) if self.glyph_index == 0 => {
                    self.token_index += 1;
                    return Some(glyph);
                }
                ShapeTokenVariant::WordSeparator(token)
                    if self.glyph_index < token.get_tokens().len() =>
                {
                    let glyph = &token.get_tokens()[self.glyph_index];
                    self.glyph_index += 1;
                    if self.glyph_index == token.get_tokens().len() {
                        // Reset glyph_index and move to next token for next call
                        self.glyph_index = 0;
                        self.token_index += 1;
                    }
                    return Some(glyph);
                }
                ShapeTokenVariant::TextFragment(token)
                    if self.glyph_index < token.get_tokens().len() =>
                {
                    let glyph = &token.get_tokens()[self.glyph_index];
                    self.glyph_index += 1;
                    if self.glyph_index == token.get_tokens().len() {
                        // Reset glyph_index and move to next token for next call
                        self.glyph_index = 0;
                        self.token_index += 1;
                    }
                    return Some(glyph);
                }
                // For non-glyph-carrying tokens or if no more glyphs in current token, move to the next one
                _ => {
                    self.token_index += 1;
                    self.glyph_index = 0;
                }
            }
        }

        // No more tokens or glyphs left
        None
    }
}
