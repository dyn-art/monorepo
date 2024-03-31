pub mod attrs;
pub mod glyph;
pub mod glyph_clusters;
pub mod line;
pub mod line_wrap;
pub mod outline;
pub mod script;
pub mod shape;
pub mod shape_tokens;
pub mod span;
pub mod utils;

use attrs::{Attrs, AttrsInterval};
pub use dyn_fonts_book;
use dyn_fonts_book::FontsBook;
use dyn_utils::{properties::size::Size, units::abs::Abs};
use line::{Line, LineDirection};
use line_wrap::{no_wrap::NoLineWrap, word_wrap::WordWrap, LineWrapStrategy};
use rust_lapper::{Interval, Lapper};
use span::{Span, SpanIntervals};

#[derive(Debug, Clone)]
pub struct AttributedString {
    text: String,
    spans: SpanIntervals,
    lines: Vec<Line>,
    config: AttributedStringConfig,
}

impl AttributedString {
    pub fn new(
        text: String,
        mut attrs_intervals: Vec<AttrsInterval>,
        config: AttributedStringConfig,
    ) -> Self {
        if attrs_intervals.is_empty() {
            attrs_intervals.push(AttrsInterval {
                start: 0,
                stop: text.len(),
                val: Attrs::new(),
            });
        }

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
            lines: Vec::new(),
            config,
        };
    }

    pub fn get_spans(&self) -> &SpanIntervals {
        &self.spans
    }

    pub fn get_size(&self) -> &Size {
        &self.config.size
    }

    pub fn tokenize_text(&mut self, fonts_book: &mut FontsBook) {
        self.divide_overlapping_spans();

        for (span, ..) in self.spans.iter_mut() {
            if span.is_dirty() {
                span.compute_tokens(&self.text, fonts_book);
            }
        }
    }

    pub fn apply_size(&mut self, size: Size) {
        self.config.size = size;
        self.layout_lines();
    }

    pub fn layout(&mut self) {
        for (span, ..) in self.spans.iter_mut() {
            span.apply_letter_spacing();
            span.apply_word_spacing();
        }
        self.layout_lines();
    }

    pub fn layout_lines(&mut self) {
        let lines = self.compute_lines();

        let container_width = self.config.size.rwidth();
        let container_height = self.config.size.rheight();

        let total_text_height = lines
            .iter()
            .enumerate()
            .fold(Abs::zero(), |acc, (index, line)| {
                if index == 0 {
                    acc + line.get_max_ascent(&self.spans)
                } else {
                    acc + line.get_max_height(&self.spans)
                }
            });
        let vertical_alignment_correction = match self.config.vertical_text_alignment {
            VerticalTextAlignment::Top => Abs::zero(),
            VerticalTextAlignment::Bottom => container_height - total_text_height,
            VerticalTextAlignment::Center => (container_height - total_text_height) / 2.0,
        };

        let mut curr_pos_x: Abs;
        let mut curr_pos_y = vertical_alignment_correction;

        // Layout tokens based on lines
        for (index, line) in lines.iter().enumerate() {
            if line.get_ranges().is_empty() {
                continue;
            }

            let line_direction = line.get_direction(&self.spans);
            let line_width = line.get_width(&self.spans);

            let horizontal_alignment_correction =
                match (self.config.horizontal_text_alignment, line_direction) {
                    (HorizontalTextAlignment::Left, _) => Abs::zero(),
                    (HorizontalTextAlignment::Right, _) => container_width - line_width,
                    (HorizontalTextAlignment::Center, _) => (container_width - line_width) / 2.0,
                    (HorizontalTextAlignment::Start, LineDirection::LeftToRight) => Abs::zero(),
                    (HorizontalTextAlignment::End, LineDirection::LeftToRight) => {
                        container_width - line_width
                    }
                    (HorizontalTextAlignment::Start, LineDirection::RightToLeft) => {
                        container_width - line_width
                    }
                    (HorizontalTextAlignment::End, LineDirection::RightToLeft) => Abs::zero(),
                };

            curr_pos_x = horizontal_alignment_correction;
            curr_pos_y += if index == 0 {
                line.get_max_ascent(&self.spans)
            } else {
                line.get_max_height(&self.spans)
            };

            for range in line.get_ranges().iter() {
                for (span, ..) in self.spans.find_mut(range.start, range.end) {
                    for glyph_token in span.iter_glyphs_in_range_mut(&range) {
                        glyph_token.transform = tiny_skia_path::Transform::from_translate(
                            curr_pos_x.to_pt(),
                            curr_pos_y.to_pt(),
                        );

                        curr_pos_x += glyph_token.x_advance;
                    }
                }
            }
        }

        self.lines = lines;
    }

    pub fn compute_lines(&self) -> Vec<Line> {
        let mut line_wrap_strategy: Box<dyn LineWrapStrategy> = match self.config.line_wrap {
            LineWrap::None => Box::new(NoLineWrap),
            LineWrap::Word => Box::new(WordWrap::new()),
            _ => Box::new(NoLineWrap),
        };
        return line_wrap_strategy.compute_lines(&self.spans, &self.config.size, &self.text);
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
                        let mut merged_attrs = Attrs::new();
                        for &span in overlaps.iter() {
                            merged_attrs.merge(span.get_attrs().clone());
                        }

                        return Span::new(range, merged_attrs);
                    }
                });
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct AttributedStringConfig {
    pub size: Size,
    pub line_wrap: LineWrap,
    pub horizontal_text_alignment: HorizontalTextAlignment,
    pub vertical_text_alignment: VerticalTextAlignment,
}

#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum LineWrap {
    /// No wrapping
    #[default]
    None,
    /// Wraps at a glyph level
    Glyph,
    /// Wraps at the word level
    Word,
    /// Wraps at the word level, or fallback to glyph level if a word can't fit on a line by itself
    WordOrGlyph,
}

#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum HorizontalTextAlignment {
    #[default]
    Start,
    End,
    Left,
    Right,
    Center,
    // Justified, // TODO
}

#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum VerticalTextAlignment {
    #[default]
    Top,
    Bottom,
    Center,
    // Justified, // TODO
}
