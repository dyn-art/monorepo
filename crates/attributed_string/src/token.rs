use crate::usvg::outlined_cluster::OutlinedCluster;
use std::ops::Range;

/// Represents a segment of text extracted during parsing.
///
/// Each `Token` holds a portion of text, its position within the original text,
/// and its categorized type based on the parsing logic.
#[derive(Clone)]
pub struct Token {
    /// The category of this token, defining its role and significance during parsing.
    pub variant: TokenVariant,
    /// Byte range in the original text marking the token's start and end indices.
    /// Enables attribute identification and position tracking.
    pub range: Range<usize>,
    ///
    pub outlined_clusters: Vec<OutlinedCluster>,
}

impl Token {
    pub fn new(variant: TokenVariant, range: Range<usize>) -> Self {
        Self {
            variant,
            range,
            outlined_clusters: Vec::new(),
        }
    }
}

/// Categorizes types of tokens encountered during text parsing.
///
/// This enum allows for distinguishing between various types of text elements, such as words, separators, or line breaks, facilitating their appropriate handling.
#[derive(Clone)]
pub enum TokenVariant {
    /// A separator that indicates boundaries between words (e.g., spaces, punctuation).
    WordSeparator,
    /// A line break in the text, aiding in text structure recognition.
    Linebreak,
    /// A continuous fragment of text, typically a word or number.
    TextFragment,
}
