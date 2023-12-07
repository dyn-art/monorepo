use std::collections::HashMap;

use crate::core::modules::{
    composition::resources::font_cache::FontCacheRes, node::components::types::Text,
};

use super::token::{Token, TokenKind};

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
                } else {
                    continue;
                }
            }
            let buzz_face = font_face_cache.get(&font_hash).unwrap();
            let token_metric = Token::compute_token_metric(buzz_face, font_size);

            // Tokenize the text, considering spaces and line breaks
            let mut start = 0;
            for (index, match_str) in section
                .value
                .match_indices(|c: char| c.is_whitespace() || c == '\n')
            {
                // Create a text fragment token for non-whitespace sections
                if start != index {
                    tokens.push(Token::new(TokenKind::TextFragment {
                        value: String::from(&section.value[start..index]),
                        style: section.style.clone(),
                        metric: token_metric.clone(),
                    }));
                }

                // Create a token for each space or line break
                tokens.push(match match_str {
                    "\n" => Token::new(TokenKind::Linebreak),
                    _ => Token::new(TokenKind::Space {
                        style: section.style.clone(),
                        metric: token_metric.clone(),
                    }),
                });

                start = index + match_str.len();
            }

            // Handle the last word in the section, if any
            if start < section.value.len() {
                tokens.push(Token::new(TokenKind::TextFragment {
                    value: String::from(&section.value[start..]),
                    style: section.style.clone(),
                    metric: token_metric,
                }));
            }
        }

        return Self {
            tokens,
            buzz_face_cache: font_face_cache,
        };
    }

    pub fn drain_into_lines(&mut self) -> Vec<Vec<Token>> {
        let mut lines: Vec<Vec<Token>> = Vec::new();

        // Split tokens into lines at each Linebreak token
        let mut current_line: Vec<Token> = Vec::new();
        for token in self.tokens.drain(..) {
            match token.kind {
                TokenKind::Linebreak { .. } => {
                    lines.push(current_line.drain(..).collect());
                }
                _ => current_line.push(token),
            }
        }

        // Add the last line if it contains any tokens
        if !current_line.is_empty() {
            lines.push(current_line);
        }

        return lines;
    }

    pub fn get_buzz_face(&self, hash: u64) -> Option<&rustybuzz::Face> {
        self.buzz_face_cache.get(&hash)
    }
}
