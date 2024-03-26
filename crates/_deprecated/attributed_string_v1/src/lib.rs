pub mod attrs;
pub mod font;
pub mod token;
pub mod usvg;

use attrs::{Attrs, AttrsInterval};
use glam::Vec2;
use rust_lapper::Lapper;
use std::ops::Range;
use tiny_skia_path::{Path, Transform};
use token::{Token, TokenVariant};
use unicode_linebreak::BreakClass;
use usvg::{
    database::FontsCache,
    text::{TextAnchor, TextFlow, WritingMode},
};

/// `AttributedString` represents a string with associated attributes applied
/// to certain ranges.
#[derive(Clone)]
struct AttributedString {
    text: String,
    token_stream: Vec<Token>,
    attrs_intervals: Lapper<usize, Attrs>,

    anchor: TextAnchor,
    text_flow: TextFlow,
    writing_mode: WritingMode,

    bbox: Vec2,
}

impl AttributedString {
    pub fn new(text: String, attrs_intervals: Vec<AttrsInterval>, bbox: Vec2) -> Self {
        let mut attrs_intervals = Lapper::new(attrs_intervals);
        attrs_intervals.divide_overlaps_with(|overlaps| {
            let mut merged_attrs = Attrs::new();
            for &attrs in overlaps.iter() {
                merged_attrs.merge(attrs.clone());
            }
            return merged_attrs;
        });

        Self {
            text,
            token_stream: Vec::new(),
            attrs_intervals,
            anchor: TextAnchor::Start,
            text_flow: TextFlow::Linear,
            writing_mode: WritingMode::LeftToRight,
            bbox,
        }
    }

    pub fn tokenize(&mut self) {
        let mut token_stream = Vec::new();

        let mut start = 0;

        // Process each character for potential tokenization
        for (index, _char) in self.text.chars().enumerate() {
            let break_class = unicode_linebreak::break_property(_char as u32);

            match break_class {
                BreakClass::Mandatory
                | BreakClass::LineFeed
                | BreakClass::NextLine
                | BreakClass::CarriageReturn => {
                    // Add text fragment token
                    if start != index {
                        token_stream.push(Token::new(
                            TokenVariant::TextFragment,
                            Range { start, end: index },
                        ));
                    }

                    // Add line break token
                    token_stream.push(Token::new(
                        TokenVariant::Linebreak,
                        Range {
                            start: index,
                            end: index + 1,
                        },
                    ));
                    start = index + 1;
                }
                BreakClass::Space | BreakClass::ZeroWidthSpace => {
                    // Add text fragment token
                    if start != index {
                        token_stream.push(Token::new(
                            TokenVariant::TextFragment,
                            Range { start, end: index },
                        ));
                    }

                    // Add word separator token
                    token_stream.push(Token::new(
                        TokenVariant::WordSeparator,
                        Range {
                            start: index,
                            end: index + 1,
                        },
                    ));
                    start = index + 1;
                }
                _ => {}
            }
        }

        // Handle the last text fragment, if any
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
            token.shape_glyphs(&self.text, &mut self.attrs_intervals, fonts_cache, fontdb);
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

    fn resolve_clusters_positions_horizontal(&mut self) {
        // let mut x = process_anchor(
        //     self.anchor,
        //     self.token_stream
        //         .iter()
        //         .fold(0.0, |acc, token| acc + token.get_advance()),
        // );
        let mut x = 0.0;
        let mut y = 0.0;

        let mut current_width = 0.0;
        let mut current_line_height = 0.0;
        for token in self.token_stream.iter_mut() {
            let token_width = token.get_advance();
            let token_height = token.get_max_height();

            let force_break = match token.variant {
                TokenVariant::Linebreak => true,
                _ => false,
            };
            let will_wrap = current_width + token_width > self.bbox.x || force_break;

            if will_wrap {
                let ignore = match token.variant {
                    TokenVariant::Linebreak => true,
                    TokenVariant::WordSeparator => true,
                    _ => false,
                };

                y += current_line_height;
                current_width = if ignore { 0.0 } else { token_width };
                current_line_height = if ignore { 0.0 } else { token_height };
            } else {
                current_width += token_width;
                if token_height > current_line_height {
                    current_line_height = token_height;
                }
            }

            x = current_width - token_width;

            // Position clusters of token
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
        for interval in self.attrs_intervals.iter() {
            // TODO
        }

        return new_paths;
    }
}

#[cfg(test)]
mod tests {
    use self::usvg::text::FontFamily;
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn attributed_string_e2e() {
        let fontdb = get_fontdb_with_system_fonts();
        let mut fonts_cache: FontsCache = HashMap::new();

        let text = String::from("Hello, world! This is a test!");
        let attribute_intervals = vec![
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
            AttributedString::new(text, attribute_intervals, Vec2::new(100.0, 100.0));

        attributed_string.tokenize();
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
