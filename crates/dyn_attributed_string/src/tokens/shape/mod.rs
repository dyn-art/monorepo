pub mod bitmap;
pub mod glyph;
pub mod linebreak;
pub mod text_fragment;
pub mod word_separator;

use self::{
    bitmap::BitmapToken, glyph::GlyphToken, linebreak::LinebreakToken,
    text_fragment::TextFragmentToken, word_separator::WordSeparatorToken,
};
use std::ops::Range;

pub trait ShapeToken {
    /// Byte range in the original text marking the token's start and stop indices.
    /// Inclusive start, exclusive of stop (start <= x < end).
    fn get_range(&self) -> &Range<usize>;
}

#[derive(Debug, Clone)]
pub enum ShapeTokenVariant {
    Glyph(GlyphToken),
    WordSeparator(WordSeparatorToken),
    Linebreak(LinebreakToken),
    Bitmap(BitmapToken),
    TextFragment(TextFragmentToken),
}

#[derive(Debug)]
pub struct ShapeBuffer {
    pub buffer: Option<rustybuzz::UnicodeBuffer>,
}
