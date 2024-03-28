use super::LineWrapStrategy;
use crate::{
    line::Line,
    shape_tokens::{ShapeToken, ShapeTokenVariant},
    span::SpanIntervals,
};
use dyn_utils::{properties::size::Size, units::abs::Abs};
use rust_lapper::Interval;
use std::ops::Range;

/// Line wrap strategy that wraps text at word boundaries
/// or line breaks based on specified width constraints.
pub struct WordWrap {
    lines: Vec<Line>,
    current_line: Vec<Range<usize>>,
    current_line_width: Abs,
    current_word: Vec<Range<usize>>,
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

    fn start_new_line(&mut self, last: bool) {
        // Append current word to current line if it's the last line and it fits
        if last && !self.in_overflow && !self.current_word.is_empty() {
            self.current_line.append(&mut self.current_word);
            self.current_line_width += self.current_word_width;
            self.current_word_width = Abs::zero();
        }

        // Add the current line to lines and prepare for next line
        if !self.current_line.is_empty() {
            let mut line = Line::new(std::mem::take(&mut self.current_line));
            line.merge_contiguous_ranges();
            self.lines.push(line);
            self.current_line_width = Abs::zero();
        }

        // Prepare the next line with the current word if it's not empty
        if !self.current_word.is_empty() {
            self.current_line.append(&mut self.current_word);
            self.current_line_width = self.current_word_width;
            self.current_word_width = Abs::zero();
        }

        // If the last line caused an overflow, don't forget the last word
        if last && !self.current_line.is_empty() {
            self.start_new_line(false);
        }
    }

    fn add_word_part(&mut self, token_width: Abs, range: Range<usize>) {
        self.current_word.push(range);
        self.current_word_width += token_width;
    }

    fn add_non_word_part(&mut self, token_width: Abs, range: Range<usize>) {
        self.current_line.push(range);
        self.current_line_width += token_width;
    }

    fn finalize_word(&mut self) {
        self.current_line.append(&mut self.current_word);
        self.current_line_width += self.current_word_width;
        self.current_word_width = Abs::zero();
    }

    fn handle_wrap(&mut self, token_variant: &ShapeTokenVariant) {
        if self.in_overflow {
            self.start_new_line(false);
            if matches!(token_variant, ShapeTokenVariant::Linebreak(_)) {
                self.start_new_line(false);
            }
        } else {
            if !self.current_word.is_empty() {
                self.finalize_word();
            }
            self.start_new_line(false);
        }
    }
}

// TODO: Improve this implementation right now its not efficient in every way
impl LineWrapStrategy for WordWrap {
    fn compute_lines(&mut self, spans: &SpanIntervals, size: &Size, _: &str) -> Vec<Line> {
        for (index, Interval { val: span, .. }) in spans.iter().enumerate() {
            let mut span_range_start = span.get_range().start;
            let font_size = span.get_attrs().get_font_size();

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

                let range = span_range_start..token_range_end;
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
                    self.add_word_part(token_width, range);
                } else {
                    if should_wrap {
                        self.handle_wrap(token_variant);
                        // TODO: Should the space be removed? or added to the next line?
                        // self.add_non_word_part(token_width, span_range);
                    } else {
                        if !self.current_word.is_empty() {
                            self.finalize_word();
                        }
                        self.add_non_word_part(token_width, range);
                    }
                }

                span_range_start = token_range_end;
            }

            // Check for any remaining part of the current span to be added
            if span_range_start < span.get_range().end {
                self.current_line
                    .push(span_range_start..span.get_range().end);
            }
        }

        // Flush any remaining parts to a new line
        self.start_new_line(true);

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
            .map(|line| line.get_ranges())
            .cloned()
            .collect::<Vec<_>>();

        let expected_line_ranges = vec![
            vec![0..7],
            vec![7..10, 10..13],
            vec![14..32],
            vec![33..62],
            vec![62..69],
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
            .map(|line| line.get_ranges())
            .cloned()
            .collect::<Vec<_>>();

        let expected_line_ranges = vec![
            vec![0..12],
            vec![12..16, 16..23],
            vec![23..33],
            vec![33..41, 41..47],
            vec![47..52],
        ];

        assert_eq!(line_ranges, expected_line_ranges);
    }

    #[test]
    fn e2e_case3() {
        init();

        let mut fonts_book = FontsBook::new();
        fonts_book.load_system_fonts();

        let text = String::from("BLOCKCHAIN        SOLANA");
        let text_len = text.len();
        let attrs_intervals = vec![AttrsInterval {
            start: 0,
            stop: text_len,
            val: Attrs::new()
                .font_family(FontFamily::Monospace)
                .font_weight(FontWeight::REGULAR)
                .font_style(FontStyle::Normal)
                .font_size(Abs::pt(14.0)),
        }];

        let mut attributed_string = AttributedString::new(
            text,
            attrs_intervals,
            AttributedStringConfig {
                size: Size::new(Abs::pt(202.0), Abs::pt(16.0)),
                line_wrap: LineWrap::Word,
            },
        );

        attributed_string.tokenize_text(&mut fonts_book);

        let lines = attributed_string.compute_lines();
        let line_ranges = lines
            .iter()
            .map(|line| line.get_ranges())
            .cloned()
            .collect::<Vec<_>>();

        let expected_line_ranges = vec![vec![0..text_len]];

        assert_eq!(line_ranges, expected_line_ranges);
    }
}
