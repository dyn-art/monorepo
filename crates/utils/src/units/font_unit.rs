use super::{abs::Abs, em::Em, Numeric};

/// A font unit.
/// https://fonts.google.com/knowledge/glossary/unit
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(tag = "type")
)]
pub enum FontUnit {
    Abs(Abs),
    Em(Em),
}

impl FontUnit {
    /// The zero font unit.
    pub const fn zero() -> Self {
        Self::Abs(Abs::zero())
    }

    /// Create a font-relative length.
    pub fn em(em: Em) -> Self {
        Self::Em(em)
    }

    /// Create a font-absolute length.
    pub fn abs(abs: Abs) -> Self {
        Self::Abs(abs)
    }

    /// Convert to an absolute length at the given font size.
    pub fn at(&self, font_size: Abs) -> Abs {
        match self {
            Self::Abs(abs) => *abs,
            Self::Em(em) => em.at(font_size),
        }
    }

    pub fn is_zero(&self) -> bool {
        match self {
            Self::Abs(abs) => abs.is_zero(),
            Self::Em(em) => em.is_zero(),
        }
    }
}

impl Default for FontUnit {
    fn default() -> Self {
        Self::Abs(Abs::default())
    }
}
