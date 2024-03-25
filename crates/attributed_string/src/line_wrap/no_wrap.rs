use super::LineWrapStrategy;
use crate::{
    attrs::AttrsIntervals,
    tokens::{
        line::{LineToken, SpanRange},
        shape::{ShapeToken, ShapeTokenVariant},
        span::SpanToken,
    },
};
use dyn_utils::properties::size::Size;

/// Line wrap strategy that only wraps if explicit Linebreak.
pub struct NoLineWrap;

impl LineWrapStrategy for NoLineWrap {
    fn compute_lines(
        &mut self,
        spans: &[SpanToken],
        _: &AttrsIntervals,
        _: &Size,
    ) -> Vec<LineToken> {
        let mut lines: Vec<LineToken> = Vec::new();
        let mut current_span_ranges: Vec<SpanRange> = Vec::new();

        for (index, span) in spans.iter().enumerate() {
            let mut span_range_start = span.get_range().start;

            for token_variant in span.get_tokens() {
                match token_variant {
                    ShapeTokenVariant::Linebreak(token) => {
                        current_span_ranges.push(SpanRange::new(
                            index,
                            span_range_start..token.get_range().end,
                        ));
                        span_range_start = token.get_range().end;
                        lines.push(LineToken::new(std::mem::take(&mut current_span_ranges)));
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
