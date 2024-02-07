/// Checks that the current number is > 0.
pub(crate) trait IsValidLength {
    /// Checks that the current number is > 0.
    fn is_valid_length(&self) -> bool;
}

impl IsValidLength for f32 {
    #[inline]
    fn is_valid_length(&self) -> bool {
        *self > 0.0 && self.is_finite()
    }
}

impl IsValidLength for f64 {
    #[inline]
    fn is_valid_length(&self) -> bool {
        *self > 0.0 && self.is_finite()
    }
}

/// A bounding box calculator.
#[derive(Clone, Copy, Debug)]
pub struct BBox {
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
}

impl Default for BBox {
    fn default() -> Self {
        Self {
            left: f32::MAX,
            top: f32::MAX,
            right: f32::MIN,
            bottom: f32::MIN,
        }
    }
}

impl BBox {
    /// Checks if the bounding box is default, i.e. invalid.
    pub fn is_default(&self) -> bool {
        self.left == f32::MAX
            && self.top == f32::MAX
            && self.right == f32::MIN
            && self.bottom == f32::MIN
    }

    /// Expand the bounding box to the specified bounds.
    #[must_use]
    pub fn expand(&self, r: impl Into<Self>) -> Self {
        self.expand_impl(r.into())
    }

    fn expand_impl(&self, r: Self) -> Self {
        Self {
            left: self.left.min(r.left),
            top: self.top.min(r.top),
            right: self.right.max(r.right),
            bottom: self.bottom.max(r.bottom),
        }
    }
}
