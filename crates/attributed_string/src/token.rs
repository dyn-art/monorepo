use crate::glyph::Glyph;
use glam::Vec2;
use std::ops::Range;

#[derive(Debug, Clone)]
pub struct Token {
    /// The variant of the token.
    pub variant: TokenVariant,
    /// Byte range in the original text marking the token's start and stop indices.
    /// Inclusive start, exclusive of stop (start <= x < end).
    pub range: Range<usize>,
}

#[derive(Debug, Clone)]
pub enum TokenVariant {
    Glyph(GlyphToken),
    Word(WordToken),
    Line(LineToken),
    WordSeparator(WordSeparatorToken),
    Linebreak,
    Bitmap(BitmapToken),
}

#[derive(Debug, Clone)]
pub struct GlyphToken {
    glyph: Glyph,
    transform: Vec2,
}

#[derive(Debug, Clone)]
pub struct WordToken {
    /// Should only contain of GlyphToken's
    tokens: Vec<Token>,
    blank: bool,
    x_advance: f32,
    y_advance: f32,
}

#[derive(Debug, Clone)]
pub struct SpanToken {
    /// Should only contain of GlyphToken's, WordToken's, WordSeparatorToken's and BitmapToken's
    tokens: Vec<Token>,
    level: unicode_bidi::Level,
}

#[derive(Debug, Clone)]
pub struct LineToken {
    /// Should only contain of SpanToken's
    tokens: Vec<Token>,
}

#[derive(Debug, Clone)]
pub struct WordSeparatorToken {
    token: GlyphToken,
}

#[derive(Debug, Clone)]
pub struct BitmapToken {
    // TODO
}
