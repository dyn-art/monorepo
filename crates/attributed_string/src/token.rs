use crate::usvg::outlined_cluster::OutlinedCluster;
use smallvec::SmallVec;
use std::ops::Range;

/// Represents a segment of text extracted during parsing.
///
/// Each `Token` holds a portion of text, its position within the original text,
/// and its categorized type based on the parsing logic.
#[derive(Clone)]
pub struct Token {
    /// The actual text this token represents.
    // pub text: String,
    /// The start and end indices of this token in the original text,
    /// allowing for position tracking and identifying attributes.
    pub range: Range<usize>,
    /// The category of this token, defining its role and significance during parsing.
    pub variant: TokenVariant,
    ///
    pub outlined_clusters: SmallVec<[OutlinedCluster; 2]>,
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
