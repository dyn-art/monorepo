pub mod token;
pub mod usvg;

use ordered_float::OrderedFloat;
use rust_lapper::{Interval, Lapper};
use smallvec::SmallVec;
use std::{ops::Range, sync::Arc};
use token::{Token, TokenVariant};
use usvg::{
    database::FontsCache,
    glyph::{Glyph, GlyphClusters},
    outline_cluster, resolve_font,
    resolved_font::ResolvedFont,
    shape_text,
    text::{AlignmentBaseline, BaselineShift, DominantBaseline, Font, LengthAdjust},
};

#[derive(Clone)]
struct AttributedString {
    text: String,
    token_stream: Vec<Token>,
    attribute_intervals: Lapper<usize, Attribute>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Attribute {
    /// A font.
    pub font: Font,
    /// A font size.
    pub font_size: OrderedFloat<f32>,
    /// Indicates that small caps should be used.
    ///
    /// Set by `font-variant="small-caps"`
    pub small_caps: bool,
    /// Indicates that a kerning should be applied.
    ///
    /// Supports both `kerning` and `font-kerning` properties.
    pub apply_kerning: bool,
    /// A span dominant baseline.
    pub dominant_baseline: DominantBaseline,
    /// A span alignment baseline.
    pub alignment_baseline: AlignmentBaseline,
    /// A list of all baseline shift that should be applied to this span.
    ///
    /// Ordered from `text` element down to the actual `span` element.
    pub baseline_shift: Vec<BaselineShift>,
    /// A letter spacing property.
    pub letter_spacing: OrderedFloat<f32>,
    /// A word spacing property.
    pub word_spacing: OrderedFloat<f32>,
    /// A text length property.
    pub text_length: Option<OrderedFloat<f32>>,
    /// A length adjust property.
    pub length_adjust: LengthAdjust,
}

type AttributeInterval = Interval<usize, Attribute>;

impl AttributedString {
    pub fn new(text: String, attribute_intervals: Vec<AttributeInterval>) -> Self {
        Self {
            text,
            token_stream: Vec::new(),
            attribute_intervals: Lapper::new(attribute_intervals),
        }
    }

    pub fn tokanize(&mut self) {
        let mut token_stream = Vec::new();

        // Tokenize the text, considering spaces and line breaks
        let mut start = 0;
        for (index, match_str) in self
            .text
            .match_indices(|c: char| is_word_separator_char(c) || is_linebreak_char(c))
        {
            // Create a text fragment token for non-whitespace segments
            if start != index {
                token_stream.push(Token {
                    variant: TokenVariant::TextFragment,
                    range: Range { start, end: index },
                    outlined_clusters: SmallVec::new(),
                })
            }

            // Create a token for each space or line break
            token_stream.push(match match_str.chars().next() {
                Some(c) if is_word_separator_char(c) => Token {
                    variant: TokenVariant::WordSeparator,
                    range: Range {
                        start: index,
                        end: index + match_str.len(),
                    },
                    outlined_clusters: SmallVec::new(),
                },
                Some(c) if is_linebreak_char(c) => Token {
                    variant: TokenVariant::Linebreak,
                    range: Range {
                        start: index,
                        end: index + match_str.len(),
                    },
                    outlined_clusters: SmallVec::new(),
                },
                _ => continue, // Should never happen
            });

            start = index + match_str.len();
        }

        // Handle the last text fragment in the segment, if any
        if start < self.text.len() {
            token_stream.push(Token {
                variant: TokenVariant::TextFragment,
                range: Range {
                    start,
                    end: self.text.len(),
                },
                outlined_clusters: SmallVec::new(),
            });
        }

        self.token_stream = token_stream;
    }

    pub fn outline(&mut self, fonts_cache: &mut FontsCache, fontdb: &fontdb::Database) {
        if !self.attribute_intervals.overlaps_merged {
            self.attribute_intervals.merge_overlaps();
        }

        // TODO: Are inter-glyph covered by TextFragment's
        // or do they go beyond line breaks, word separator?
        for token in &mut self.token_stream {
            let mut glyphs: Vec<Option<Glyph>> = vec![None; token.range.end - token.range.start];

            // Outline token and thus create glyphs based on attributes
            for Interval { start, stop, val } in self
                .attribute_intervals
                .find(token.range.start, token.range.end)
            {
                let resolved_font = match Self::resolve_font(&val.font, fonts_cache, fontdb) {
                    Some(v) => v.clone(),
                    None => continue,
                };

                let interval_glyphs = shape_text(
                    &self.text[token.range.start.max(*start)..token.range.end.min(*stop)],
                    resolved_font,
                    val.small_caps,
                    val.apply_kerning,
                    fontdb,
                );

                // Add interval_glyphs to glyphs vector at start to stop index
                for (index, glyph) in interval_glyphs.into_iter().enumerate() {
                    let global_index = start - token.range.start + index;
                    glyphs[global_index] = Some(glyph);
                }
            }

            // Validate glyphs
            let maybe_glyphs_len = glyphs.len();
            let glyphs: Vec<Glyph> = glyphs.into_iter().filter_map(|glyph| glyph).collect();
            if glyphs.is_empty() || glyphs.len() != maybe_glyphs_len {
                continue;
            }

            // Convert glyphs to outlined glyph clusters
            for (range, byte_idx) in GlyphClusters::new(&glyphs) {
                let interval_index = token.range.start + byte_idx.value();
                let maybe_interval = self
                    .attribute_intervals
                    .find(interval_index, interval_index)
                    .last();
                if let Some(interval) = maybe_interval {
                    token.outlined_clusters.push(outline_cluster(
                        &glyphs[range],
                        &self.text[token.range.clone()],
                        interval.val.font_size.0,
                        fontdb,
                    ));
                }
            }
        }
    }

    fn resolve_font<'a>(
        font: &Font,
        fonts_cache: &'a mut FontsCache,
        fontdb: &fontdb::Database,
    ) -> Option<&'a Arc<ResolvedFont>> {
        // Check if the font is already in the cache
        if !fonts_cache.contains_key(font) {
            if let Some(resolved_font) = resolve_font(font, fontdb) {
                fonts_cache.insert(font.clone(), Arc::new(resolved_font));
            } else {
                return None;
            }
        }

        return fonts_cache.get(font);
    }

    pub fn to_paths(&mut self) -> Vec<()> {
        // TODO
        Vec::new()
    }
}

// https://www.w3.org/TR/css-text-3/#word-separator
pub fn is_word_separator_char(c: char) -> bool {
    matches!(
        c as u32,
        0x0020 | 0x00A0 | 0x1361 | 0x010100 | 0x010101 | 0x01039F | 0x01091F
    )
}

pub fn is_linebreak_char(c: char) -> bool {
    matches!(c, '\n')
}

#[cfg(test)]
mod tests {
    use self::usvg::text::{FontFamily, FontStretch, FontStyle};
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn attributed_string_e2e() {
        let fontdb = get_fontdb_with_system_fonts();
        let mut fonts_cache: FontsCache = HashMap::new();

        let text = String::from("Hello, world! This is a test!");
        let attribute_intervals = vec![AttributeInterval {
            start: 0,
            stop: text.len(),
            val: Attribute {
                font: Font {
                    families: vec![FontFamily::Serif],
                    stretch: FontStretch::default(),
                    style: FontStyle::default(),
                    weight: 10,
                },
                font_size: OrderedFloat(12.0),
                small_caps: false,
                apply_kerning: true,
                dominant_baseline: DominantBaseline::Alphabetic,
                alignment_baseline: AlignmentBaseline::Baseline,
                baseline_shift: vec![],
                letter_spacing: OrderedFloat(0.0),
                word_spacing: OrderedFloat(0.0),
                text_length: None,
                length_adjust: LengthAdjust::SpacingAndGlyphs,
            },
        }];
        let mut attributed_string = AttributedString::new(text, attribute_intervals);

        attributed_string.tokanize();

        attributed_string.outline(&mut fonts_cache, &fontdb);

        assert!(
            !attributed_string.token_stream.is_empty(),
            "Token stream should not be empty after processing."
        );
    }

    fn get_fontdb_with_system_fonts() -> fontdb::Database {
        let mut db = fontdb::Database::new();
        db.load_system_fonts();
        db.set_serif_family("Times New Roman");
        db.set_sans_serif_family("Arial");
        db.set_cursive_family("Comic Sans MS");
        db.set_fantasy_family("Impact");
        db.set_monospace_family("Courier New");
        return db;
    }
}
