use crate::span::SpanIntervals;
use dyn_utils::units::{abs::Abs, Numeric};
use rust_lapper::Interval;
use std::ops::Range;

/// Represents a line of text.
#[derive(Debug, Clone)]
pub struct Line {
    // TODO: Right now its a vector of ranges
    // because some line wrap implementations want to ignore specific chars (e.g. spaces)
    // but can it be resolved with a more efficient and streamlined approach?
    ranges: Vec<Range<usize>>,
}

impl Line {
    pub fn new(ranges: Vec<Range<usize>>) -> Self {
        Self { ranges }
    }

    pub fn get_range(&self) -> Range<usize> {
        let start = self.ranges.first().map(|r| r.start).unwrap_or_default();
        let end = self.ranges.last().map(|r| r.end).unwrap_or_default();

        return start..end;
    }

    pub fn get_ranges(&self) -> &Vec<Range<usize>> {
        &self.ranges
    }

    pub fn get_direction(&self, spans: &SpanIntervals) -> LineDirection {
        let mut ltr_count = 0;
        let mut rtl_count = 0;

        for range in self.ranges.iter() {
            for Interval { val: span, .. } in spans.find(range.start, range.end) {
                let range_len = Range {
                    start: range.start.max(span.get_range().start),
                    end: range.end.min(span.get_range().end),
                }
                .len();
                if span.is_rtl() {
                    rtl_count += range_len;
                } else {
                    ltr_count += range_len;
                }
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
    // Should lines be allowed to do so?
    pub fn get_width(&self, spans: &SpanIntervals) -> Abs {
        let mut width = Abs::zero();

        // TODO: Trailing space should be removed in linewrap strategy?
        let mut current_space = Abs::zero();
        for range in self.ranges.iter() {
            for Interval { val: span, .. } in spans.find(range.start, range.end) {
                for token in span.iter_tokens_in_range(range) {
                    if token.is_blank() {
                        current_space += token.get_shape_token().get_width();
                    } else {
                        width += token.get_shape_token().get_width();
                        if !current_space.is_zero() {
                            width += current_space;
                            current_space = Abs::zero();
                        }
                    }
                }
            }
        }

        return width;
    }

    pub fn get_max_height(&self, spans: &SpanIntervals) -> Abs {
        let mut current_height = Abs::zero();

        for range in self.ranges.iter() {
            for Interval { val: span, .. } in spans.find(range.start, range.end) {
                for glyph_token in span.iter_glyphs_in_range(range) {
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
        }

        return current_height;
    }

    pub fn get_max_ascent(&self, spans: &SpanIntervals) -> Abs {
        let mut current_ascent = Abs::zero();

        for range in self.ranges.iter() {
            for Interval { val: span, .. } in spans.find(range.start, range.end) {
                for glyph_token in span.iter_glyphs_in_range(range) {
                    current_ascent = current_ascent.max(
                        glyph_token
                            .get_glyph()
                            .ascent
                            .at(span.get_attrs().get_font_size()),
                    );
                }
            }
        }

        return current_ascent;
    }

    /// Merges contiguous ranges within this line token.
    pub fn merge_contiguous_ranges(&mut self) {
        let mut merged: Vec<Range<usize>> = Vec::new();
        let mut last_range: Option<Range<usize>> = None;

        for range in self.ranges.iter() {
            match last_range.as_mut() {
                // If the current range continues from the last extend the range
                Some(last) if last.end == range.start => {
                    last.end = range.end;
                }
                // Otherwise, push the last range to the merged list and update last_range
                _ => {
                    if let Some(last) = last_range.take() {
                        merged.push(last);
                    }
                    last_range = Some(range.clone());
                }
            }
        }

        // Add the last range in the sequence
        if let Some(last) = last_range {
            merged.push(last);
        }

        self.ranges = merged;
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LineDirection {
    LeftToRight,
    RightToLeft,
}
