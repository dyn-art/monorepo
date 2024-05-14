pub mod layouter;
pub mod line;
pub mod line_wrap;

use dyn_utils::units::{auto_length::AutoLength, axes::Axes};

pub type LayoutSize = Axes<AutoLength>;

#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum LineWrap {
    /// No wrapping
    #[default]
    None,
    // /// Wraps at a glyph level
    // Glyph,
    /// Wraps at the word level
    Word,
    // /// Wraps at the word level, or fallback to glyph level if a word can't fit on a line by itself
    // WordOrGlyph,
}

#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum HorizontalTextAlignment {
    #[default]
    Start,
    End,
    Left,
    Right,
    Center,
    // Justified, // TODO
}

#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum VerticalTextAlignment {
    #[default]
    Top,
    Bottom,
    Center,
    // Justified, // TODO
}

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum TextSizingMode {
    #[default]
    WidthAndHeight,
    Height,
    Fixed,
}
