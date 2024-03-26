use super::LineWrapStrategy;
use crate::{
    attrs::AttrsIntervals,
    tokens::{
        line::{LineToken, SpanRange},
        shape::{ShapeToken, ShapeTokenVariant},
        span::SpanToken,
    },
};
use dyn_utils::{properties::size::Size, units::abs::Abs};

/// Line wrap strategy that wraps text at word boundaries
/// or line breaks based on specified width constraints.
pub struct WordWrap {
    lines: Vec<LineToken>,
    current_line: Vec<SpanRange>,
    current_line_width: Abs,
    current_word: Vec<SpanRange>,
    current_word_width: Abs,
    in_overflow: bool,
}

impl WordWrap {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            current_line: Vec::new(),
            current_line_width: Abs::zero(),
            current_word: Vec::new(),
            current_word_width: Abs::zero(),
            in_overflow: false,
        }
    }

    fn start_new_line(&mut self) {
        if !self.current_line.is_empty() {
            let mut line = LineToken::new(std::mem::take(&mut self.current_line));
            line.merge_contiguous_spans();
            self.lines.push(line);
            self.current_line_width = Abs::zero();
        }

        // If current_word is not empty, it means we have a pending word to add to the new line
        if !self.current_word.is_empty() {
            self.current_line.append(&mut self.current_word);
            self.current_line_width = self.current_word_width; // Start with the width of the current word
            self.current_word_width = Abs::zero();
        }
    }

    fn add_word_part(&mut self, token_width: Abs, span_range: SpanRange) {
        self.current_word.push(span_range);
        self.current_word_width += token_width;
    }

    fn add_non_word_part(&mut self, token_width: Abs, span_range: SpanRange) {
        self.current_line.push(span_range);
        self.current_line_width += token_width;
    }

    fn finalize_word(&mut self) {
        self.current_line.append(&mut self.current_word);
        self.current_line_width += self.current_word_width;
        self.current_word_width = Abs::zero();
    }

    fn handle_wrap(&mut self, token_variant: &ShapeTokenVariant) {
        if self.in_overflow {
            self.start_new_line();
            if matches!(token_variant, ShapeTokenVariant::Linebreak(_)) {
                self.start_new_line();
            }
        } else {
            if !self.current_word.is_empty() {
                self.finalize_word();
            }
            self.start_new_line();
        }
    }
}

// TODO: Improve this implementation right now its not efficient in every way
impl LineWrapStrategy for WordWrap {
    fn compute_lines(
        &mut self,
        spans: &[SpanToken],
        attrs_intervals: &AttrsIntervals,
        size: &Size,
        _: &str,
    ) -> Vec<LineToken> {
        for (index, span) in spans.iter().enumerate() {
            let mut span_range_start = span.get_range().start;
            let attrs = &attrs_intervals.intervals[span.get_attrs_index()].val;
            let font_size = attrs.get_font_size();

            for token_variant in span.get_tokens() {
                let (token_width, token_range_end) = match token_variant {
                    ShapeTokenVariant::Glyph(token) => (
                        token.get_glyph().x_advance.at(font_size),
                        token.get_range().end,
                    ),
                    ShapeTokenVariant::TextFragment(token) => {
                        (token.x_advance().at(font_size), token.get_range().end)
                    }
                    ShapeTokenVariant::WordSeparator(token) => {
                        (token.x_advance().at(font_size), token.get_range().end)
                    }
                    ShapeTokenVariant::Linebreak(token) => (Abs::pt(0.0), token.get_range().end),
                    ShapeTokenVariant::Bitmap(token) => (Abs::pt(0.0), token.get_range().end),
                };

                let span_range = SpanRange::new(index, span_range_start..token_range_end);
                self.in_overflow =
                    self.current_line_width + self.current_word_width + token_width > size.rwidth();
                let is_word_part = matches!(
                    token_variant,
                    ShapeTokenVariant::Glyph(_)
                        | ShapeTokenVariant::TextFragment(_)
                        | ShapeTokenVariant::Bitmap(_)
                );
                let should_wrap =
                    self.in_overflow || matches!(token_variant, ShapeTokenVariant::Linebreak(_));
                // let current_text = &text[span_range_start..token_range_end];

                if is_word_part {
                    self.add_word_part(token_width, span_range);
                } else {
                    if should_wrap {
                        self.handle_wrap(token_variant);
                        self.add_non_word_part(token_width, span_range);
                    } else {
                        if !self.current_word.is_empty() {
                            self.finalize_word();
                        }
                        self.add_non_word_part(token_width, span_range);
                    }
                }

                span_range_start = token_range_end;
            }

            // Check for any remaining part of the current span to be added
            if span_range_start < span.get_range().end {
                self.current_line.push(SpanRange::new(
                    index,
                    span_range_start..span.get_range().end,
                ));
            }
        }

        // Flush any remaining parts to a new line
        self.start_new_line();
        if !self.current_line.is_empty() {
            self.start_new_line();
        }

        return std::mem::take(&mut self.lines);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        attrs::{Attrs, AttrsInterval},
        AttributedString, AttributedStringConfig, LineWrap,
    };
    use dyn_fonts_book::{
        font::{
            info::FontFamily,
            variant::{FontStyle, FontWeight},
        },
        FontsBook,
    };
    use std::ops::Range;

    fn init() {
        let _ = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
            .is_test(true)
            .try_init();
    }

    #[test]
    fn e2e_case1() {
        init();

        let mut fonts_book = FontsBook::new();
        fonts_book.load_system_fonts();

        let text = String::from("Hello, world!\nשלום עולם!\nThis is a mix of English and Hebrew.");
        let attrs_intervals = vec![
            AttrsInterval {
                start: 0,
                stop: 10,
                val: Attrs::new()
                    .font_family(FontFamily::Monospace)
                    .font_weight(FontWeight::REGULAR)
                    .font_size(Abs::pt(24.0)),
            },
            AttrsInterval {
                start: 10,
                stop: text.len(),
                val: Attrs::new()
                    .font_family(FontFamily::Serif)
                    .font_weight(FontWeight::REGULAR)
                    .font_size(Abs::pt(12.0)),
            },
        ];

        let mut attributed_string = AttributedString::new(
            text,
            attrs_intervals,
            AttributedStringConfig {
                size: Size::new(Abs::pt(150.0), Abs::pt(100.0)),
                line_wrap: LineWrap::Word,
            },
        );

        attributed_string.tokenize_text(&mut fonts_book);

        let lines = attributed_string.compute_lines();
        let line_ranges = lines
            .iter()
            .map(|line| line.get_span_ranges())
            .cloned()
            .collect::<Vec<_>>();

        let expected_line_ranges = vec![
            vec![SpanRange {
                index: 0,
                range: Range { start: 0, end: 7 },
            }],
            vec![
                SpanRange {
                    index: 0,
                    range: Range { start: 7, end: 10 },
                },
                SpanRange {
                    index: 1,
                    range: Range { start: 10, end: 13 },
                },
            ],
            vec![SpanRange {
                index: 2,
                range: Range { start: 14, end: 32 },
            }],
            vec![SpanRange {
                index: 3,
                range: Range { start: 33, end: 62 },
            }],
            vec![SpanRange {
                index: 3,
                range: Range { start: 62, end: 69 },
            }],
        ];

        assert_eq!(line_ranges, expected_line_ranges);
    }

    #[test]
    fn e2e_case2() {
        init();

        let mut fonts_book = FontsBook::new();
        fonts_book.load_system_fonts();

        let text = String::from("Hello there Jeff! Long line test testtestExtra small");
        let attrs_intervals = vec![
            AttrsInterval {
                start: 0,
                stop: 12,
                val: Attrs::new()
                    .font_family(FontFamily::Monospace)
                    .font_weight(FontWeight::REGULAR)
                    .font_style(FontStyle::Normal)
                    .font_size(Abs::pt(48.0)),
            },
            AttrsInterval {
                start: 12,
                stop: 16,
                val: Attrs::new()
                    .font_family(FontFamily::Serif)
                    .font_weight(FontWeight::REGULAR)
                    .font_style(FontStyle::Italic)
                    .font_size(Abs::pt(70.0)),
            },
            AttrsInterval {
                start: 16,
                stop: 41,
                val: Attrs::new()
                    .font_family(FontFamily::Monospace)
                    .font_weight(FontWeight::REGULAR)
                    .font_style(FontStyle::Normal)
                    .font_size(Abs::pt(48.0)),
            },
            AttrsInterval {
                start: 41,
                stop: 52,
                val: Attrs::new()
                    .font_family(FontFamily::Monospace)
                    .font_weight(FontWeight::REGULAR)
                    .font_style(FontStyle::Normal)
                    .font_size(Abs::pt(24.0)),
            },
        ];

        let mut attributed_string = AttributedString::new(
            text,
            attrs_intervals,
            AttributedStringConfig {
                size: Size::new(Abs::pt(378.0), Abs::pt(238.0)),
                line_wrap: LineWrap::Word,
            },
        );

        attributed_string.tokenize_text(&mut fonts_book);

        let lines = attributed_string.compute_lines();
        let line_ranges = lines
            .iter()
            .map(|line| line.get_span_ranges())
            .cloned()
            .collect::<Vec<_>>();

        let expected_line_ranges = vec![
            vec![SpanRange {
                index: 0,
                range: Range { start: 0, end: 12 },
            }],
            vec![
                SpanRange {
                    index: 1,
                    range: Range { start: 12, end: 16 },
                },
                SpanRange {
                    index: 2,
                    range: Range { start: 16, end: 23 },
                },
            ],
            vec![SpanRange {
                index: 2,
                range: Range { start: 23, end: 33 },
            }],
            vec![
                SpanRange {
                    index: 2,
                    range: Range { start: 33, end: 41 },
                },
                SpanRange {
                    index: 3,
                    range: Range { start: 41, end: 47 },
                },
            ],
            vec![SpanRange {
                index: 3,
                range: Range { start: 47, end: 52 },
            }],
        ];

        assert_eq!(line_ranges, expected_line_ranges);
    }
}
