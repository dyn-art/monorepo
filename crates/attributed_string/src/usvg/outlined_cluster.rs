// This code is closely derived from:
// https://github.com/RazrFalcon/resvg/blob/master/crates/usvg/src/text_to_paths.rs

use super::byte_index::ByteIndex;
use tiny_skia_path::Transform;

/// An outlined cluster.
///
/// Cluster/grapheme is a single, unbroken, renderable character.
/// It can be positioned, rotated, spaced, etc.
///
/// Let's say we have `й` which is *CYRILLIC SMALL LETTER I* and *COMBINING BREVE*.
/// It consists of two code points, will be shaped (via harfbuzz) as two glyphs into one cluster,
/// and then will be combined into the one `OutlinedCluster`.
#[derive(Clone)]
pub struct OutlinedCluster {
    /// Position in bytes in the original string.
    ///
    /// We use it to match a cluster with a character in the text chunk and therefore with the style.
    pub byte_idx: ByteIndex,

    /// Cluster's original codepoint.
    ///
    /// Technically, a cluster can contain multiple codepoints,
    /// but we are storing only the first one.
    pub codepoint: char,

    /// Cluster's width.
    ///
    /// It's different from advance in that it's not affected by letter spacing and word spacing.
    pub width: f32,

    /// An advance along the X axis.
    ///
    /// Can be negative.
    pub advance: f32,

    /// An ascent in SVG coordinates.
    pub ascent: f32,

    /// A descent in SVG coordinates.
    pub descent: f32,

    /// A x-height in SVG coordinates.
    pub x_height: f32,

    /// Indicates that this cluster was affected by the relative shift (via dx/dy attributes)
    /// during the text layouting. Which breaks the `text-decoration` line.
    ///
    /// Used during the `text-decoration` processing.
    pub has_relative_shift: bool,

    /// An actual outline.
    pub path: Option<tiny_skia_path::Path>,

    /// A cluster's transform that contains it's position, rotation, etc.
    pub transform: Transform,

    /// Not all clusters should be rendered.
    ///
    /// For example, if a cluster is outside the text path than it should not be rendered.
    pub visible: bool,
}

impl OutlinedCluster {
    fn height(&self) -> f32 {
        self.ascent - self.descent
    }
}
