pub mod bitmap;
pub mod glyph;
pub mod line;
pub mod linebreak;
pub mod paragraph;
pub mod span;
pub mod text_fragment;
pub mod word_separator;

use self::{
    bitmap::BitmapToken, glyph::GlyphToken, line::LineToken, linebreak::LinebreakToken,
    paragraph::ParagraphToken, span::SpanToken, text_fragment::TextFragmentToken,
    word_separator::WordSeparatorToken,
};
use std::ops::Range;

pub trait Token {
    /// Byte range in the original text marking the token's start and stop indices.
    /// Inclusive start, exclusive of stop (start <= x < end).
    fn get_range(&self) -> &Range<usize>;
}

#[derive(Debug, Clone)]
pub enum TokenVariant {
    Glyph(GlyphToken),
    WordSeparator(WordSeparatorToken),
    Linebreak(LinebreakToken),
    Bitmap(BitmapToken),
    TextFragment(TextFragmentToken),
    Span(SpanToken),
    Line(LineToken),
    Paragraph(ParagraphToken),
}
