use super::{abs::Abs, length::Length, ratio::Ratio};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(tag = "type")
)]
pub enum AutoLength {
    Abs { value: Abs },
    Ratio { value: Ratio },
    Auto,
}

impl AutoLength {
    /// The zero length.
    pub const fn zero() -> Self {
        Self::Abs { value: Abs::zero() }
    }

    pub fn abs(value: Abs) -> Self {
        Self::Abs { value }
    }

    pub fn ratio(value: Ratio) -> Self {
        Self::Ratio { value }
    }

    pub fn auto() -> Self {
        Self::Auto
    }

    pub fn from_length(length: Length) -> Self {
        match length {
            Length::Abs { value } => Self::abs(value),
            Length::Ratio { value } => Self::ratio(value),
        }
    }
}

impl Default for AutoLength {
    fn default() -> Self {
        Self::from_length(Length::default())
    }
}
