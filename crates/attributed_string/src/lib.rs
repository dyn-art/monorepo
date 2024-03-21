pub mod attrs;
pub mod bidi_para;
pub mod font;
pub mod fonts_cache;
pub mod glyph;
pub mod path_builder;
pub mod tokens;

use attrs::{Attrs, AttrsInterval, AttrsIntervals};
use bidi_para::BidiParagraphs;
use glam::Vec2;
use rust_lapper::Lapper;
use std::ops::Range;
use tokens::{
    linebreak::LinebreakToken, span::SpanToken, text_fragment::TextFragmentToken,
    word_separator::WordSeparatorToken, TokenVariant,
};
use unicode_linebreak::BreakClass;

#[derive(Debug, Clone)]
struct AttributedString {
    text: String,
    token_stream: Vec<TokenVariant>,
    attrs_intervals: AttrsIntervals,
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
        let mut token_stream: Vec<TokenVariant> = Vec::new();
        let text_start = self.text.as_ptr() as usize;

        // Process bidi paragraphs
        let bidi_para_range_iter = BidiParagraphs::new(&self.text).map(|(para, level)| {
            let start = para.as_ptr() as usize - text_start;
            let end = start + para.len();
            (start..end, level)
        });

        for (para_range, para_level) in bidi_para_range_iter {
            let mut start = para_range.start;
            let para_text = &self.text[para_range.clone()];
            let mut span_token = SpanToken::new(para_range.clone(), para_level);

            // Process each character for potential tokenization within the paragraph
            for (index, _char) in para_text.char_indices() {
                let global_index = start + index; // Adjust index relative to the entire text
                let break_class = unicode_linebreak::break_property(_char as u32);

                match break_class {
                    BreakClass::Mandatory
                    | BreakClass::LineFeed
                    | BreakClass::NextLine
                    | BreakClass::CarriageReturn => {
                        // Add text fragment token
                        if start != global_index {
                            span_token.push_token(TokenVariant::TextFragment(
                                TextFragmentToken::from_text(
                                    &self.text,
                                    Range {
                                        start,
                                        end: global_index,
                                    },
                                    &self.attrs_intervals,
                                ),
                            ));
                        }

                        // Add line break token
                        span_token.push_token(TokenVariant::Linebreak(LinebreakToken::new(
                            Range {
                                start: global_index,
                                end: global_index + 1,
                            },
                        )));
                        start = global_index + 1;
                    }
                    BreakClass::Space | BreakClass::ZeroWidthSpace => {
                        // Add text fragment token
                        if start != global_index {
                            span_token.push_token(TokenVariant::TextFragment(
                                TextFragmentToken::from_text(
                                    &self.text,
                                    Range {
                                        start,
                                        end: global_index,
                                    },
                                    &self.attrs_intervals,
                                ),
                            ));
                        }

                        // Add word separator token
                        span_token.push_token(TokenVariant::WordSeparator(
                            WordSeparatorToken::from_text(
                                &self.text,
                                Range {
                                    start: global_index,
                                    end: global_index + 1,
                                },
                                &self.attrs_intervals,
                            ),
                        ));
                        start = global_index + 1;
                    }
                    _ => {}
                }
            }

            // Handle the last text fragment within the paragraph, if any
            if start < para_range.end {
                span_token.push_token(TokenVariant::TextFragment(TextFragmentToken::from_text(
                    &self.text,
                    Range {
                        start,
                        end: para_range.end,
                    },
                    &self.attrs_intervals,
                )));
            }

            token_stream.push(TokenVariant::Span(span_token));
        }

        self.token_stream = token_stream;
    }

    pub fn layout(&mut self) {
        // TODO: Layout tokens by createing lines, ..
    }
}

#[cfg(test)]
mod tests {
    use self::attrs::FontFamily;
    use super::*;

    #[test]
    fn e2e() {
        let text = String::from("Hello, world! שלום עולם! This is a mix of English and Hebrew.");
        let attrs_intervals = vec![
            AttrsInterval {
                start: 0,
                stop: 10,
                val: Attrs::new()
                    .font_family(FontFamily::Monospace)
                    .font_weight(400)
                    .font_size(24.0),
            },
            AttrsInterval {
                start: 10,
                stop: text.len(),
                val: Attrs::new()
                    .font_family(FontFamily::Serif)
                    .font_weight(400)
                    .font_size(12.0),
            },
        ];
        let mut attributed_string =
            AttributedString::new(text, attrs_intervals, Vec2::new(100.0, 50.0));

        attributed_string.tokenize();

        assert_eq!(attributed_string.token_stream.is_empty(), false);
    }
}
