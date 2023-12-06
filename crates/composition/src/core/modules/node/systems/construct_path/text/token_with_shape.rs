use rustybuzz::{GlyphBuffer, UnicodeBuffer};

use super::token::Token;

#[derive(Debug)]
pub struct TokenWithShape {
    pub token: Token,
    pub glyph_buffer: GlyphBuffer,
    width: Option<f32>,
}

impl TokenWithShape {
    pub fn new(token: Token, font_face: &rustybuzz::Face) -> Self {
        let mut unicode_buffer = UnicodeBuffer::new();
        unicode_buffer.push_str(Self::get_token_str(&token));

        // Shape the accumulated text in the unicode buffer
        let glyph_buffer = rustybuzz::shape(&font_face, &[], unicode_buffer);

        return Self {
            token,
            glyph_buffer,
            width: None,
        };
    }

    fn get_token_str(token: &Token) -> &str {
        match token {
            Token::Space { .. } => "_",
            Token::TextFragment { value, .. } => value.as_str(),
            _ => "_",
        }
    }

    pub fn get_width(&mut self) -> f32 {
        if let Some(width) = self.width {
            return width;
        }

        let token_width: i32 = self
            .glyph_buffer
            .glyph_positions()
            .iter()
            .map(|pos| pos.x_advance)
            .sum();
        let scale = match &self.token {
            Token::Space { metric, .. } | Token::TextFragment { metric, .. } => metric.scale,
            _ => 1.0,
        };
        let scaled_token_width = token_width as f32 * scale;

        self.width = Some(scaled_token_width);
        return scaled_token_width;
    }
}
