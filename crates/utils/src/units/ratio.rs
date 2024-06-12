/**
 * -----------------------------------------------------------------------------
 * This file includes code derived from the project typst/typst by @typst.
 * Project Repository: https://github.com/typst/typst/blob/main/crates/typst/src/layout/ratio.rs
 *
 * Date of Import: 03 May 2024
 * -----------------------------------------------------------------------------
 * The code included in this file is licensed under the Apache License v2,
 * as per the original project by @typst.
 * For the license text, see: https://github.com/typst/typst/blob/main/LICENSE
 * -----------------------------------------------------------------------------
 */
use super::scalar::Scalar;
use super::Numeric;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, Div, Mul, Neg};

/// A ratio of a whole.
///
/// Written as a number, followed by a percent sign.
#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[cfg_attr(
    feature = "specta_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
#[cfg_attr(feature = "utoipa_support", derive(utoipa::ToSchema))]
pub struct Ratio(Scalar);

impl Ratio {
    /// A ratio of `0%` represented as `0.0`.
    pub const fn zero() -> Self {
        Self(Scalar::ZERO)
    }

    /// A ratio of `100%` represented as `1.0`.
    pub const fn one() -> Self {
        Self(Scalar::ONE)
    }

    /// Create a new ratio from a value, where `1.0` means `100%`.
    pub fn new(ratio: f32) -> Self {
        Self(Scalar::new(ratio))
    }

    /// Get the underlying ratio.
    pub fn get(&self) -> f32 {
        (self.0).get()
    }

    /// Return the ratio of the given `whole`.
    pub fn of<T: Numeric>(&self, whole: T) -> T {
        let resolved = whole * self.get();
        if resolved.is_finite() {
            resolved
        } else {
            T::zero()
        }
    }
}

impl Debug for Ratio {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?}%", self.get())
    }
}

impl Neg for Ratio {
    type Output = Self;

    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl Add for Ratio {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

sub_impl!(Ratio - Ratio -> Ratio);

impl Mul for Ratio {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self(self.0 * other.0)
    }
}

impl Mul<f32> for Ratio {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self(self.0 * other)
    }
}

impl Mul<Ratio> for f32 {
    type Output = Ratio;

    fn mul(self, other: Ratio) -> Ratio {
        other * self
    }
}

impl Div for Ratio {
    type Output = f32;

    fn div(self, other: Self) -> f32 {
        self.get() / other.get()
    }
}

impl Div<f32> for Ratio {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        Self(self.0 / other)
    }
}

impl Div<Ratio> for f32 {
    type Output = Self;

    fn div(self, other: Ratio) -> Self {
        self / other.get()
    }
}

assign_impl!(Ratio += Ratio);
assign_impl!(Ratio -= Ratio);
assign_impl!(Ratio *= Ratio);
assign_impl!(Ratio *= f32);
assign_impl!(Ratio /= f32);
