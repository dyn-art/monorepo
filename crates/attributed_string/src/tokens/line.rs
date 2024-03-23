use super::{shape::ShapeToken, span::SpanToken};
use crate::{attrs::AttrsIntervals, utils::is_range_within};
use std::ops::Range;

/// Represents a line of text.
#[derive(Debug, Clone)]
pub struct LineToken {
    span_ranges: Vec<SpanRange>,
    max_ascent: f32,
    max_descent: f32,
}

impl LineToken {
    pub fn new(mut span_ranges: Vec<SpanRange>) -> Self {
        sort_span_ranges(&mut span_ranges);
        Self {
            span_ranges,
            max_ascent: 0.0,
            max_descent: 0.0,
        }
    }

    pub fn get_range(&self) -> Range<usize> {
        let start = self
            .span_ranges
            .first()
            .map(|sr| sr.range.start)
            .unwrap_or_default();
        let end = self
            .span_ranges
            .last()
            .map(|sr| sr.range.end)
            .unwrap_or_default();

        return start..end;
    }

    pub fn get_span_ranges(&self) -> &Vec<SpanRange> {
        &self.span_ranges
    }

    pub fn push_span_range(&mut self, span_range: SpanRange) {
        self.span_ranges.push(span_range);
        sort_span_ranges(&mut self.span_ranges);
    }

    // TODO: Improve
    pub fn height(&self, spans: &[SpanToken], attrs_intervals: &AttrsIntervals) -> f32 {
        let line_range = self.get_range();
        let mut current_height: f32 = 0.0;

        for span_range in self.span_ranges.iter() {
            let span = &spans[span_range.index];
            let attrs = &attrs_intervals.intervals[span.get_attrs_index()].val;

            for glyph_token in span.iter_glyphs() {
                if !is_range_within(glyph_token.get_range(), &line_range) {
                    continue;
                }

                current_height =
                    current_height.max(glyph_token.get_glyph().height() * attrs.get_font_size());
            }
        }

        return current_height;
    }
}

#[derive(Debug, Clone)]
pub struct SpanRange {
    pub index: usize,
    pub range: Range<usize>,
}

impl SpanRange {
    pub fn new(index: usize, range: Range<usize>) -> Self {
        Self { index, range }
    }
}

pub fn sort_span_ranges(span_ranges: &mut Vec<SpanRange>) {
    span_ranges.sort_by(|a, b| {
        // First, compare by index
        a.index
            .cmp(&b.index)
            // If indices are equal, compare by the ending point of the range
            .then_with(|| a.range.end.cmp(&b.range.end))
    });
}
