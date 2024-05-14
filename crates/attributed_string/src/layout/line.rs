use crate::span::SpanIntervals;
use dyn_utils::units::{abs::Abs, Numeric};
use rust_lapper::Interval;
use std::ops::Range;

/// Represents a line of text.
#[derive(Debug, Clone)]
pub struct Line {
    range: Range<usize>,
}

impl Line {
    pub fn new(range: Range<usize>) -> Self {
        Self { range }
    }

    pub fn from_ranges(ranges: Vec<Range<usize>>) -> Self {
        let start = ranges.first().map(|r| r.start).unwrap_or_default();
        let end = ranges.last().map(|r| r.end).unwrap_or_default();

        return Self { range: start..end };
    }

    pub fn get_range(&self) -> &Range<usize> {
        &self.range
    }

    pub fn get_direction(&self, spans: &SpanIntervals) -> LineDirection {
        let mut ltr_count = 0;
        let mut rtl_count = 0;

        for Interval { val: span, .. } in spans.find(self.range.start, self.range.end) {
            let range_len = Range {
                start: self.range.start.max(span.get_range().start),
                end: self.range.end.min(span.get_range().end),
            }
            .len();
            if span.is_rtl() {
                rtl_count += range_len;
            } else {
                ltr_count += range_len;
            }
        }

        // Determine direction based on the counts
        return if ltr_count >= rtl_count {
            LineDirection::LeftToRight
        } else {
            LineDirection::RightToLeft
        };
    }

    // TODO: Improve, what if Line spans across tokens?
    // Should lines be allowed to do so? Or should the token be split?
    pub fn get_x_advance(&self, spans: &SpanIntervals) -> Abs {
        let mut width = Abs::zero();

        // TODO: Trailing space should be removed in linewrap strategy?
        let mut current_space = Abs::zero();
        for Interval { val: span, .. } in spans.find(self.range.start, self.range.end) {
            for token in span.iter_tokens_in_range(&self.range) {
                if token.is_blank() {
                    current_space += token.get_shape_token().x_advance();
                } else {
                    width += token.get_shape_token().x_advance();
                    if !current_space.is_zero() {
                        width += current_space;
                        current_space = Abs::zero();
                    }
                }
            }
        }

        return width;
    }

    pub fn get_max_height(&self, spans: &SpanIntervals) -> Abs {
        let mut current_height = Abs::zero();

        for Interval { val: span, .. } in spans.find(self.range.start, self.range.end) {
            for glyph_token in span.iter_glyphs_in_range(&self.range) {
                let height = span
                    .get_attrs()
                    .get_line_height()
                    .map(|h| h.at(span.get_attrs().get_font_size()))
                    .unwrap_or(
                        glyph_token
                            .get_glyph()
                            .height()
                            .at(span.get_attrs().get_font_size()),
                    );
                current_height = current_height.max(height);
            }
        }

        return current_height;
    }

    pub fn get_max_ascent(&self, spans: &SpanIntervals) -> Abs {
        let mut current_ascent = Abs::zero();

        for Interval { val: span, .. } in spans.find(self.range.start, self.range.end) {
            for glyph_token in span.iter_glyphs_in_range(&self.range) {
                current_ascent = current_ascent.max(
                    glyph_token
                        .get_glyph()
                        .ascent
                        .at(span.get_attrs().get_font_size()),
                );
            }
        }

        return current_ascent;
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LineDirection {
    LeftToRight,
    RightToLeft,
}
