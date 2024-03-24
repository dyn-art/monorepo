#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct Color(u8, u8, u8);

impl Color {
    /// Creates a new `Color` instance from RGB components.
    pub fn new_rgb(red: u8, green: u8, blue: u8) -> Self {
        Self(red, green, blue)
    }

    /// Returns a color instance set to black.
    pub fn black() -> Color {
        Color::new_rgb(0, 0, 0)
    }

    /// Returns a color instance set to white.
    pub fn white() -> Color {
        Color::new_rgb(255, 255, 255)
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::black()
    }
}
