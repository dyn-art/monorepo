pub mod attrs;
pub mod font;
pub mod fonts_cache;
pub mod glyph;
pub mod glyph_clusters;
pub mod path_builder;
pub mod tokens;
pub mod utils;

use crate::tokens::shape::ShapeToken;
use attrs::{Attrs, AttrsInterval, AttrsIntervals};
use fonts_cache::FontsCache;
use glam::Vec2;
use rust_lapper::Lapper;
use tokens::{
    line::{LineToken, SpanRange},
    span::SpanToken,
};
use utils::is_range_within;

#[derive(Debug, Clone)]
struct AttributedString {
    text: String,
    spans: Vec<SpanToken>,
    lines: Vec<LineToken>,
    attrs_intervals: AttrsIntervals,
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

        return Self {
            text,
            spans: Vec::new(),
            lines: Vec::new(),
            attrs_intervals: Lapper::new(attrs_intervals),
            config,
        };
    }

    pub fn tokenize_text(&mut self, fonts_cache: &mut FontsCache) {
        self.devide_overlapping_attrs();

        let mut spans: Vec<SpanToken> = Vec::new();
        let bidi_info = unicode_bidi::BidiInfo::new(&self.text, None);

        // Determine spans
        for (index, attrs_interval) in self.attrs_intervals.iter().enumerate() {
            let mut span_start = attrs_interval.start;
            let mut current_bidi_level = bidi_info.levels[span_start];

            for i in attrs_interval.start..attrs_interval.stop {
                let char_bidi_level = bidi_info.levels[i];

                // When bidi level changes, create a new span for the previous segment
                if char_bidi_level != current_bidi_level {
                    spans.push(SpanToken::from_text(
                        &self.text,
                        span_start..i,
                        current_bidi_level,
                        index,
                        &attrs_interval.val,
                        fonts_cache,
                    ));

                    // Update for the new span
                    span_start = i;
                    current_bidi_level = char_bidi_level;
                }
            }

            // Ensure to add the last span in the current attribute range
            spans.push(SpanToken::from_text(
                &self.text,
                span_start..attrs_interval.stop,
                current_bidi_level,
                index,
                &attrs_interval.val,
                fonts_cache,
            ));
        }

        self.spans = spans;
    }

    pub fn devide_overlapping_attrs(&mut self) {
        self.attrs_intervals.divide_overlaps_with(|overlaps| {
            let mut merged_attrs = Attrs::new();
            for &attrs in overlaps.iter() {
                merged_attrs.merge(attrs.clone());
            }
            return merged_attrs;
        });
    }

    pub fn layout(&mut self) {
        let mut lines: Vec<LineToken> = Vec::new();

        match self.config.line_wrap {
            LineWrap::None => {
                let mut span_ranges: Vec<SpanRange> = Vec::new();
                for (index, span) in self.spans.iter().enumerate() {
                    span_ranges.push(SpanRange::from_span(index, &span));
                }
                lines.push(LineToken::new(span_ranges));
            }
            // TODO: Other line wrap implementations
            _ => {}
        }

        for line in lines.iter() {
            if line.get_span_ranges().is_empty() {
                continue;
            }
            let line_range = line.get_range();

            let mut pos = Vec2::new(0.0, 0.0);
            let mut max_ascent: f32 = 0.0;
            let mut max_descent: f32 = 0.0;

            for span_range in line.get_span_ranges().iter() {
                let span = &mut self.spans[span_range.index];
                let attrs = &self.attrs_intervals.intervals[span.get_attrs_index()].val;
                let font_size = attrs.get_font_size();

                for glyph_token in span.iter_glyphs_mut() {
                    if !is_range_within(glyph_token.get_range(), &line_range) {
                        continue;
                    }

                    let advance = glyph_token.get_glyph().advance * font_size;

                    glyph_token.set_transform(pos);

                    pos += advance;
                    max_ascent = max_ascent.max(glyph_token.get_glyph().ascent);
                    max_descent = max_descent.max(glyph_token.get_glyph().descent);
                }
            }
        }

        self.lines = lines;
    }

    pub fn to_path(&self, fonts_cache: &mut FontsCache) -> Option<tiny_skia_path::Path> {
        let mut text_builder = tiny_skia_path::PathBuilder::new();

        for span in self.spans.iter() {
            let attrs = &self.attrs_intervals.intervals[span.get_attrs_index()].val;
            let mut span_builder = tiny_skia_path::PathBuilder::new();

            if let Some(font) = fonts_cache.get_font_by_attrs(attrs) {
                let font_size = attrs.get_font_size();

                for (cluster, byte_index) in span.iter_glyph_clusters() {
                    let mut cluster_builder = tiny_skia_path::PathBuilder::new();
                    let mut width = 0.0;
                    let mut x: f32 = 0.0;

                    for glyph_token in cluster {
                        log::info!(
                            "Glyph: Range({:?}), {:?}, AttrsIndex({}), {:?}, ByteIndex({})",
                            glyph_token.get_range(),
                            span.get_level(),
                            span.get_attrs_index(),
                            glyph_token.get_transform(),
                            byte_index
                        );

                        let sx = font.scale(font_size);

                        if let Some(outline) = font.outline(glyph_token.get_glyph().glyph_id) {
                            // By default, glyphs are upside-down, so we have to mirror them
                            let mut transform = tiny_skia_path::Transform::from_scale(1.0, -1.0);

                            // Scale to font-size
                            transform = transform.pre_scale(sx, sx);

                            // Apply offset.
                            //
                            // The first glyph in the cluster will have an offset from 0x0,
                            // but the later one will have an offset from the "current position".
                            // So we have to keep an advance.
                            transform = transform.pre_translate(
                                x + glyph_token.get_glyph().offset.x
                                    + glyph_token.get_transform().x,
                                glyph_token.get_glyph().offset.y + glyph_token.get_transform().y,
                            );

                            if let Some(outline) = outline.transform(transform) {
                                cluster_builder.push_path(&outline);
                            }
                        }

                        x += glyph_token.get_glyph().advance.x;

                        let glyph_width = glyph_token.get_glyph().advance.x * sx;
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

        return text_builder.finish();
    }
}

#[derive(Debug, Default, Clone)]
struct AttributedStringConfig {
    pub bbox: Vec2,
    pub line_wrap: LineWrap,
}

#[derive(Debug, Default, Clone, Copy)]
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

#[cfg(test)]
mod tests {
    use self::attrs::FontFamily;
    use super::*;
    use unicode_bidi::BidiInfo;

    fn init() {
        let _ = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
            .is_test(true)
            .try_init();
    }

    #[test]
    fn e2e() {
        init();

        let mut fonts_cache = FontsCache::new();
        fonts_cache.load_system_fonts();

        // let text = String::from("Hello, world!\nשלום עולם!\nThis is a mix of English and Hebrew.");
        let text = String::from("Hello, world!");
        let attrs_intervals = vec![
            AttrsInterval {
                start: 0,
                stop: 10,
                val: Attrs::new()
                    .font_family(FontFamily::Monospace)
                    .font_weight(400)
                    .font_size(24.0),
            },
            AttrsInterval {
                start: 10,
                stop: text.len(),
                val: Attrs::new()
                    .font_family(FontFamily::Serif)
                    .font_weight(400)
                    .font_size(12.0),
            },
        ];

        let mut attributed_string = AttributedString::new(
            text,
            attrs_intervals,
            AttributedStringConfig {
                bbox: Vec2::new(100.0, 100.0),
                ..Default::default()
            },
        );

        attributed_string.tokenize_text(&mut fonts_cache);
        attributed_string.layout();
        let path = attributed_string.to_path(&mut fonts_cache);

        // https://yqnn.github.io/svg-path-editor/
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
