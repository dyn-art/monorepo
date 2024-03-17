pub mod token;
pub mod usvg;

use rust_lapper::{Interval, Lapper};
use smallvec::SmallVec;
use std::ops::Range;
use token::{Token, TokenVariant};

#[derive(Clone)]
struct AttributedString {
    text: String,
    token_stream: SmallVec<[Token; 8]>,
    attribute_intervals: Lapper<usize, Attribute>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Attribute {
    // TODO
}

type AttributeInterval = Interval<usize, Attribute>;

impl AttributedString {
    pub fn new(text: String, attribute_intervals: Vec<AttributeInterval>) -> Self {
        Self {
            text,
            token_stream: SmallVec::new(),
            attribute_intervals: Lapper::new(attribute_intervals),
        }
    }

    pub fn tokanize(&mut self) {
        let mut token_stream: SmallVec<[Token; 8]> = SmallVec::new();

        // Tokenize the text, considering spaces and line breaks
        let mut start = 0;
        for (index, match_str) in self
            .text
            .match_indices(|c: char| is_word_separator_char(c) || is_linebreak_char(c))
        {
            // Create a text fragment token for non-whitespace segments
            if start != index {
                token_stream.push(Token {
                    // text: String::from(match_str),
                    variant: TokenVariant::TextFragment,
                    range: Range {
                        start: index,
                        end: match_str.len(),
                    },
                })
            }

            // Create a token for each space or line break
            token_stream.push(match match_str.chars().next() {
                Some(c) if is_word_separator_char(c) => Token {
                    // text: String::from(match_str),
                    variant: TokenVariant::WordSeparator,
                    range: Range {
                        start: index,
                        end: match_str.len(),
                    },
                },
                Some(c) if is_linebreak_char(c) => Token {
                    // text: String::from(match_str),
                    variant: TokenVariant::Linebreak,
                    range: Range {
                        start: index,
                        end: match_str.len(),
                    },
                },
                _ => continue, // Should never happen
            });

            start = index + match_str.len();
        }

        // Handle the last text fragment in the segment, if any
        if start < self.text.len() {
            token_stream.push(Token {
                // text: String::from(&self.text[start..]),
                variant: TokenVariant::TextFragment,
                range: Range {
                    start,
                    end: self.text.len(),
                },
            });
        }

        self.token_stream = token_stream;
    }

    pub fn outline(&mut self, fontdb: &fontdb::Database) {
        // TODO
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
