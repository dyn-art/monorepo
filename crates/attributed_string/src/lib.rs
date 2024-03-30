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

use crate::outline::outline;
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

    pub fn tokenize_text(&mut self, fonts_book: &mut FontsBook) {
        self.divide_overlapping_spans();

        for (span, ..) in self.spans.iter_mut() {
            if span.is_dirty() {
                span.compute_tokens(&self.text, fonts_book);
            }
        }
    }

    pub fn divide_overlapping_spans(&mut self) {
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

    pub fn compute_lines(&self) -> Vec<Line> {
        let mut line_wrap_strategy: Box<dyn LineWrapStrategy> = match self.config.line_wrap {
            LineWrap::None => Box::new(NoLineWrap),
            LineWrap::Word => Box::new(WordWrap::new()),
            _ => Box::new(NoLineWrap),
        };
        return line_wrap_strategy.compute_lines(&self.spans, &self.config.size, &self.text);
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
                        glyph_token.transform = glyph_token
                            .transform
                            .pre_translate(curr_pos_x.to_pt(), curr_pos_y.to_pt());

                        curr_pos_x += glyph_token.x_advance;
                    }
                }
            }
        }

        self.lines = lines;
    }

    pub fn to_path(&self, fonts_book: &mut FontsBook) -> Option<tiny_skia_path::Path> {
        let mut text_builder = tiny_skia_path::PathBuilder::new();

        for Interval { val: span, .. } in self.spans.iter() {
            let mut span_builder = tiny_skia_path::PathBuilder::new();

            if let Some(font) = fonts_book.get_font_by_info(span.get_attrs().get_font_info()) {
                let font_size = span.get_attrs().get_font_size();

                for (cluster, _) in span.iter_glyph_clusters() {
                    let mut cluster_builder = tiny_skia_path::PathBuilder::new();
                    let mut width = Abs::zero();
                    let mut x = Abs::zero();

                    for glyph_token in cluster {
                        let sx = font.get_scale_factor(font_size);

                        if let Some(outline) = outline(glyph_token.get_glyph().glyph_id, &font) {
                            // By default, glyphs are upside-down, so we have to mirror them
                            let mut transform = tiny_skia_path::Transform::from_scale(1.0, -1.0);

                            // Scale to font-size
                            transform = transform.pre_scale(sx.to_pt(), sx.to_pt());

                            // Apply offset and transform.
                            //
                            // The first glyph in the cluster will have an offset from 0x0,
                            // but the later one will have an offset from the "current position".
                            // So we have to keep an advance.
                            transform.tx += (x + glyph_token.get_glyph().x_offset.at(font_size))
                                .to_pt()
                                + glyph_token.transform.tx;
                            transform.ty += glyph_token.get_glyph().y_offset.at(font_size).to_pt()
                                + glyph_token.transform.ty;

                            if let Some(outline) = outline.transform(transform) {
                                cluster_builder.push_path(&outline);
                            }
                        }

                        x += glyph_token.x_advance;

                        let glyph_width = glyph_token.x_advance;
                        if glyph_width > width {
                            width = glyph_width;
                        }
                    }

                    if let Some(path) = cluster_builder.finish() {
                        span_builder.push_path(&path);
                    }
                }
            }

            if let Some(path) = span_builder.finish() {
                text_builder.push_path(&path);
            }
        }

        // Draw bounding box
        // let rect_path = tiny_skia_path::PathBuilder::from_rect(
        //     tiny_skia_path::Rect::from_xywh(
        //         0.0,
        //         0.0,
        //         self.config.size.width(),
        //         self.config.size.height(),
        //     )
        //     .unwrap(),
        // );
        // let stroked_rect_path = tiny_skia_path::PathStroker::new()
        //     .stroke(
        //         &rect_path,
        //         &tiny_skia_path::Stroke {
        //             width: 1.0,
        //             ..Default::default()
        //         },
        //         1.0,
        //     )
        //     .unwrap();
        // text_builder.push_path(&stroked_rect_path);

        return text_builder.finish();
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

#[cfg(test)]
mod tests {
    use super::*;
    use dyn_fonts_book::font::{info::FontFamily, variant::FontWeight};
    use dyn_utils::units::{em::Em, font_unit::FontUnit};
    use unicode_bidi::BidiInfo;

    fn init() {
        let _ = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
            .is_test(true)
            .try_init();
    }

    #[test]
    fn e2e() {
        init();

        let mut fonts_book = FontsBook::new();
        fonts_book.load_system_fonts();

        let text = String::from("Hello, world!\nשלום עולם!\nThis is a mix of English and Hebrew.");
        let attrs_intervals = vec![
            AttrsInterval {
                start: 0,
                stop: 10,
                val: Attrs::new()
                    .font_family(FontFamily::Monospace)
                    .font_weight(FontWeight::REGULAR)
                    .font_size(Abs::pt(24.0))
                    .letter_spacing(FontUnit::em(Em::new(0.5))),
            },
            AttrsInterval {
                start: 10,
                stop: text.len(),
                val: Attrs::new()
                    .font_family(FontFamily::Serif)
                    .font_weight(FontWeight::REGULAR)
                    .font_size(Abs::pt(12.0)), // .word_spacing(FontUnit::em(Em::new(0.5))),
            },
        ];

        let mut attributed_string = AttributedString::new(
            text,
            attrs_intervals,
            AttributedStringConfig {
                size: Size::new(Abs::pt(150.0), Abs::pt(100.0)),
                line_wrap: LineWrap::Word,
                horizontal_text_alignment: HorizontalTextAlignment::Start,
                vertical_text_alignment: VerticalTextAlignment::Center,
            },
        );

        attributed_string.tokenize_text(&mut fonts_book);
        attributed_string.layout();
        let path = attributed_string.to_path(&mut fonts_book);

        // https://svg-path.com/
        log::info!("{:?}", path);

        assert_eq!(path.is_some(), true);
    }

    #[test]
    fn bidi_para_e2e() {
        // This example text is defined using `concat!` because some browsers
        // and text editors have trouble displaying bidi strings.
        let text = concat!["א", "ב", "ג", "a", "b", "c",];

        // Resolve embedding levels within the text.  Pass `None` to detect the
        // paragraph level automatically.
        let bidi_info = BidiInfo::new(&text, None);

        // This paragraph has embedding level 1 because its first strong character is RTL.
        assert_eq!(bidi_info.paragraphs.len(), 1);
        let para = &bidi_info.paragraphs[0];
        assert_eq!(para.level.number(), 1);
        assert_eq!(para.level.is_rtl(), true);

        // Re-ordering is done after wrapping each paragraph into a sequence of
        // lines. For this example, I'll just use a single line that spans the
        // entire paragraph.
        let line = para.range.clone();

        let display = bidi_info.reorder_line(para, line);
        assert_eq!(display, concat!["a", "b", "c", "ג", "ב", "א",]);
    }
}
