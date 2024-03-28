use super::LineWrapStrategy;
use crate::{
    line::Line,
    shape_tokens::{ShapeToken, ShapeTokenVariant},
    span::SpanIntervals,
};
use dyn_utils::properties::size::Size;
use rust_lapper::Interval;
use std::ops::Range;

/// Line wrap strategy that only wraps if explicit Linebreak.
pub struct NoLineWrap;

impl LineWrapStrategy for NoLineWrap {
    fn compute_lines(&mut self, spans: &SpanIntervals, size: &Size, _: &str) -> Vec<Line> {
        let mut lines: Vec<Line> = Vec::new();
        let mut current_line_ranges: Vec<Range<usize>> = Vec::new();

        for (index, Interval { val: span, .. }) in spans.iter().enumerate() {
            let mut span_range_start = span.get_range().start;

            for token_variant in span.get_tokens() {
                match token_variant {
                    ShapeTokenVariant::Linebreak(token) => {
                        current_line_ranges.push(span_range_start..token.get_range().end);
                        span_range_start = token.get_range().end;
                        lines.push(Line::new(std::mem::take(&mut current_line_ranges)));
                    }
                    _ => {}
                }
            }

            // Check for any remaining part of the current span to be added
            if span_range_start < span.get_range().end {
                current_line_ranges.push(span_range_start..span.get_range().end);
            }
        }

        // Flush any remaining span ranges to a new line
        if !current_line_ranges.is_empty() {
            lines.push(Line::new(current_line_ranges));
        }

        return lines;
    }
}
