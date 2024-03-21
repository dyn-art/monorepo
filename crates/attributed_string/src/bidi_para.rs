// Based on:
// https://github.com/pop-os/cosmic-text/blob/main/src/bidi_para.rs

use unicode_bidi::{bidi_class, BidiClass, BidiInfo, Level, ParagraphInfo};

/// An iterator over the paragraphs in the input text.
/// It is equivalent to [`core::str::Lines`] but follows `unicode-bidi` behaviour.
#[derive(Debug)]
pub struct BidiParagraphs<'text> {
    text: &'text str,
    info: std::vec::IntoIter<ParagraphInfo>,
}

impl<'text> BidiParagraphs<'text> {
    /// Create an iterator to split the input text into paragraphs
    /// in accordance with `unicode-bidi` behaviour.
    pub fn new(text: &'text str) -> Self {
        let info = BidiInfo::new(text, None);
        let info = info.paragraphs.into_iter();
        Self { text, info }
    }
}

impl<'text> Iterator for BidiParagraphs<'text> {
    type Item = (&'text str, Level);

    fn next(&mut self) -> Option<Self::Item> {
        let para = self.info.next()?;
        let paragraph = &self.text[para.range];
        // `para.range` includes the newline that splits the line, so remove it if present
        let mut char_indices = paragraph.char_indices();
        if let Some(i) = char_indices.next_back().and_then(|(i, c)| {
            // `BidiClass::B` is a Paragraph_Separator (various newline characters)
            (bidi_class(c) == BidiClass::B).then_some(i)
        }) {
            Some((&paragraph[0..i], para.level))
        } else {
            Some((paragraph, para.level))
        }
    }
}
