// This code is closely derived from:
// https://github.com/RazrFalcon/resvg/blob/master/crates/usvg/src/text_to_paths.rs

/// A read-only text index in bytes.
///
/// Guarantee to be on a char boundary and in text bounds.
#[derive(Clone, Copy, PartialEq)]
pub struct ByteIndex(usize);

impl ByteIndex {
    pub fn new(i: usize) -> Self {
        ByteIndex(i)
    }

    pub fn value(&self) -> usize {
        self.0
    }

    /// Converts byte position into a code point position.
    pub fn code_point_at(&self, text: &str) -> usize {
        text.char_indices()
            .take_while(|(i, _)| *i != self.0)
            .count()
    }

    /// Converts byte position into a character.
    pub fn char_from(&self, text: &str) -> char {
        text[self.0..].chars().next().unwrap()
    }
}
