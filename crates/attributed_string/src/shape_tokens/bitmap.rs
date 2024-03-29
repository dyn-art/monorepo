use super::ShapeToken;
use dyn_utils::units::abs::Abs;
use glam::Vec2;
use std::ops::Range;

/// Represents an inline bitmap image or emoji.
#[derive(Debug, Clone)]
pub struct BitmapToken {
    pub range: Range<usize>,
    /// Image data or reference.
    pub image: (),
    /// Size of the bitmap image.
    pub size: Vec2,
    /// Alt text for accessibility and text fallback.
    pub alt_text: String,
}

impl ShapeToken for BitmapToken {
    fn get_range(&self) -> &Range<usize> {
        &self.range
    }

    fn get_width(&self) -> Abs {
        Abs::pt(self.size.x)
    }

    fn get_height(&self) -> Abs {
        Abs::pt(self.size.y)
    }
}
