use crate::span::SpanIntervals;
use dyn_utils::units::{abs::Abs, em::Em};
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

    pub fn max_height(&self, spans: &SpanIntervals) -> Abs {
        let mut current_height = Abs::zero();

        for range in self.ranges.iter() {
            for Interval { val: span, .. } in spans.find(range.start, range.end) {
                for glyph_token in span.iter_glyphs_in_range(range) {
                    current_height = current_height.max(
                        glyph_token
                            .get_glyph()
                            .height()
                            .at(span.get_attrs().get_font_size()),
                    );
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
