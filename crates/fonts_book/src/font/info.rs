use super::variant::FontVariant;
use std::fmt::{Display, Formatter};

/// Properties of a single font.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct FontInfo {
    /// The typographic font family this font is part of.
    pub family: FontFamily,
    /// Properties that distinguish this font from other fonts in the same
    /// family.
    pub variant: FontVariant,
}

impl FontInfo {
    /// Create a font info from its two components.
    pub fn new(family: FontFamily, variant: FontVariant) -> Self {
        Self { family, variant }
    }
}

/// A typographic font family.
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
