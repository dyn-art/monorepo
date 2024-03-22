pub mod attrs;
pub mod font;
pub mod fonts_cache;
pub mod glyph;
pub mod path_builder;
pub mod tokens;

use crate::tokens::shape::ShapeToken;
use attrs::{Attrs, AttrsInterval, AttrsIntervals};
use fonts_cache::FontsCache;
use glam::Vec2;
use rust_lapper::Lapper;
use tokens::{layout::paragraph::ParagraphToken, span::SpanToken};

#[derive(Debug, Clone)]
struct AttributedString {
    text: String,
    shape_tokens: Vec<SpanToken>,
    layout_tokens: Vec<ParagraphToken>,
    attrs_intervals: AttrsIntervals,
    bbox: Vec2,
}

impl AttributedString {
    pub fn new(text: String, mut attrs_intervals: Vec<AttrsInterval>, bbox: Vec2) -> Self {
        if attrs_intervals.is_empty() {
            attrs_intervals.push(AttrsInterval {
                start: 0,
                stop: text.len(),
                val: Attrs::new(),
            });
        }

        return Self {
            text,
            shape_tokens: Vec::new(),
            layout_tokens: Vec::new(),
            attrs_intervals: Lapper::new(attrs_intervals),
            bbox,
        };
    }

    pub fn tokenize(&mut self, fonts_cache: &mut FontsCache) {
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

        self.shape_tokens = spans;
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
        // TODO: Layout tokens by createing lines, ..
    }

    pub fn to_path(&self) {
        // TODO
        for span in self.shape_tokens.iter() {
            for glyph in span.iter_glyphs() {
                log::info!("Glyph: {:?}", glyph.get_range());
            }
        }
    }
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

        let text = String::from("Hello, world!\nשלום עולם!\nThis is a mix of English and Hebrew.");
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

        let mut attributed_string =
            AttributedString::new(text, attrs_intervals, Vec2::new(100.0, 50.0));

        attributed_string.tokenize(&mut fonts_cache);
        attributed_string.layout();
        attributed_string.to_path();

        // println!("{:#?}", attributed_string);

        assert_eq!(attributed_string.shape_tokens.is_empty(), false);
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
