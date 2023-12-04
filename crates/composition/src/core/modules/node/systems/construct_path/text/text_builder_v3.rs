use bevy_utils::HashMap;
use log::info;
use rustybuzz::UnicodeBuffer;

use crate::core::modules::{
    composition::resources::font_cache::FontCacheRes,
    node::components::{
        mixins::Anchor,
        types::{Text, TextStyle},
    },
};

pub struct TextBuilder {
    subpaths: Vec<Vec<Anchor>>,
    max_line_width: f32,
}

impl TextBuilder {
    pub fn new(max_line_width: f32) -> Self {
        Self {
            subpaths: Vec::new(),
            max_line_width,
        }
    }

    pub fn process_text(&mut self, text: &Text, font_cache: &mut FontCacheRes) {
        let token_stream = TokenStream::from_text(text, font_cache);
        let lines = token_stream.into_lines();

        for line in &lines {
            self.process_line(line, &token_stream)
        }
    }

    pub fn process_line(&mut self, line: &Vec<&Token>, token_stream: &TokenStream) {
        let mut unicode_buffer = UnicodeBuffer::new();

        for (index, token) in line.iter().enumerate() {
            if let Token::Space { style, metric } | Token::TextFragment { style, metric, .. } =
                token
            {
                if let Some(font_face) = token_stream.get_buzz_face(style.font_hash) {
                    // Append to render string to the unicode buffer
                    unicode_buffer.push_str(match token {
                        Token::Space { .. } => " ",
                        Token::TextFragment { value, .. } => value.as_str(),
                        _ => "",
                    });

                    // Shape the accumulated text in the unicode buffer
                    let glyph_buffer = rustybuzz::shape(&font_face, &[], unicode_buffer);

                    // Render the glyphs and prepare the unicode buffer for the next iteration
                    // self.process_glyphs(&glyph_buffer, &font_face);
                    unicode_buffer = glyph_buffer.clear();
                }
            }
        }
    }

    /// Converts the constructed paths into a flat vector of vertices.
    pub fn into_vertices(&mut self) -> Vec<Anchor> {
        self.subpaths.drain(..).flatten().collect()
    }
}

#[derive(Debug)]
pub enum Token {
    TextFragment {
        value: String,
        style: TextStyle,
        metric: TokenStyleMetric,
    },
    Space {
        style: TextStyle,
        metric: TokenStyleMetric,
    },
    Linebreak,
}

pub struct TokenStream<'a> {
    tokens: Vec<Token>,
    buzz_face_cache: HashMap<u64, rustybuzz::Face<'a>>,
}

impl<'a> TokenStream<'a> {
    pub fn from_text(text: &Text, font_cache: &'a mut FontCacheRes) -> Self {
        let mut tokens: Vec<Token> = Vec::new();
        let mut font_face_cache: HashMap<u64, rustybuzz::Face<'a>> = HashMap::new();

        // Preload required faces to avoid mutable borrow conflicts during local font face caching
        for section in &text.sections {
            font_cache.load_ttfp_face(&section.style.font_hash);
        }

        // Iterate through text sections, creating tokens
        for section in &text.sections {
            let font_hash = section.style.font_hash;
            let font_size = section.style.font_size as f32;

            // Cache rustybuzz font face locally
            if !font_face_cache.contains_key(&font_hash) {
                if let Some(face) = font_cache.get_buzz_face(&section.style.font_hash) {
                    font_face_cache.insert(section.style.font_hash, face.clone());
                }
            }
            let buzz_face = font_face_cache.get(&font_hash).unwrap();

            // Tokenize the text, considering spaces and line breaks
            let mut start = 0;
            for (index, match_str) in section
                .value
                .match_indices(|c: char| c.is_whitespace() || c == '\n')
            {
                // Create a text fragment token for non-whitespace sections
                if start != index {
                    tokens.push(Token::TextFragment {
                        value: String::from(&section.value[start..index]),
                        style: section.style.clone(),
                        metric: Self::get_token_style_metric(buzz_face, font_size),
                    });
                }

                // Create a token for each space or line break
                tokens.push(match match_str {
                    "\n" => Token::Linebreak,
                    _ => Token::Space {
                        style: section.style.clone(),
                        metric: Self::get_token_style_metric(buzz_face, font_size),
                    },
                });

                start = index + match_str.len();
            }

            // Handle the last word in the section, if any
            if start < section.value.len() {
                tokens.push(Token::TextFragment {
                    value: String::from(&section.value[start..]),
                    style: section.style.clone(),
                    metric: Self::get_token_style_metric(buzz_face, font_size),
                });
            }
        }

        return Self {
            tokens,
            buzz_face_cache: font_face_cache,
        };
    }

    pub fn into_lines(&self) -> Vec<Vec<&Token>> {
        let mut lines: Vec<Vec<&Token>> = Vec::new();

        let mut current_line: Vec<&Token> = Vec::new();
        for token in &self.tokens {
            if let Token::Linebreak = token {
                lines.push(current_line.drain(..).collect());
            } else {
                current_line.push(token);
            }
        }

        return lines;
    }

    pub fn get_buzz_face(&self, hash: u64) -> Option<&rustybuzz::Face> {
        self.buzz_face_cache.get(&hash)
    }

    fn get_token_style_metric(buzz_face: &rustybuzz::Face<'a>, font_size: f32) -> TokenStyleMetric {
        let scale = (buzz_face.units_per_em() as f32).recip() * font_size;
        let font_height = buzz_face.height() as f32;
        return TokenStyleMetric {
            ascender: (buzz_face.ascender() as f32 / font_height) * font_size / scale,
            height: font_height,
            scale,
        };
    }
}

#[derive(Debug)]
pub struct TokenStyleMetric {
    pub height: f32,
    pub ascender: f32,
    pub scale: f32,
}
