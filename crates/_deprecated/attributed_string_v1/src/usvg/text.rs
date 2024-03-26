// This code is closely derived from:
// https://github.com/RazrFalcon/resvg/blob/master/crates/usvg/src/tree/text.rs

use super::geom::NonEmptyString;
use std::{fmt::Display, sync::Arc};

/// A type of font family.
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum FontFamily {
    /// A serif font.
    Serif,
    /// A sans-serif font.
    SansSerif,
    /// A cursive font.
    Cursive,
    /// A fantasy font.
    Fantasy,
    /// A monospace font.
    Monospace,
    /// A custom named font.
    Named(String),
}

impl Display for FontFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            FontFamily::Monospace => "monospace".to_string(),
            FontFamily::Serif => "serif".to_string(),
            FontFamily::SansSerif => "sans-serif".to_string(),
            FontFamily::Cursive => "cursive".to_string(),
            FontFamily::Fantasy => "fantasy".to_string(),
            FontFamily::Named(s) => format!("\"{}\"", s),
        };
        write!(f, "{}", str)
    }
}

/// A font stretch property.
#[allow(missing_docs)]
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub enum FontStretch {
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    Normal,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}

impl Default for FontStretch {
    #[inline]
    fn default() -> Self {
        Self::Normal
    }
}

/// A font style property.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum FontStyle {
    /// A face that is neither italic not obliqued.
    Normal,
    /// A form that is generally cursive in nature.
    Italic,
    /// A typically-sloped version of the regular face.
    Oblique,
}

impl Default for FontStyle {
    #[inline]
    fn default() -> FontStyle {
        Self::Normal
    }
}

/// Text font properties.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Font {
    pub families: Vec<FontFamily>,
    pub style: FontStyle,
    pub stretch: FontStretch,
    pub weight: u16,
}

/// A dominant baseline property.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DominantBaseline {
    Auto,
    UseScript,
    NoChange,
    ResetSize,
    Ideographic,
    Alphabetic,
    Hanging,
    Mathematical,
    Central,
    Middle,
    TextAfterEdge,
    TextBeforeEdge,
}

impl Default for DominantBaseline {
    fn default() -> Self {
        Self::Auto
    }
}

/// An alignment baseline property.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AlignmentBaseline {
    Auto,
    Baseline,
    BeforeEdge,
    TextBeforeEdge,
    Middle,
    Central,
    AfterEdge,
    TextAfterEdge,
    Ideographic,
    Alphabetic,
    Hanging,
    Mathematical,
}

impl Default for AlignmentBaseline {
    fn default() -> Self {
        Self::Auto
    }
}

/// A baseline shift property.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BaselineShift {
    Baseline,
    Subscript,
    Superscript,
    Number(u16),
}

impl Default for BaselineShift {
    #[inline]
    fn default() -> BaselineShift {
        BaselineShift::Baseline
    }
}

/// A length adjust property.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LengthAdjust {
    Spacing,
    SpacingAndGlyphs,
}

impl Default for LengthAdjust {
    fn default() -> Self {
        Self::Spacing
    }
}

/// A text chunk anchor property.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TextAnchor {
    Start,
    Middle,
    End,
}

impl Default for TextAnchor {
    fn default() -> Self {
        Self::Start
    }
}

/// A path used by text-on-path.
#[derive(Debug)]
pub struct TextPath {
    pub id: NonEmptyString,
    pub start_offset: f32,
    pub path: Arc<tiny_skia_path::Path>,
}

impl TextPath {
    /// Element's ID.
    ///
    /// Taken from the SVG itself.
    pub fn id(&self) -> &str {
        self.id.get()
    }

    /// A text offset in SVG coordinates.
    ///
    /// Percentage values already resolved.
    pub fn start_offset(&self) -> f32 {
        self.start_offset
    }

    /// A path.
    pub fn path(&self) -> &tiny_skia_path::Path {
        &self.path
    }
}

/// A text chunk flow property.
#[derive(Clone, Debug)]
pub enum TextFlow {
    /// A linear layout.
    ///
    /// Includes left-to-right, right-to-left and top-to-bottom.
    Linear,
    /// A text-on-path layout.
    Path(Arc<TextPath>),
}

/// A writing mode.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum WritingMode {
    LeftToRight,
    TopToBottom,
}
