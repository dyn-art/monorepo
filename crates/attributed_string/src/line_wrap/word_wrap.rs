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
        for Interval { val: span, .. } in spans.iter() {
            let mut span_range_start = span.get_range().start;

            for token_variant in span.get_tokens() {
                let (token_width, token_range_end) = match token_variant {
                    ShapeTokenVariant::Glyph(token) => (token.x_advance, token.get_range().end),
                    ShapeTokenVariant::TextFragment(token) => {
                        (token.x_advance(), token.get_range().end)
                    }
                    ShapeTokenVariant::WordSeparator(token) => {
                        (token.x_advance(), token.get_range().end)
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
                        if self.in_overflow {
                            self.add_non_word_part(token_width, range);
                        }
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
