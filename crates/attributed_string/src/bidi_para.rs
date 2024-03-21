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
        Some((paragraph, para.level))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn e2e() {
        // This example text is defined using `concat!` because some browsers
        // and text editors have trouble displaying bidi strings.
        let text = concat!["א", "ב", "ג", "a", "b", "c",];

        // Resolve embedding levels within the text.  Pass `None` to detect the
        // paragraph level automatically.
        let bidi_info = BidiInfo::new(&text, None);

        // This paragraph has embedding level 1 because its first strong character is RTL.
        assert_eq!(bidi_info.paragraphs.len(), 1);
        let para = &bidi_info.paragraphs[0];
        assert_eq!(para.level.number(), 1);
        assert_eq!(para.level.is_rtl(), true);

        // Re-ordering is done after wrapping each paragraph into a sequence of
        // lines. For this example, I'll just use a single line that spans the
        // entire paragraph.
        let line = para.range.clone();

        let display = bidi_info.reorder_line(para, line);
        assert_eq!(display, concat!["a", "b", "c", "ג", "ב", "א",]);
    }
}
