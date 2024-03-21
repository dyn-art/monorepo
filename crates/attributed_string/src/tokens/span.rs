use super::{Token, TokenVariant};
use std::ops::Range;

/// A span of text with common attributes.
#[derive(Debug, Clone)]
pub struct SpanToken {
    range: Range<usize>,
    /// Tokens within the span, including glyphs, words, and separators.
    tokens: Vec<TokenVariant>,
    /// Bidi level for handling text directionality within the span.
    level: unicode_bidi::Level,
}

impl SpanToken {
    pub fn new(range: Range<usize>, level: unicode_bidi::Level) -> Self {
        Self {
            range,
            tokens: Vec::new(),
            level,
        }
    }

    pub fn push_token(&mut self, token_variant: TokenVariant) -> bool {
        let compatible_token = match token_variant {
            TokenVariant::Glyph(..)
            | TokenVariant::WordSeparator(..)
            | TokenVariant::Linebreak(..)
            | TokenVariant::Bitmap(..)
            | TokenVariant::TextFragment(..) => true,
            _ => false,
        };

        if compatible_token {
            self.tokens.push(token_variant);
        }

        return compatible_token;
    }
}

impl Token for SpanToken {
    fn get_range(&self) -> &Range<usize> {
        &self.range
    }
}
