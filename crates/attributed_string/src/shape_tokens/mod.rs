pub mod bitmap;
pub mod glyph;
pub mod linebreak;
pub mod text_fragment;
pub mod word_separator;

use self::{
    bitmap::BitmapToken, glyph::GlyphToken, linebreak::LinebreakToken,
    text_fragment::TextFragmentToken, word_separator::WordSeparatorToken,
};
use dyn_utils::units::abs::Abs;
use std::ops::Range;

pub trait ShapeToken {
    /// Byte range in the original text marking the token's start and stop indices.
    /// Inclusive start, exclusive of stop (start <= x < end).
    fn get_range(&self) -> &Range<usize>;
    /// Get the absolute width of the token.
    fn get_width(&self) -> Abs;
    /// Get the absolute height of the token.
    fn get_height(&self) -> Abs;
}

#[derive(Debug, Clone)]
pub enum ShapeTokenVariant {
    Glyph(GlyphToken),
    WordSeparator(WordSeparatorToken),
    Linebreak(LinebreakToken),
    Bitmap(BitmapToken),
    TextFragment(TextFragmentToken),
}

impl ShapeTokenVariant {
    pub fn get_shape_token(&self) -> &dyn ShapeToken {
        match self {
            ShapeTokenVariant::Glyph(token) => token,
            ShapeTokenVariant::WordSeparator(token) => token,
            ShapeTokenVariant::Linebreak(token) => token,
            ShapeTokenVariant::Bitmap(token) => token,
            ShapeTokenVariant::TextFragment(token) => token,
        }
    }

    pub fn is_blank(&self) -> bool {
        match self {
            ShapeTokenVariant::WordSeparator(_) | ShapeTokenVariant::Linebreak(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct ShapeBuffer {
    pub buffer: Option<rustybuzz::UnicodeBuffer>,
}
