use super::{shape::ShapeToken, span::SpanToken};
use crate::{attrs::AttrsIntervals, utils::is_range_within};
use std::ops::Range;

/// Represents a line of text.
#[derive(Debug, Clone)]
pub struct LineToken {
    span_ranges: Vec<SpanRange>,
}

impl LineToken {
    pub fn new(mut span_ranges: Vec<SpanRange>) -> Self {
        sort_span_ranges(&mut span_ranges);
        Self { span_ranges }
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
    pub fn max_height(&self, spans: &[SpanToken], attrs_intervals: &AttrsIntervals) -> f32 {
        let line_range = self.get_range();
        let mut current_height: f32 = 0.0;

        for span_range in self.span_ranges.iter() {
            let span = &spans[span_range.index];
            let attrs = &attrs_intervals.intervals[span.get_attrs_index()].val;

            for glyph_token in span.iter_glyphs() {
                if !is_range_within(glyph_token.get_range(), &line_range) {
                    continue;
                }

                current_height = current_height.max(
                    glyph_token
                        .get_glyph()
                        .height()
                        .at(attrs.get_font_size())
                        .to_pt(),
                );
            }
        }

        return current_height;
    }

    // TODO: Improve
    pub fn max_ascent(&self, spans: &[SpanToken], attrs_intervals: &AttrsIntervals) -> f32 {
        let line_range = self.get_range();
        let mut current_height: f32 = 0.0;

        for span_range in self.span_ranges.iter() {
            let span = &spans[span_range.index];
            let attrs = &attrs_intervals.intervals[span.get_attrs_index()].val;

            for glyph_token in span.iter_glyphs() {
                if !is_range_within(glyph_token.get_range(), &line_range) {
                    continue;
                }

                current_height = current_height.max(
                    glyph_token
                        .get_glyph()
                        .ascent
                        .at(attrs.get_font_size())
                        .to_pt(),
                );
            }
        }

        return current_height;
    }

    /// Merges contiguous spans with the same index within this line token.
    pub fn merge_contiguous_spans(&mut self) {
        let mut merged: Vec<SpanRange> = Vec::new();
        let mut last_span: Option<SpanRange> = None;

        for span in self.span_ranges.iter() {
            match last_span.as_mut() {
                // If the current span continues from the last
                // and has the same index, extend the range
                Some(last) if last.index == span.index && last.range.end == span.range.start => {
                    last.range.end = span.range.end;
                }
                // Otherwise, push the last span to the merged list and update last_span
                _ => {
                    if let Some(last) = last_span.take() {
                        merged.push(last);
                    }
                    last_span = Some(span.clone());
                }
            }
        }

        // Add the last span in the sequence
        if let Some(last) = last_span {
            merged.push(last);
        }

        self.span_ranges = merged;
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
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
