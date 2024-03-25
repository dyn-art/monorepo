#[macro_use]
pub mod macros;
pub mod abs;
pub mod angle;
pub mod em;
pub mod ratio;
pub mod scalar;

use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// A numeric type.
pub trait Numeric:
    Sized
    + Debug
    + Copy
    + PartialEq
    + Neg<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<f32, Output = Self>
    + Div<f32, Output = Self>
{
    /// The identity element for addition.
    fn zero() -> Self;

    /// Whether `self` is zero.
    fn is_zero(self) -> bool {
        self == Self::zero()
    }

    /// Whether `self` consists only of finite parts.
    fn is_finite(self) -> bool;
}
