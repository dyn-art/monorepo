use super::shape::{
    linebreak::LinebreakToken, text_fragment::TextFragmentToken,
    word_separator::WordSeparatorToken, ShapeBuffer, ShapeTokenVariant,
};
use crate::{attrs::Attrs, fonts_cache::FontsCache};
use glam::Vec2;
use std::ops::Range;
use unicode_linebreak::BreakClass;

/// A span of text with common attributes.
/// It is guranteed that a span only matches one attribute set.
// TODO: Should this token own a copy of attrs
#[derive(Debug, Clone)]
pub struct SpanToken {
    // Most likely exactly correlates to an attribute range if no bidi paragraph.
    range: Range<usize>,
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
}
