use dyn_attributed_string::dyn_fonts_book::font::{
    info::FontFamily,
    variant::{FontStretch, FontStyle, FontWeight},
};
use dyn_utils::{properties::size::Size, units::abs::Abs};
use glam::Vec2;

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

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct TextAttributeInterval {
    pub start: usize,
    pub end: usize,
    pub attributes: TextAttributes,
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct TextAttributes {
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub font_family: Option<FontFamily>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub font_style: Option<FontStyle>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub font_stretch: Option<FontStretch>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub font_weight: Option<FontWeight>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub font_size: Option<Abs>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub small_caps: Option<bool>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub apply_kerning: Option<bool>,
}
