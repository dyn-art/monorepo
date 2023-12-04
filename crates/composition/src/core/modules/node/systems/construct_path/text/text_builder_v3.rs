use bevy_utils::HashMap;
use log::info;

use crate::core::modules::{
    composition::resources::font_cache::FontCacheRes,
    node::components::{
        mixins::Anchor,
        types::{Text, TextStyle},
    },
};

pub struct TextBuilder {
    subpaths: Vec<Vec<Anchor>>,
}

impl TextBuilder {
    pub fn new(width: f32) -> Self {
        Self {
            subpaths: Vec::new(),
        }
    }

    pub fn process_text(&mut self, text: &Text, font_cache: &mut FontCacheRes) {
        let token_stream = TokenStream::from_text(text, font_cache);

        // TODO
        info!("process_text: {:#?}", token_stream.tokens);
    }

    /// Converts the constructed paths into a flat vector of vertices.
    pub fn into_vertices(&mut self) -> Vec<Anchor> {
        self.subpaths.drain(..).flatten().collect()
    }
}

#[derive(Debug)]
pub enum Token {
    TextFragment { value: String, style: TextStyle },
    Space { style: TextStyle },
    Linebreak,
}

impl Token {
    pub fn text_fragment(value: &str, style: TextStyle) -> Self {
        Token::TextFragment {
            value: value.to_string(),
            style,
        }
    }
}

pub struct TokenStream<'a> {
    pub tokens: Vec<Token>,
    font_face_cache: HashMap<u64, rustybuzz::Face<'a>>,
}

impl<'a> TokenStream<'a> {
    pub fn from_text(text: &Text, font_cache: &'a mut FontCacheRes) -> Self {
        let mut tokens: Vec<Token> = Vec::new();
        let mut font_face_cache: HashMap<u64, rustybuzz::Face<'a>> = HashMap::new();

        // Preload required faces to avoid mutable borrow conflicts during local font face caching
        for section in &text.sections {
            font_cache.load_font_face(&section.style.font_hash);
        }

        // Iterate through text sections, creating tokens
        for section in &text.sections {
            if !font_face_cache.contains_key(&section.style.font_hash) {
                if let Some(face) = font_cache.get_font_face(&section.style.font_hash) {
                    font_face_cache.insert(section.style.font_hash, face.clone());
                }
            }

            // Tokenize the text, considering spaces and line breaks
            let mut start = 0;
            for (index, match_str) in section
                .value
                .match_indices(|c: char| c.is_whitespace() || c == '\n')
            {
                // Create a text fragment token for non-whitespace sections
                if start != index {
                    tokens.push(Token::text_fragment(
                        &section.value[start..index],
                        section.style.clone(),
                    ));
                }

                // Create a token for each space or line break
                tokens.push(match match_str {
                    "\n" => Token::Linebreak,
                    _ => Token::Space {
                        style: section.style.clone(),
                    },
                });

                start = index + match_str.len();
            }

            // Handle the last word in the section, if any
            if start < section.value.len() {
                tokens.push(Token::text_fragment(
                    &section.value[start..],
                    section.style.clone(),
                ));
            }
        }

        return Self {
            tokens,
            font_face_cache,
        };
    }

    pub fn into_lines(&self) -> Vec<Vec<Token>> {
        // TODO
        return Vec::new();
    }
}
