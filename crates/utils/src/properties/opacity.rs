use crate::units::{ratio::Ratio, scalar::Scalar, Numeric};
use std::fmt::{Debug, Formatter};

const MIN_OPACITY: f32 = 0.0;
const MAX_OPACITY: f32 = 1.0;

/// An opacity.
#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[cfg_attr(feature = "specta_support", derive(serde::Serialize, specta::Type))]
#[cfg_attr(feature = "utoipa_support", derive(utoipa::ToSchema))]
pub struct Opacity(Ratio);

impl Opacity {
    /// The 0% opacity.
    pub const fn zero() -> Self {
        Self(Ratio::zero())
    }

    /// The 100% opacity.
    pub const fn one() -> Self {
        Self(Ratio::one())
    }

    /// Create a new opcaity from a value between 0.0 and 1.0,
    /// where `1.0` means `100%`.
    pub fn new(opacity: f32) -> Self {
        Self(Ratio::new(opacity.max(MIN_OPACITY).min(MAX_OPACITY)))
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

impl Default for Opacity {
    fn default() -> Self {
        Opacity::one()
    }
}

#[cfg(feature = "specta_support")]
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
