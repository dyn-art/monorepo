use std::collections::HashMap;

use crate::modules::{
    composition::resources::font_cache::FontCacheRes, node::components::types::TextNode,
};

use super::token::{Token, TokenKind};

pub struct TokenStream<'a> {
    tokens: Vec<Token>,
    buzz_face_cache: HashMap<u64, rustybuzz::Face<'a>>,
}

impl<'a> TokenStream<'a> {
    pub fn from_text(text: &TextNode, font_cache: &'a mut FontCacheRes) -> Self {
        let mut tokens: Vec<Token> = Vec::new();
        let mut buzz_face_cache: HashMap<u64, rustybuzz::Face<'a>> = HashMap::new();

        // Preload required faces to avoid mutable borrow conflicts during local font face caching
        // TODO: Figure out whether preloading the ttf_face once or reconstructing the ttf_face everytime
        //  is more performant
        // for segment in &text.segments {
        //     font_cache.load_ttfp_face(&segment.style.font_id);
        // }

        // Iterate through text segments, creating tokens
        for segment in &text.segments {
            let font_id = segment.style.font_id;
            let font_size = segment.style.font_size as f32;

            // Cache rustybuzz font face locally
            let buzz_face = buzz_face_cache.entry(font_id).or_insert_with(|| {
                font_cache
                    .create_buzz_face(&segment.style.font_id)
                    .expect("Font face not found")
            });
            let token_metric = Token::compute_token_metric(buzz_face, font_size);

            // Tokenize the text, considering spaces and line breaks
            let mut start = 0;
            for (index, match_str) in segment
                .value
                .match_indices(|c: char| c.is_whitespace() || c == '\n')
            {
                // Create a text fragment token for non-whitespace segments
                if start != index {
                    tokens.push(Token::new(TokenKind::TextFragment {
                        value: String::from(&segment.value[start..index]),
                        style: segment.style,
                        metric: token_metric,
                    }));
                }

                // Create a token for each space or line break
                tokens.push(match match_str {
                    "\n" => Token::new(TokenKind::Linebreak),
                    _ => Token::new(TokenKind::Space {
                        style: segment.style,
                        metric: token_metric,
                    }),
                });

                start = index + match_str.len();
            }

            // Handle the last word in the segment, if any
            if start < segment.value.len() {
                tokens.push(Token::new(TokenKind::TextFragment {
                    value: String::from(&segment.value[start..]),
                    style: segment.style,
                    metric: token_metric,
                }));
            }
        }

        return Self {
            tokens,
            buzz_face_cache,
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
