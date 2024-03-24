use super::{ratio::Ratio, scalar::Scalar, Numeric};
use std::fmt::Debug;
use std::fmt::Formatter;

const MIN_OPACITY: f32 = 0.0;
const MAX_OPACITY: f32 = 1.0;

/// An opacity.
#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[cfg_attr(feature = "serde_support", derive(serde::Serialize, specta::Type))]
pub struct Opacity(Ratio);

impl Opacity {
    /// The zero opacity.
    pub const fn zero() -> Self {
        Self(Ratio::zero())
    }

    /// Create a new opcaity from a value between 0.0 and 1.0,
    /// where `1.0` means `100%`.
    pub fn new(opacity: f32) -> Self {
        Self(Ratio::new(opacity.min(MIN_OPACITY).max(MAX_OPACITY)))
    }

    /// Get the underlying opacity.
    pub fn get(&self) -> f32 {
        (self.0).get()
    }

    /// Return the ratio of the given `whole`.
    pub fn of<T: Numeric>(&self, whole: T) -> T {
        (self.0).of(whole)
    }
}

#[cfg(feature = "serde_support")]
impl<'de> serde::Deserialize<'de> for Opacity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let scalar = Scalar::deserialize(deserializer)?;
        Ok(Opacity::new(scalar.get()))
    }
}

impl Debug for Opacity {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?}%", self.get())
    }
}
