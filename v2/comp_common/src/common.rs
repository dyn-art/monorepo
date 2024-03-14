use glam::{Mat3, Vec2, Vec4};

#[derive(Debug, Default, PartialEq, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct Degree(f32);

impl Degree {
    #[inline]
    pub fn new(value: f32) -> Self {
        Self(value.clamp(0.0, 360.0))
    }

    #[inline]
    pub fn get(&self) -> f32 {
        self.0
    }

    #[inline]
    pub fn to_radians(&self) -> f32 {
        self.0.to_radians()
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
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

impl Default for Percent {
    fn default() -> Self {
        Self::new(1.0)
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
pub struct Size(Vec2);

impl Size {
    #[inline]
    pub fn new(width: f32, height: f32) -> Self {
        Self(Vec2::new(width.max(0.0), height.max(0.0)))
    }

    #[inline]
    pub fn get_mut(&mut self) -> &mut Vec2 {
        &mut self.0
    }

    #[inline]
    pub fn get(&self) -> &Vec2 {
        &self.0
    }

    #[inline]
    pub fn width(&self) -> f32 {
        self.0.x
    }

    #[inline]
    pub fn height(&self) -> f32 {
        self.0.y
    }
}

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct CornerRadii(Vec4);

impl CornerRadii {
    #[inline]
    pub fn new(top_left: f32, top_right: f32, bottom_right: f32, bottom_left: f32) -> Self {
        Self(Vec4::new(
            top_left.max(0.0),
            top_right.max(0.0),
            bottom_right.max(0.0),
            bottom_left.max(0.0),
        ))
    }

    #[inline]
    pub fn get_mut(&mut self) -> &mut Vec4 {
        &mut self.0
    }

    #[inline]
    pub fn get(&self) -> &Vec4 {
        &self.0
    }

    #[inline]
    pub fn top_left(&self) -> f32 {
        self.0.x
    }

    #[inline]
    pub fn top_right(&self) -> f32 {
        self.0.y
    }

    #[inline]
    pub fn bottom_right(&self) -> f32 {
        self.0.z
    }

    #[inline]
    pub fn bottom_left(&self) -> f32 {
        self.0.w
    }
}

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

#[derive(Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct Opacity(Percent);

impl Opacity {
    #[inline]
    pub fn new(opacity: f32) -> Self {
        Self(Percent::new(opacity))
    }

    #[inline]
    pub fn get(&self) -> f32 {
        self.0.get()
    }
}

impl Default for Opacity {
    fn default() -> Self {
        Self(Percent(1.0))
    }
}

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
    pub physical_size: Size,
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

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(tag = "type")
)]
pub enum ImageScaleMode {
    /// Fills the area completely with the image.
    #[default]
    Fill,

    /// Fits the image within the area while maintaining its aspect ratio.
    Fit,

    /// Crops the image to fill the area.
    Crop {
        #[cfg_attr(feature = "serde_support", serde(default))]
        transform: Mat3,
    },

    /// Tiles the image within the area.
    #[serde(rename_all = "camelCase")]
    Tile {
        #[cfg_attr(feature = "serde_support", serde(default))]
        rotation: f32,
        scaling_factor: f32,
    },
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(tag = "type")
)]
pub enum GradientVariant {
    Linear {
        #[serde(default)]
        transform: Mat3,
    },
    Radial {
        #[serde(default)]
        transform: Mat3,
    },
}

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct GradientColorStop {
    /// The position of the color stop in the gradient, ranging from 0.0 to 1.0.
    pub position: Percent,

    /// The color of the stop.
    pub color: Color,

    /// The opacity of the stop.
    #[serde(default)]
    pub opacity: Percent,
}
