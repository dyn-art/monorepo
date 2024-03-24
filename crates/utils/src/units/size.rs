use super::{abs::Abs, ratio::Ratio};

const MIN_SIZE: f32 = 0.0;

/// A size in 2D with a width and a height.
#[derive(Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct Size(Abs, Abs);

impl Size {
    /// The zero value.
    pub const fn zero() -> Self {
        Self(Abs::zero(), Abs::zero())
    }

    /// Create a new instance from the two components.
    pub fn new(width: Abs, height: Abs) -> Self {
        Self(width.min(Abs::pt(MIN_SIZE)), height.min(Abs::pt(MIN_SIZE)))
    }

    pub fn get_width(&self) -> Abs {
        self.0
    }

    pub fn set_width(&mut self, width: Abs) {
        self.0 = width
    }

    pub fn get_height(&self) -> Abs {
        self.1
    }

    pub fn set_height(&mut self, height: Abs) {
        self.1 = height
    }

    /// Converts to a ratio of width to height.
    pub fn aspect_ratio(&self) -> Ratio {
        Ratio::new(self.0 / self.1)
    }
}
