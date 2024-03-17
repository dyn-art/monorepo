// This code is closely derived from:
// https://github.com/RazrFalcon/resvg/blob/master/crates/usvg/src/text_to_paths.rs

use super::{byte_index::ByteIndex, resolved_font::ResolvedFont};
use rustybuzz::ttf_parser::GlyphId;
use std::sync::Arc;

/// A glyph.
///
/// Basically, a glyph ID and it's metrics.
#[derive(Clone)]
pub struct Glyph {
    /// The glyph ID in the font.
    pub id: GlyphId,

    /// Position in bytes in the original string.
    ///
    /// We use it to match a glyph with a character in the text chunk and therefore with the style.
    pub byte_idx: ByteIndex,

    /// The glyph offset in font units.
    pub dx: i32,

    /// The glyph offset in font units.
    pub dy: i32,

    /// The glyph width / X-advance in font units.
    pub width: i32,

    /// Reference to the source font.
    ///
    /// Each glyph can have it's own source font.
    pub font: Arc<ResolvedFont>,
}

impl Glyph {
    pub fn is_missing(&self) -> bool {
        self.id.0 == 0
    }
}

/// An iterator over glyph clusters.
///
/// Input:  0 2 2 2 3 4 4 5 5
/// Result: 0 1     4 5   7
pub struct GlyphClusters<'a> {
    data: &'a [Glyph],
    idx: usize,
}

impl<'a> GlyphClusters<'a> {
    fn new(data: &'a [Glyph]) -> Self {
        GlyphClusters { data, idx: 0 }
    }
}

impl<'a> Iterator for GlyphClusters<'a> {
    type Item = (std::ops::Range<usize>, ByteIndex);

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == self.data.len() {
            return None;
        }

        let start = self.idx;
        let cluster = self.data[self.idx].byte_idx;
        for g in &self.data[self.idx..] {
            if g.byte_idx != cluster {
                break;
            }

            self.idx += 1;
        }

        Some((start..self.idx, cluster))
    }
}
