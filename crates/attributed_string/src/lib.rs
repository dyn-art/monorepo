pub mod attribute;
pub mod font;
pub mod token;
pub mod usvg;

use attribute::{Attribute, AttributeInterval};
use rust_lapper::Lapper;
use std::ops::Range;
use tiny_skia_path::{Path, Transform};
use token::{Token, TokenVariant};
use usvg::{
    database::FontsCache,
    process_anchor,
    text::{TextAnchor, TextFlow, WritingMode},
};

/// `AttributedString` represents a string with associated attributes applied
/// to certain ranges.
#[derive(Clone)]
struct AttributedString {
    /// The full text as a `String` without any attribute information.
    pub text: String,

    /// A list of tokens derived from the text.
    ///
    /// Each `Token` represents a semantically meaningful unit of the text, such as a `TextFragment`, `WordSeparator` or `Linebreak`,
    /// facilitating further processing.
    pub token_stream: Vec<Token>,

    /// Attribute intervals mapped to text ranges.
    pub attribute_intervals: Lapper<usize, Attribute>,

    /// Defines the anchoring position of the text relative to its container.
    pub anchor: TextAnchor,

    /// Describes how text is divided and flowed within a layout.
    pub text_flow: TextFlow,

    /// Specifies the writing mode for the text.
    pub writing_mode: WritingMode,
}

impl AttributedString {
    pub fn new(text: String, attribute_intervals: Vec<AttributeInterval>) -> Self {
        Self {
            text,
            token_stream: Vec::new(),
            attribute_intervals: Lapper::new(attribute_intervals),
            anchor: TextAnchor::Start,
            text_flow: TextFlow::Linear,
            writing_mode: WritingMode::LeftToRight,
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
                token_stream.push(Token::new(
                    TokenVariant::TextFragment,
                    Range { start, end: index },
                ));
            }

            // Create a token for each space or line break
            token_stream.push(match match_str.chars().next() {
                Some(c) if is_word_separator_char(c) => Token::new(
                    TokenVariant::WordSeparator,
                    Range {
                        start: index,
                        end: index + match_str.len(),
                    },
                ),
                Some(c) if is_linebreak_char(c) => Token::new(
                    TokenVariant::Linebreak,
                    Range {
                        start: index,
                        end: index + match_str.len(),
                    },
                ),
                _ => continue, // Should never happen
            });

            start = index + match_str.len();
        }

        // Handle the last text fragment in the segment, if any
        if start < self.text.len() {
            token_stream.push(Token::new(
                TokenVariant::TextFragment,
                Range {
                    start,
                    end: self.text.len(),
                },
            ));
        }

        self.token_stream = token_stream;
    }

    pub fn shape_glyphs(&mut self, fonts_cache: &mut FontsCache, fontdb: &fontdb::Database) {
        // TODO: Are inter-glyph covered by TextFragment's
        // or do they go beyond line breaks, word separator?
        for token in &mut self.token_stream {
            token.shape_glyphs(
                &self.text,
                &mut self.attribute_intervals,
                fonts_cache,
                fontdb,
            );
        }
    }

    pub fn apply_modifications(&mut self) {
        // TODO
        // apply_writing_mode
        // apply_letter_spacing
        // apply_word_spacing
        // apply_length_adjust

        self.resolve_clusters_positions();
    }

    /// Resolves clusters positions.
    ///
    /// Mainly sets the `transform` property.
    ///
    /// Returns the last text position. The next text chunk should start from that position.
    fn resolve_clusters_positions(&mut self) {
        match self.text_flow {
            TextFlow::Linear => self.resolve_clusters_positions_horizontal(),
            _ => {}
        }
    }

    // TODO: Apply linebreaks
    fn resolve_clusters_positions_horizontal(&mut self) {
        let mut x = process_anchor(
            self.anchor,
            self.token_stream
                .iter()
                .fold(0.0, |acc, token| acc + token.clusers_length()),
        );
        let y = 0.0;

        for token in self.token_stream.iter_mut() {
            for cluster in token.outlined_clusters.iter_mut() {
                cluster.transform = cluster.transform.pre_translate(x, y);
                x += cluster.advance;
            }
        }
    }

    pub fn to_paths(
        &mut self,
        fonts_cache: &mut FontsCache,
        fontdb: &fontdb::Database,
    ) -> Vec<Path> {
        let mut new_paths: Vec<Path> = Vec::new();
        let (x, y) = (0.0, 0.0);

        let mut text_ts = Transform::default();
        if self.writing_mode == WritingMode::TopToBottom {
            if let TextFlow::Linear = self.text_flow {
                text_ts = text_ts.pre_rotate_at(90.0, x, y);
            }
        }

        // Outline tokens
        for token in &mut self.token_stream {
            if let Some(path) = token.outline() {
                new_paths.push(path);
            }
        }

        // Outline text decorations
        for interval in self.attribute_intervals.iter() {
            // TODO
        }

        return new_paths;
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
    use crate::usvg::text::{AlignmentBaseline, DominantBaseline, Font, LengthAdjust};
    use ordered_float::OrderedFloat;
    use std::collections::HashMap;

    #[test]
    fn attributed_string_e2e() {
        let fontdb = get_fontdb_with_system_fonts();
        let mut fonts_cache: FontsCache = HashMap::new();

        let text = String::from("Hello, world! This is a test!");
        let attribute_intervals = vec![
            AttributeInterval {
                start: 0,
                stop: 10,
                val: Attribute {
                    font: Font {
                        families: vec![FontFamily::Monospace],
                        stretch: FontStretch::default(),
                        style: FontStyle::default(),
                        weight: 10,
                    },
                    font_size: OrderedFloat(24.0),
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
            },
            AttributeInterval {
                start: 10,
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
            },
        ];
        let mut attributed_string = AttributedString::new(text, attribute_intervals);

        attributed_string.tokanize();
        attributed_string.shape_glyphs(&mut fonts_cache, &fontdb);
        attributed_string.apply_modifications();

        let paths = attributed_string.to_paths(&mut fonts_cache, &fontdb);
        let mut path_builder = tiny_skia_path::PathBuilder::new();
        for path in &paths {
            path_builder.push_path(&path);
        }
        let path = path_builder.finish();

        // https://yqnn.github.io/svg-path-editor/
        println!("Paths: {:?}", path);

        assert_eq!(path.is_some(), true);
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
