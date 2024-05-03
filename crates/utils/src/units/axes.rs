/**
 * -----------------------------------------------------------------------------
 * This file includes code derived from the project typst/typst by @typst.
 * Project Repository: https://github.com/typst/typst/blob/main/crates/typst/src/layout/axes.rs
 *
 * Date of Import: 03 May 2024
 * -----------------------------------------------------------------------------
 * The code included in this file is licensed under the Apache License v2,
 * as per the original project by @typst.
 * For the license text, see: https://github.com/typst/typst/blob/main/LICENSE
 * -----------------------------------------------------------------------------
 */
use super::abs::Abs;
use std::{
    any::Any,
    fmt::{self, Debug, Formatter},
    ops::Deref,
};

/// A container with a horizontal and vertical component.
#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct Axes<T> {
    /// The horizontal component.
    pub x: T,
    /// The vertical component.
    pub y: T,
}

impl<T> Axes<T> {
    /// Create a new instance from the two components.
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// Convert from `&Axes<T>` to `Axes<&T>`.
    pub fn as_ref(&self) -> Axes<&T> {
        Axes {
            x: &self.x,
            y: &self.y,
        }
    }

    /// Convert from `&Axes<T>` to `Axes<&<T as Deref>::Target>`.
    pub fn as_deref(&self) -> Axes<&T::Target>
    where
        T: Deref,
    {
        Axes {
            x: &self.x,
            y: &self.y,
        }
    }

    /// Convert from `&mut Axes<T>` to `Axes<&mut T>`.
    pub fn as_mut(&mut self) -> Axes<&mut T> {
        Axes {
            x: &mut self.x,
            y: &mut self.y,
        }
    }
}

impl<T: Default> Axes<T> {
    /// Create a new instance with y set to its default value.
    pub fn with_x(x: T) -> Self {
        Self { x, y: T::default() }
    }

    /// Create a new instance with x set to its default value.
    pub fn with_y(y: T) -> Self {
        Self { x: T::default(), y }
    }
}

impl<T: Ord> Axes<T> {
    /// The component-wise minimum of this and another instance.
    pub fn min(self, other: Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    /// The component-wise minimum of this and another instance.
    pub fn max(self, other: Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }

    /// The minimum of width and height.
    pub fn min_by_side(self) -> T {
        self.x.min(self.y)
    }

    /// The minimum of width and height.
    pub fn max_by_side(self) -> T {
        self.x.max(self.y)
    }
}

impl<T> Debug for Axes<T>
where
    T: Debug + 'static,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if (&self.x as &dyn Any).is::<Abs>() {
            write!(f, "Size({:?}, {:?})", self.x, self.y)
        } else {
            write!(f, "Axes({:?}, {:?})", self.x, self.y)
        }
    }
}
