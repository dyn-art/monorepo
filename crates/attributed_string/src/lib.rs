pub mod glyph;
pub mod glyph_clusters;
pub mod layout;
pub mod outline;
pub mod script;
pub mod shape;
pub mod shape_tokens;
pub mod span;
pub mod text_attrs;
pub mod utils;

pub use dyn_fonts_book;
use dyn_fonts_book::FontsBook;
use rust_lapper::{Interval, Lapper};
use span::{Span, SpanIntervals};
use text_attrs::{TextAttrs, TextAttrsInterval};

#[derive(Debug, Clone)]
pub struct AttributedString {
    text: String,
    spans: SpanIntervals,
}

impl AttributedString {
    pub fn new(text: String, attrs_intervals: Vec<TextAttrsInterval>) -> Self {
        let text_len = text.len();
        let bidi_info = unicode_bidi::BidiInfo::new(&text, None);
        let span_intervals = attrs_intervals
            .iter()
            .filter_map(|interval| {
                if  interval.stop <= text_len {
                    Some(Span::new(interval.start..interval.stop, interval.val.clone()))
                } else {
                    log::warn!("Attribute interval from {} to {} was dropped because it was not in the provided text boundaries!", interval.start, interval.stop);
                    None
                }
            })
            .flat_map(|span| span.divide_at_bidi_level(&bidi_info))
            .map(|span| Interval {
                start: span.get_range().start,
                stop: span.get_range().end,
                val: span,
            })
            .collect::<Vec<_>>();

        return Self {
            text,
            spans: Lapper::new(span_intervals),
        };
    }

    pub fn get_spans(&self) -> &SpanIntervals {
        &self.spans
    }

    pub fn get_spans_mut(&mut self) -> &mut SpanIntervals {
        &mut self.spans
    }

    pub fn tokenize_text(&mut self, fonts_book: &mut FontsBook) {
        self.divide_overlapping_spans();

        for (span, ..) in self.spans.iter_mut() {
            if span.is_dirty() {
                span.compute_tokens(&self.text, fonts_book);
            }
        }
    }

    fn divide_overlapping_spans(&mut self) {
        if !self.spans.overlaps_merged {
            self.spans
                .divide_overlaps_with(|overlaps, range| match overlaps.len() {
                    0 => panic!("Failed to devide overlapping spans!"), // Should never happen
                    1 => {
                        let mut overlap = overlaps[0].clone();
                        if overlap.get_range().clone() != range {
                            overlap.mark_dirty();
                        }

                        return overlap;
                    }
                    _ => {
                        let mut merged_attrs = TextAttrs::new();
                        for &span in overlaps.iter() {
                            merged_attrs.merge(span.get_attrs().clone());
                        }

                        return Span::new(range, merged_attrs);
                    }
                });
        }
    }

    pub fn adjust_intervals(attrs_intervals: &mut Vec<TextAttrsInterval>, text: &str) {
        if attrs_intervals.is_empty() {
            attrs_intervals.push(TextAttrsInterval {
                start: 0,
                stop: text.len(),
                val: TextAttrs::new(),
            });
            return;
        }

        let text_len = text.len();

        let mut largest = None;
        let mut largest_stop = 0;

        // Adjust intervals where 'stop' exceeds the text length
        // and find the interval with the largest 'stop'
        for interval in attrs_intervals.iter_mut() {
            if interval.stop > text_len {
                interval.stop = text_len;
            }

            if interval.stop > largest_stop {
                largest_stop = interval.stop;
                largest = Some(interval);
            }
        }

        // If no interval exactly matches the text length,
        // extend the longest interval to match the text length
        if largest_stop < text_len {
            if let Some(longest) = largest {
                longest.stop = text_len;
            }
        }
    }
}
