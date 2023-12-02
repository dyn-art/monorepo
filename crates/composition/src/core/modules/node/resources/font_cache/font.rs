use serde::{Deserialize, Serialize};
use specta::Type;

/// Represents a font with specific characteristics.
/// Used for text rendering and styling.
#[derive(Serialize, Deserialize, Debug, Type)]
pub struct Font {
    /// The name of the font, often used for display purposes.
    pub name: String,
    /// The font family to which this font belongs.
    pub family: String,
    /// The style of the font, such as italic or normal.
    pub style: FontStyle,
    /// The weight of the font, typically ranging from 100 (thin) to 900 (black).
    weight: u16,
    /// A unique identifier for the font, often used for caching or referencing.
    hash: String,
}

/// Extends the `Font` structure with additional content for rendering.
/// Includes a preview URL and the font content itself.
#[derive(Serialize, Deserialize, Debug, Type)]
pub struct FontWithContent {
    /// The base font information.
    pub font: Font,
    /// An optional URL for a preview of the font.
    pub preview_url: Option<String>,
    /// The actual content of the font, either as a URL or binary data.
    pub content: FontContent,
}

/// Represents the content of a font, provided either as a URL or binary data.
#[derive(Serialize, Deserialize, Debug, Type)]
pub enum FontContent {
    /// A URL pointing to the font resource.
    Url(String),
    /// The binary data of the font, typically in a format like TTF or OTF.
    Binary(Vec<u8>),
}

/// Defines the style of a font, such as italic or normal.
#[derive(Serialize, Deserialize, Debug, Type)]
pub enum FontStyle {
    /// A style where the letters slant to the right.
    Italic,
    /// The standard, upright style of the font.
    Normal,
}
