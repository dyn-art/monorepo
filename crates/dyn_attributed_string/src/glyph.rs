use dyn_fonts_book::{em::Em, font::FontId};
use rustybuzz::ttf_parser::GlyphId;
use std::ops::Range;

#[derive(Debug, Default, Clone)]
pub struct Glyph {
    /// A font id this glyph belongs to.
    pub font_id: FontId,
    /// A glyph id.
    pub glyph_id: GlyphId,
    /// Position in bytes in the original string.
    pub range: Range<usize>,
    /// A width relative the font size.
    ///
    /// It's different from advance in that it's not affected by letter spacing, ..
    pub width: Em,
    /// An advance in horizontal direction relative the font size.
    pub x_advance: Em,
    /// An advance in vertical direction relative the font size.
    pub y_advance: Em,
    /// An offset in horizontal direction relative the font size.
    pub x_offset: Em,
    /// An offset in vertical direction relative the font size.
    pub y_offset: Em,
    /// The distance from the baseline to the typographic ascender
    /// relative the font size.
    pub ascent: Em,
    /// The distance from the baseline to the typographic descender
    /// relative the font size.
    pub descent: Em,
}

impl Glyph {
    pub fn height(&self) -> Em {
        self.ascent - self.descent
    }
}
