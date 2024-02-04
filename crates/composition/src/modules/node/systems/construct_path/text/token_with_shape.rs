use rustybuzz::{GlyphBuffer, UnicodeBuffer};

use super::token::{Token, TokenKind};

#[derive(Debug)]
pub struct TokenWithShape {
    pub token: Token,
    pub glyph_buffer: GlyphBuffer,
    width: Option<f32>,
}

impl TokenWithShape {
    pub fn new(token: Token, font_face: &rustybuzz::Face) -> Self {
        // Shape the token's text
        let mut unicode_buffer = UnicodeBuffer::new();
        unicode_buffer.push_str(token.get_str());
        let glyph_buffer = rustybuzz::shape(&font_face, &[], unicode_buffer);

        return Self {
            token,
            glyph_buffer,
            width: None,
        };
    }

    pub fn get_width(&mut self) -> f32 {
        if let Some(width) = self.width {
            return width;
        }

        let width = self.calculate_width();
        self.width = Some(width);
        return width;
    }

    pub fn calculate_width(&self) -> f32 {
        // Calculate the total width of the glyph buffer
        let token_width: i32 = self
            .glyph_buffer
            .glyph_positions()
            .iter()
            .map(|pos| pos.x_advance)
            .sum();

        // Determine the scale based on the token type
        let scale = match &self.token.kind {
            TokenKind::Space { metric, .. } | TokenKind::TextFragment { metric, .. } => {
                metric.font_scale
            }
            _ => 1.0,
        };

        // Cache the calculated width for future use
        let scaled_token_width = token_width as f32 * scale;

        return scaled_token_width;
    }
}
