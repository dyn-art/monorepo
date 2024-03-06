use bevy_ecs::entity::Entity;
use glam::{Vec2, Vec4};

#[derive(Debug, Default, PartialEq, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct Percent(f32);

impl Percent {
    #[inline]
    pub fn new(value: f32) -> Self {
        Self(value.clamp(0.0, 1.0))
    }

    #[inline]
    pub fn get(&self) -> f32 {
        self.0
    }
}

#[derive(Debug, Default, PartialEq, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct Pixel(pub f32);

#[derive(Debug, Default, PartialEq, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct Inch(pub f32);

#[derive(Debug, Default, PartialEq, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct Centimeter(pub f32);

#[derive(Debug, Default, PartialEq, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct Millimeter(pub f32);

// https://oreillymedia.github.io/Using_Svg/guide/units.html
#[derive(Debug, PartialEq, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(tag = "type")
)]
pub enum MeasurementUnit {
    Pixels { pixels: Pixel },
    Percent { percent: Percent },
    Inch { inch: Inch },
    Centimeter { centimeter: Centimeter },
    Millimeter { millimeter: Millimeter },
}

impl MeasurementUnit {
    pub fn to_pixel(&self, parent_pixels: f32) -> Pixel {
        match self {
            Self::Percent { percent } => Pixel(parent_pixels * percent.get()),
            Self::Pixels { pixels } => *pixels,
            Self::Inch { inch } => Pixel(inch.0 * 96.0),
            Self::Centimeter { centimeter } => Pixel(centimeter.0 * 37.795),
            Self::Millimeter { millimeter } => Pixel(millimeter.0 * 3.7795),
        }
    }
}

impl Default for MeasurementUnit {
    fn default() -> Self {
        Self::Pixels {
            pixels: Pixel::default(),
        }
    }
}

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct Size(pub Vec2);

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct CornerRadii(pub Vec4);

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum BlendMode {
    #[default]
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    Hue,
    Saturation,
    Color,
    Luminosity,
}

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum Visibility {
    #[default]
    Visible,
    Hidden,
}

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct Opacity(pub Percent);

#[derive(Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Default for Color {
    fn default() -> Self {
        Self::black()
    }
}

impl Color {
    /// Creates a new `Color` instance from RGB components.
    #[inline]
    pub fn new_rgb(red: u8, green: u8, blue: u8) -> Color {
        Color { red, green, blue }
    }

    /// Returns a color instance set to black.
    #[inline]
    pub fn black() -> Color {
        Color::new_rgb(0, 0, 0)
    }

    /// Returns a color instance set to white.
    #[inline]
    pub fn white() -> Color {
        Color::new_rgb(255, 255, 255)
    }
}

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct Viewport {
    pub physical_position: Vec2,
    pub physical_size: Vec2,
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct Stroke {
    pub fill: Fill,
    pub width: f32,
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct Fill {
    pub paint: Entity,
    pub blend_mode: BlendMode,
    pub opacity: Opacity,
}

/// A styled text segment.
#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct TextSpan {
    /// Text content.
    pub text: String,
    /// Font metadata.
    pub font: FontMetadata,
    /// Style properties.
    pub style: TextStyle,
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct FontMetadata {
    /// The font family to which this font belongs.
    pub family: String,
    /// The style of the font, such as italic or normal.
    pub style: FontStyle,
    /// The weight of the font, typically ranging from 100 (thin) to 900 (black).
    pub weight: u16,
}

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum FontStyle {
    /// A face that is neither italic not obliqued.
    #[default]
    Normal,
    /// A form that is generally cursive in nature.
    Italic,
    /// A typically-sloped version of the regular face.
    Oblique,
}

/// Style properties for a text segment.
#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct TextStyle {
    /// Glyph height in pixels, may scale with window.
    pub font_size: f32,
    /// Character spacing.
    pub letter_spacing: LetterSpacing,
    /// Line spacing.
    pub line_height: LineHeight,
}

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum HorizontalTextAlignment {
    #[default]
    Left,
    Center,
    Right,
    Justified,
}

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum VerticalTextAlignment {
    Top,
    #[default]
    Center,
    Bottom,
    Justified,
}

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum LetterSpacing {
    #[default]
    Auto,
    Fixed(MeasurementUnit),
}

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum LineHeight {
    #[default]
    Auto,
    Fixed(MeasurementUnit),
}

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum BreakLineOn {
    #[default]
    WordBoundary,
    AnyCharacter,
    NoWrap,
}
