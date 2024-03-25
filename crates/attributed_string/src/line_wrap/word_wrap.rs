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
pub struct WordWrap;

// TODO: Does not work
impl LineWrapStrategy for WordWrap {
    fn compute_lines(
        &mut self,
        spans: &[SpanToken],
        attrs_intervals: &AttrsIntervals,
        size: &Size,
    ) -> Vec<LineToken> {
        let mut lines: Vec<LineToken> = Vec::new();

        let mut current_span_ranges: Vec<SpanRange> = Vec::new();
        let mut current_line_width = Abs::pt(0.0);

        let mut overflown_span_ranges: Vec<SpanRange> = Vec::new();
        let mut current_word_width = Abs::pt(0.0);
        let mut in_overflow = false;

        for (index, span) in spans.iter().enumerate() {
            let mut span_range_start = span.get_range().start;
            let attrs = &attrs_intervals.intervals[span.get_attrs_index()].val;
            let font_size = attrs.get_font_size();

            for token_variant in span.get_tokens() {
                match token_variant {
                    ShapeTokenVariant::TextFragment(token) => {
                        let token_width = token.x_advance().at(font_size);

                        // If adding this token exceeds line width,
                        // mark start of overflow without immediate line break.
                        // This avoids breaking within words across spans,
                        // setting up for cohesive word wrap.
                        if current_line_width + token_width > size.rwidth() {
                            overflown_span_ranges.push(SpanRange::new(
                                index,
                                span_range_start..token.get_range().end,
                            ));
                            span_range_start = token.get_range().end;
                            in_overflow = true;
                        }

                        current_word_width += token_width;
                        current_line_width += token_width;
                    }
                    ShapeTokenVariant::WordSeparator(token) => {
                        let separator_width = token.x_advance().at(font_size);

                        if in_overflow {
                            lines.push(LineToken::new(std::mem::take(&mut current_span_ranges)));
                            current_span_ranges.append(&mut overflown_span_ranges);
                            current_line_width = current_word_width;
                            current_word_width = Abs::pt(0.0);
                            in_overflow = false;
                        }

                        current_line_width += separator_width;
                    }
                    ShapeTokenVariant::Linebreak(token) => {
                        current_span_ranges.push(SpanRange::new(
                            index,
                            span_range_start..token.get_range().end,
                        ));
                        span_range_start = token.get_range().end;
                        lines.push(LineToken::new(std::mem::take(&mut current_span_ranges)));

                        if in_overflow {
                            current_span_ranges.append(&mut overflown_span_ranges);
                            current_line_width = current_word_width;
                            current_word_width = Abs::pt(0.0);
                            in_overflow = false;
                        } else {
                            current_line_width = Abs::pt(0.0);
                            current_word_width = Abs::pt(0.0);
                        }
                    }
                    _ => {}
                }
            }

            // Account for remaining span range
            if span_range_start < span.get_range().end {
                current_span_ranges.push(SpanRange::new(
                    index,
                    span_range_start..span.get_range().end,
                ));
            }
        }

        // Create new line from remaining span ranges
        if !current_span_ranges.is_empty() {
            lines.push(LineToken::new(current_span_ranges));
        }

        return lines;
    }
}
