use super::{abs::Abs, ratio::Ratio};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(tag = "type")
)]
pub enum Length {
    Abs { value: Abs },
    Ratio { value: Ratio },
}

impl Length {
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

    pub fn into_abs(&self, whole: Abs) -> Self {
        match self {
            Self::Abs { .. } => *self,
            Self::Ratio { value } => Self::abs(value.of(whole)),
        }
    }
}

impl Default for Length {
    fn default() -> Self {
        Self::zero()
    }
}
