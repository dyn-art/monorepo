use std::ops::Range;

use super::span::SpanToken;

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

    pub fn from_span(index: usize, span: &SpanToken) -> Self {
        Self {
            index,
            range: span.get_range().clone(),
        }
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
