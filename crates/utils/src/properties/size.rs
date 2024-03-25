use crate::units::{abs::Abs, ratio::Ratio};
use glam::Vec2;

const MIN_SIZE: f32 = 0.0;

/// An absolute size in 2D with a width and a height.
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
        Self(width.max(Abs::pt(MIN_SIZE)), height.max(Abs::pt(MIN_SIZE)))
    }

    pub fn from_vec2(vec2: Vec2) -> Self {
        Self(
            Abs::pt(vec2.x).min(Abs::pt(MIN_SIZE)),
            Abs::pt(vec2.y).min(Abs::pt(MIN_SIZE)),
        )
    }

    pub fn width(&self) -> f32 {
        self.0.to_pt()
    }

    pub fn set_width(&mut self, width: Abs) {
        self.0 = width
    }

    pub fn height(&self) -> f32 {
        self.1.to_pt()
    }

    pub fn set_height(&mut self, height: Abs) {
        self.1 = height
    }

    pub fn to_vec2(&self) -> Vec2 {
        Vec2::new(self.0.to_pt(), self.1.to_pt())
    }

    pub fn to_tuple(&self) -> (f32, f32) {
        (self.0.to_pt(), self.1.to_pt())
    }

    /// Converts to a ratio of width to height.
    pub fn aspect_ratio(&self) -> Ratio {
        Ratio::new(self.0 / self.1)
    }
}
