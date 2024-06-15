#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[cfg_attr(
    feature = "specta_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
#[cfg_attr(feature = "utoipa_support", derive(utoipa::ToSchema))]
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

    pub fn get_red(&self) -> u8 {
        self.0
    }

    pub fn get_green(&self) -> u8 {
        self.1
    }

    pub fn get_blue(&self) -> u8 {
        self.2
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::black()
    }
}
