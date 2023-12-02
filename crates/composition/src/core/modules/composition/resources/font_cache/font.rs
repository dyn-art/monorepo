use serde::{Deserialize, Serialize};
use specta::Type;

/// Represents a font with specific characteristics.
/// Used for text rendering and styling.
#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Default, Debug, Type)]
pub struct Font {
    /// The name of the font, often used for display purposes.
    pub name: String,
    /// The font family to which this font belongs.
    pub family: String,
    /// The style of the font, such as italic or normal.
    pub style: FontStyle,
    /// The weight of the font, typically ranging from 100 (thin) to 900 (black).
    pub weight: u16,
}

/// Extends the `Font` structure with additional content for rendering.
/// Includes a preview URL and the font content itself.
#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Debug, Type)]
pub struct FontWithContent {
    /// The base font information.
    pub font: Font,
    /// The actual content of the font, either as a URL or binary data.
    pub content: Vec<u8>,
    /// Optional hash for uniquely identifying a font.
    /// If `None`, the font will be hashed using Rust's `DefaultHasher` when needed.
    pub hash: Option<u64>,
}

/// Defines the style of a font, such as italic or normal.
#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Default, Debug, Type)]
pub enum FontStyle {
    /// The standard, upright style of the font.
    #[default]
    Normal,
    /// A style where the letters slant to the right.
    Italic,
}
