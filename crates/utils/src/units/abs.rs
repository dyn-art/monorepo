// Based on:
// https://github.com/typst/typst/blob/main/crates/typst/src/layout/abs.rs

use super::scalar::Scalar;
use super::Numeric;
use std::fmt::{self, Debug, Formatter};
use std::iter::Sum;
use std::ops::{Add, Div, Mul, Neg, Rem};

/// An absolute length.
#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct Abs(Scalar);

impl Abs {
    /// The zero length.
    pub const fn zero() -> Self {
        Self(Scalar::ZERO)
    }

    /// Create an absolute length from a number of raw units.
    pub fn raw(raw: f32) -> Self {
        Self(Scalar::new(raw))
    }

    /// Create an absolute length from a value in a unit.
    pub fn with_unit(val: f32, unit: AbsUnit) -> Self {
        Self(Scalar::new(val * unit.raw_scale()))
    }

    /// Create an absolute length from a number of points.
    pub fn pt(pt: f32) -> Self {
        Self::with_unit(pt, AbsUnit::Pt)
    }

    /// Create an absolute length from a number of millimeters.
    pub fn mm(mm: f32) -> Self {
        Self::with_unit(mm, AbsUnit::Mm)
    }

    /// Create an absolute length from a number of centimeters.
    pub fn cm(cm: f32) -> Self {
        Self::with_unit(cm, AbsUnit::Cm)
    }

    /// Create an absolute length from a number of inches.
    pub fn inches(inches: f32) -> Self {
        Self::with_unit(inches, AbsUnit::In)
    }

    /// Get the value of this absolute length in raw units.
    pub fn to_raw(&self) -> f32 {
        (self.0).get()
    }

    /// Get the value of this absolute length in a unit.
    pub fn to_unit(&self, unit: AbsUnit) -> f32 {
        self.to_raw() / unit.raw_scale()
    }

    /// Convert this to a number of points.
    pub fn to_pt(&self) -> f32 {
        self.to_unit(AbsUnit::Pt)
    }

    /// Convert this to a number of millimeters.
    pub fn to_mm(&self) -> f32 {
        self.to_unit(AbsUnit::Mm)
    }

    /// Convert this to a number of centimeters.
    pub fn to_cm(&self) -> f32 {
        self.to_unit(AbsUnit::Cm)
    }

    /// Convert this to a number of inches.
    pub fn to_inches(&self) -> f32 {
        self.to_unit(AbsUnit::In)
    }
}

impl Numeric for Abs {
    fn zero() -> Self {
        Self::zero()
    }

    fn is_finite(self) -> bool {
        self.0.is_finite()
    }
}

impl Debug for Abs {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}pt", self.to_pt())
    }
}

impl Neg for Abs {
    type Output = Self;

    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl Add for Abs {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

sub_impl!(Abs - Abs -> Abs);

impl Mul<f32> for Abs {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self(self.0 * other)
    }
}

impl Mul<Abs> for f32 {
    type Output = Abs;

    fn mul(self, other: Abs) -> Abs {
        other * self
    }
}

impl Div<f32> for Abs {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        Self(self.0 / other)
    }
}

impl Div for Abs {
    type Output = f32;

    fn div(self, other: Self) -> f32 {
        self.to_raw() / other.to_raw()
    }
}

impl Rem for Abs {
    type Output = Self;

    fn rem(self, other: Self) -> Self::Output {
        Self(self.0 % other.0)
    }
}

impl Sum for Abs {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self(iter.map(|s| s.0).sum())
    }
}

impl<'a> Sum<&'a Self> for Abs {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        Self(iter.map(|s| s.0).sum())
    }
}

assign_impl!(Abs += Abs);
assign_impl!(Abs -= Abs);
assign_impl!(Abs *= f32);
assign_impl!(Abs /= f32);

/// Different units of absolute measurement.
#[derive(Default, Debug, Eq, PartialEq, Hash, Clone, Copy)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum AbsUnit {
    /// Points.
    #[default]
    Pt,
    /// Millimeters.
    Mm,
    /// Centimeters.
    Cm,
    /// Inches.
    In,
}

impl AbsUnit {
    /// How many raw units correspond to a value of `1.0` in this unit.
    fn raw_scale(self) -> f32 {
        match self {
            AbsUnit::Pt => 1.0,
            AbsUnit::Mm => 2.83465,
            AbsUnit::Cm => 28.3465,
            AbsUnit::In => 72.0,
        }
    }
}
