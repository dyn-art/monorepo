// Based on:
// https://github.com/typst/typst/blob/main/crates/typst/src/layout/angle.rs

use super::scalar::Scalar;
use super::Numeric;
use std::f32::consts::PI;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, Div, Mul, Neg};

const MIN_ANGLE_RAD: f32 = -2.0 * PI; // -360 degrees in radians
const MAX_ANGLE_RAD: f32 = 2.0 * PI; // 360 degrees in radians

/// An angle describing a rotation.
#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[cfg_attr(feature = "serde_support", derive(specta::Type))]
pub struct Angle(Scalar);

impl Angle {
    /// The zero angle.
    pub const fn zero() -> Self {
        Self(Scalar::ZERO)
    }

    /// Create an angle from a number of raw units.
    pub fn raw(raw: f32) -> Self {
        Self(Scalar::new(raw.max(MIN_ANGLE_RAD).min(MAX_ANGLE_RAD)))
    }

    /// Create an angle from a value in a unit.
    pub fn with_unit(val: f32, unit: AngleUnit) -> Self {
        Self::raw(val * unit.raw_scale())
    }

    /// Create an angle from a number of radians.
    pub fn rad(rad: f32) -> Self {
        Self::with_unit(rad, AngleUnit::Rad)
    }

    /// Create an angle from a number of degrees.
    pub fn deg(deg: f32) -> Self {
        Self::with_unit(deg, AngleUnit::Deg)
    }

    /// Get the value of this angle in raw units.
    pub fn to_raw(&self) -> f32 {
        (self.0).get()
    }

    /// Get the value of this angle in a unit.
    pub fn to_unit(&self, unit: AngleUnit) -> f32 {
        self.to_raw() / unit.raw_scale()
    }

    /// Converts this angle to radians.
    pub fn to_rad(&self) -> f32 {
        self.to_unit(AngleUnit::Rad)
    }

    /// Converts this angle to degrees.
    pub fn to_deg(&self) -> f32 {
        self.to_unit(AngleUnit::Deg)
    }
}

impl Numeric for Angle {
    fn zero() -> Self {
        Self::zero()
    }

    fn is_finite(self) -> bool {
        self.0.is_finite()
    }
}

impl Debug for Angle {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?}deg", self.to_deg())
    }
}

impl Neg for Angle {
    type Output = Self;

    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl Add for Angle {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

sub_impl!(Angle - Angle -> Angle);

impl Mul<f32> for Angle {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self(self.0 * other)
    }
}

impl Mul<Angle> for f32 {
    type Output = Angle;

    fn mul(self, other: Angle) -> Angle {
        other * self
    }
}

impl Div for Angle {
    type Output = f32;

    fn div(self, other: Self) -> f32 {
        self.to_raw() / other.to_raw()
    }
}

impl Div<f32> for Angle {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        Self(self.0 / other)
    }
}

assign_impl!(Angle += Angle);
assign_impl!(Angle -= Angle);
assign_impl!(Angle *= f32);
assign_impl!(Angle /= f32);

#[cfg(feature = "serde_support")]
impl serde::Serialize for Angle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        Scalar::new(self.to_deg()).serialize(serializer)
    }
}

#[cfg(feature = "serde_support")]
impl<'de> serde::Deserialize<'de> for Angle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let scalar = Scalar::deserialize(deserializer)?;
        Ok(Angle::deg(scalar.get()))
    }
}

/// Different units of angular measurement.
#[derive(Debug, Default, Eq, PartialEq, Hash, Clone, Copy)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum AngleUnit {
    /// Radians.
    #[default]
    Rad,
    /// Degrees.
    Deg,
}

impl AngleUnit {
    /// How many raw units correspond to a value of `1.0` in this unit.
    fn raw_scale(self) -> f32 {
        match self {
            Self::Rad => 1.0,
            Self::Deg => PI / 180.0,
        }
    }
}
