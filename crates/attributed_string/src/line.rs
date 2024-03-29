use crate::span::SpanIntervals;
use dyn_utils::units::abs::Abs;
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

    pub fn direction(&self, spans: &SpanIntervals) -> LineDirection {
        let mut ltr_count = 0;
        let mut rtl_count = 0;

        for range in self.ranges.iter() {
            for interval in spans.find(range.start, range.end) {
                if interval.val.get_bidi_level().number() % 2 == 0 {
                    ltr_count += 1;
                } else {
                    rtl_count += 1;
                }
            }
        }

        // Determine direction based on the counts
        if ltr_count > 0 && rtl_count > 0 {
            LineDirection::Mixed
        } else if rtl_count > 0 {
            LineDirection::RightToLeft
        } else {
            LineDirection::LeftToRight
        }
    }

    pub fn width(&self, spans: &SpanIntervals) -> Abs {
        let mut width = Abs::zero();

        for range in self.ranges.iter() {
            for Interval { val: span, .. } in spans.find(range.start, range.end) {
                width += span.width();
            }
        }

        return width;
    }

    pub fn max_height(&self, spans: &SpanIntervals) -> Abs {
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

    pub fn max_ascent(&self, spans: &SpanIntervals) -> Abs {
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

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum LineDirection {
    #[default]
    Mixed,
    LeftToRight,
    RightToLeft,
}
