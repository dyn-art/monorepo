use super::{
    layout_config::{HorizontalTextAlignment, LayoutConfig, LineWrap, VerticalTextAlignment},
    line::{Line, LineDirection},
    line_wrap::{no_wrap::NoLineWrap, word_wrap::WordWrap, LineWrapStrategy},
};
use crate::span::SpanIntervals;
use dyn_utils::{
    properties::size::Size,
    units::{abs::Abs, auto_length::AutoLength},
};

#[derive(Debug, Clone)]
pub struct LayoutHandler {
    lines: Vec<Line>,
    config: LayoutConfig,
}

impl LayoutHandler {
    pub fn new(config: LayoutConfig) -> Self {
        Self {
            lines: Vec::new(),
            config,
        }
    }

    pub fn layout(&mut self, spans: &mut SpanIntervals) {
        for (span, ..) in spans.iter_mut() {
            span.apply_letter_spacing();
            span.apply_word_spacing();
        }
        self.layout_lines(spans);
    }

    pub fn layout_lines(&mut self, spans: &mut SpanIntervals) {
        self.lines = self.compute_lines(spans);

        let text_size = self.compute_text_size(&spans);
        let container_size = self.compute_container_size(&text_size);

        let vertical_alignment_correction = match self.config.vertical_text_alignment {
            VerticalTextAlignment::Top => Abs::zero(),
            VerticalTextAlignment::Bottom => container_size.height - text_size.height,
            VerticalTextAlignment::Center => (container_size.height - text_size.height) / 2.0,
        };

        let mut curr_pos_x: Abs;
        let mut curr_pos_y = vertical_alignment_correction;

        // Layout tokens based on lines
        for (index, line) in self.lines.iter().enumerate() {
            if line.get_ranges().is_empty() {
                continue;
            }

            let line_direction = line.get_direction(&spans);
            let line_width = line.get_width(&spans);

            let horizontal_alignment_correction =
                match (self.config.horizontal_text_alignment, line_direction) {
                    (HorizontalTextAlignment::Left, _) => Abs::zero(),
                    (HorizontalTextAlignment::Right, _) => container_size.width - line_width,
                    (HorizontalTextAlignment::Center, _) => {
                        (container_size.width - line_width) / 2.0
                    }
                    (HorizontalTextAlignment::Start, LineDirection::LeftToRight) => Abs::zero(),
                    (HorizontalTextAlignment::End, LineDirection::LeftToRight) => {
                        container_size.width - line_width
                    }
                    (HorizontalTextAlignment::Start, LineDirection::RightToLeft) => {
                        container_size.width - line_width
                    }
                    (HorizontalTextAlignment::End, LineDirection::RightToLeft) => Abs::zero(),
                };

            curr_pos_x = horizontal_alignment_correction;
            curr_pos_y += if index == 0 {
                line.get_max_ascent(&spans)
            } else {
                line.get_max_height(&spans)
            };

            for range in line.get_ranges().iter() {
                for (span, ..) in spans.find_mut(range.start, range.end) {
                    for glyph_token in span.iter_glyphs_in_range_mut(&range) {
                        glyph_token.layout.transform = tiny_skia_path::Transform::from_translate(
                            curr_pos_x.to_pt(),
                            curr_pos_y.to_pt(),
                        );

                        curr_pos_x += glyph_token.layout.x_advance;
                    }
                }
            }
        }
    }

    fn compute_lines(&self, spans: &SpanIntervals) -> Vec<Line> {
        let mut size = Size::zero();

        let mut line_wrap_strategy: Box<dyn LineWrapStrategy> = match (
            self.config.size.x,
            self.config.size.y,
            self.config.line_wrap,
        ) {
            (_, _, LineWrap::None) => Box::new(NoLineWrap),
            (AutoLength::Abs { value: x }, AutoLength::Abs { value: y }, LineWrap::Word) => {
                size.width = x;
                size.height = y;
                Box::new(WordWrap::new())
            }
            (AutoLength::Abs { value: x }, _, LineWrap::Word) => {
                size.width = x;
                Box::new(WordWrap::new())
            }
            _ => Box::new(NoLineWrap),
        };
        return line_wrap_strategy.compute_lines(spans, &size);
    }

    pub fn compute_text_size(&self, spans: &SpanIntervals) -> Size {
        Size::new(
            self.lines
                .iter()
                .fold(Abs::zero(), |acc, line| acc.max(line.get_width(&spans))),
            self.lines
                .iter()
                .enumerate()
                .fold(Abs::zero(), |acc, (index, line)| {
                    if index == 0 {
                        acc + line.get_max_ascent(&spans)
                    } else {
                        acc + line.get_max_height(&spans)
                    }
                }),
        )
    }

    pub fn compute_container_size(&self, text_size: &Size) -> Size {
        Size::new(
            match self.config.size.x {
                AutoLength::Abs { value } => value,
                AutoLength::Ratio { value } => value.of(text_size.width),
                AutoLength::Auto => text_size.width,
            },
            match self.config.size.y {
                AutoLength::Abs { value } => value,
                AutoLength::Ratio { value } => value.of(text_size.height),
                AutoLength::Auto => text_size.height,
            },
        )
    }
}
