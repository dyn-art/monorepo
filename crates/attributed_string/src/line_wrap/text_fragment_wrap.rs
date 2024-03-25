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
pub struct TextFragmentWrap;

impl LineWrapStrategy for TextFragmentWrap {
    fn compute_lines(
        &mut self,
        spans: &[SpanToken],
        attrs_intervals: &AttrsIntervals,
        size: &Size,
    ) -> Vec<LineToken> {
        let mut lines: Vec<LineToken> = Vec::new();

        let mut current_span_ranges: Vec<SpanRange> = Vec::new();
        let mut current_line_width: Abs = Abs::pt(0.0);

        for (index, span) in spans.iter().enumerate() {
            let mut span_range_start = span.get_range().start;
            let attrs = &attrs_intervals.intervals[span.get_attrs_index()].val;
            let font_size = attrs.get_font_size();

            for token_variant in span.get_tokens() {
                match token_variant {
                    ShapeTokenVariant::TextFragment(token) => {
                        let token_width = token.x_advance().at(font_size);

                        // If adding this token exceeds line width, break line
                        if current_line_width + token_width > size.rwidth() {
                            current_span_ranges.push(SpanRange::new(
                                index,
                                span_range_start..token.get_range().end,
                            ));
                            lines.push(LineToken::new(std::mem::take(&mut current_span_ranges)));
                            span_range_start = token.get_range().end;
                            current_line_width = Abs::pt(0.0);
                        }

                        current_line_width += token_width;
                    }
                    ShapeTokenVariant::WordSeparator(token) => {
                        current_line_width += token.x_advance().at(font_size);
                    }
                    ShapeTokenVariant::Glyph(token) => {
                        current_line_width += token.get_glyph().x_advance.at(font_size);
                    }
                    ShapeTokenVariant::Linebreak(token) => {
                        current_span_ranges.push(SpanRange::new(
                            index,
                            span_range_start..token.get_range().end,
                        ));
                        lines.push(LineToken::new(std::mem::take(&mut current_span_ranges)));
                        span_range_start = token.get_range().end;
                        current_line_width = Abs::pt(0.0);
                    }
                    _ => {}
                }
            }

            if span_range_start < span.get_range().end {
                current_span_ranges.push(SpanRange::new(
                    index,
                    span_range_start..span.get_range().end,
                ));
            }
        }

        if !current_span_ranges.is_empty() {
            lines.push(LineToken::new(current_span_ranges));
        }

        return lines;
    }
}
