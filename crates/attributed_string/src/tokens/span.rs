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
    pub fn get_attrs_index(&self) -> usize {
        self.attrs_index
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

    pub fn iter_glyphs<'a>(&'a self) -> impl Iterator<Item = &'a GlyphToken> + 'a {
        self.tokens
            .iter()
            .flat_map(|token_variant| match token_variant {
                ShapeTokenVariant::Glyph(token) => Box::new(std::iter::once(token))
                    as Box<dyn Iterator<Item = &'a GlyphToken> + 'a>,
                ShapeTokenVariant::TextFragment(token) => Box::new(token.get_tokens().iter())
                    as Box<dyn Iterator<Item = &'a GlyphToken> + 'a>,
                ShapeTokenVariant::WordSeparator(token) => Box::new(token.get_tokens().iter())
                    as Box<dyn Iterator<Item = &'a GlyphToken> + 'a>,
                _ => Box::new(std::iter::empty()) as Box<dyn Iterator<Item = &'a GlyphToken> + 'a>,
            })
    }

    pub(crate) fn iter_glyphs_mut<'a>(
        &'a mut self,
    ) -> impl Iterator<Item = &'a mut GlyphToken> + 'a {
        self.tokens
            .iter_mut()
            .flat_map(|token_variant| match token_variant {
                ShapeTokenVariant::Glyph(token) => Box::new(std::iter::once(token))
                    as Box<dyn Iterator<Item = &'a mut GlyphToken> + 'a>,
                ShapeTokenVariant::TextFragment(token) => {
                    Box::new(token.get_tokens_mut().iter_mut())
                        as Box<dyn Iterator<Item = &'a mut GlyphToken> + 'a>
                }
                ShapeTokenVariant::WordSeparator(token) => {
                    Box::new(token.get_tokens_mut().iter_mut())
                        as Box<dyn Iterator<Item = &'a mut GlyphToken> + 'a>
                }
                _ => Box::new(std::iter::empty())
                    as Box<dyn Iterator<Item = &'a mut GlyphToken> + 'a>,
            })
    }
}
