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
    Abs { value: Abs },
    Em { value: Em },
}

impl FontUnit {
    /// The zero font unit.
    pub const fn zero() -> Self {
        Self::Abs { value: Abs::zero() }
    }

    /// Create a font-relative length.
    pub fn em(value: Em) -> Self {
        Self::Em { value }
    }

    /// Create a font-absolute length.
    pub fn abs(value: Abs) -> Self {
        Self::Abs { value }
    }

    /// Convert to an absolute length at the given font size.
    pub fn at(&self, font_size: Abs) -> Abs {
        match self {
            Self::Abs { value } => *value,
            Self::Em { value } => value.at(font_size),
        }
    }

    pub fn is_zero(&self) -> bool {
        match self {
            Self::Abs { value } => value.is_zero(),
            Self::Em { value } => value.is_zero(),
        }
    }
}

impl Default for FontUnit {
    fn default() -> Self {
        Self::Abs {
            value: Abs::default(),
        }
    }
}
