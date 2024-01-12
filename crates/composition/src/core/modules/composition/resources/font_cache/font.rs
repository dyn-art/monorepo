use serde::{Deserialize, Serialize};
use specta::Type;

/// Represents a font with specific characteristics.
/// Used for text rendering and styling.
#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Default, Debug, Type)]
pub struct FontMetadata {
    /// The name of the font, often used for display purposes.
    pub name: String,
    /// The font family to which this font belongs.
    pub family: String,
    /// The style of the font, such as italic or normal.
    pub style: FontStyle,
    /// The weight of the font, typically ranging from 100 (thin) to 900 (black).
    pub weight: u16,
}

/// Extends the `FontMetadata` structure with additional content for rendering.
#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Debug, Type)]
pub struct Font {
    /// The base font information.
    pub metadata: FontMetadata,
    /// The actual content of the font.
    pub content: FontContent,
}

/// Defines the content of a font.
#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Debug, Type)]
#[serde(tag = "type")]
pub enum FontContent {
    /// Font content stored as binary data.
    Binary { content: Vec<u8> },
    /// Font content referenced by a URL.
    ///
    /// This variant is only supported when the `resolve-url` feature is enabled.
    Url { url: String },
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
