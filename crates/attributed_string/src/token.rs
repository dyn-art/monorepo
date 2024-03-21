use crate::glyph::Glyph;
use glam::Vec2;
use std::ops::Range;

pub struct Token {
    /// The variant of the token.
    pub variant: TokenVariant,
    /// Byte range in the original text marking the token's start and stop indices.
    /// Inclusive start, exclusive of stop (start <= x < end).
    pub range: Range<usize>,
}

pub enum TokenVariant {
    Glyph(GlyphToken),
    Word(WordToken),
    Line(LineToken),
    WordSeparator(WordSeparatorToken),
    Linebreak,
    Bitmap(BitmapToken),
}

pub struct GlyphToken {
    glyph: Glyph,
    transform: Vec2,
}

pub struct WordToken {
    /// Should only contain of GlyphToken's
    tokens: Vec<Token>,
    blank: bool,
    x_advance: f32,
    y_advance: f32,
}

pub struct SpanToken {
    /// Should only contain of GlyphToken's, WordToken's, WordSeparatorToken's and BitmapToken's
    tokens: Vec<Token>,
    level: unicode_bidi::Level,
}

pub struct LineToken {
    /// Should only contain of SpanToken's
    tokens: Vec<Token>,
}

pub struct WordSeparatorToken {
    token: GlyphToken,
}

pub struct BitmapToken {
    // TODO
}
