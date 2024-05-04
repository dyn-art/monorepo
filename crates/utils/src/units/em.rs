/**
 * -----------------------------------------------------------------------------
 * This file includes code derived from the project typst/typst by @typst.
 * Project Repository: https://github.com/typst/typst/blob/main/crates/typst/src/layout/em.rs
 *
 * Date of Import: 03 May 2024
 * -----------------------------------------------------------------------------
 * The code included in this file is licensed under the Apache License v2,
 * as per the original project by @typst.
 * For the license text, see: https://github.com/typst/typst/blob/main/LICENSE
 * -----------------------------------------------------------------------------
 */
use super::abs::Abs;
use super::scalar::Scalar;
use super::Numeric;
use std::fmt::{Debug, Formatter};
use std::iter::Sum;
use std::ops::{Add, Div, Mul, Neg};

/// A length that is relative to the font size.
///
/// `1em` is the same as the font size.
#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct Em(Scalar);

impl Em {
    /// The zero em length.
    pub const fn zero() -> Self {
        Self(Scalar::ZERO)
    }

    /// Create a font-relative length.
    pub fn new(em: f32) -> Self {
        Self(Scalar::new(em))
    }

    /// Create an em length from font units at the given units per em.
    pub fn from_units(units: impl Into<f32>, units_per_em: f32) -> Self {
        Self(Scalar::new(units.into() / units_per_em))
    }

    /// The number of em units.
    pub fn get(&self) -> f32 {
        (self.0).get()
    }

    /// Convert to an absolute length at the given font size.
    pub fn at(&self, font_size: Abs) -> Abs {
        let resolved = font_size * self.get();
        if resolved.is_finite() {
            resolved
        } else {
            Abs::zero()
        }
    }
}

impl Numeric for Em {
    fn zero() -> Self {
        Self::zero()
    }

    fn is_finite(self) -> bool {
        self.0.is_finite()
    }
}

impl Debug for Em {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?}em", self.get())
    }
}

impl Neg for Em {
    type Output = Self;

    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl Add for Em {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

sub_impl!(Em - Em -> Em);

impl Mul<f32> for Em {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self(self.0 * other)
    }
}

impl Mul<Em> for f32 {
    type Output = Em;

    fn mul(self, other: Em) -> Em {
        other * self
    }
}

impl Div<f32> for Em {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        Self(self.0 / other)
    }
}

impl Div for Em {
    type Output = f32;

    fn div(self, other: Self) -> f32 {
        self.get() / other.get()
    }
}

impl Sum for Em {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self(iter.map(|s| s.0).sum())
    }
}

assign_impl!(Em += Em);
assign_impl!(Em -= Em);
assign_impl!(Em *= f32);
assign_impl!(Em /= f32);
