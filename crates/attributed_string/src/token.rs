use smallvec::SmallVec;
use std::ops::Range;

#[derive(Debug, Clone)]
pub enum TokenVariant {
    Cluster(GlyphClusterToken),
    WordSeparator(WordSeparatorToken),
    Linbreak(LinbreakToken),
    TextFragment(TextFragmentToken),
    Unresolved,
}

trait Token {
    fn get_range(&self) -> &Range<usize>;
}

/// A TextFragment Token
#[derive(Debug, Clone)]
pub struct TextFragmentToken {
    pub range: Range<usize>,

    pub token_cluster: SmallVec<[GlyphClusterToken; 5]>,
}

impl Token for TextFragmentToken {
    fn get_range(&self) -> &Range<usize> {
        &self.range
    }
}

/// A GlyphCluster/Grapheme Token
#[derive(Debug, Clone)]
pub struct GlyphClusterToken {
    pub range: Range<usize>,

    // Set after outline
    outlined: Option<()>,
}

impl Token for GlyphClusterToken {
    fn get_range(&self) -> &Range<usize> {
        &self.range
    }
}

/// A Word Separator Token
#[derive(Debug, Clone)]
pub struct WordSeparatorToken {
    pub range: Range<usize>,

    pub token_cluster: SmallVec<[GlyphClusterToken; 2]>,
}

impl Token for WordSeparatorToken {
    fn get_range(&self) -> &Range<usize> {
        &self.range
    }
}

/// A Linebreak Token
#[derive(Debug, Clone)]
pub struct LinbreakToken {
    pub range: Range<usize>,
}

impl Token for LinbreakToken {
    fn get_range(&self) -> &Range<usize> {
        &self.range
    }
}
