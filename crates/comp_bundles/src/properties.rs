use dyn_attributed_string::{
    attrs::{Attrs, AttrsInterval},
    dyn_fonts_book::font::{
        info::FontFamily,
        variant::{FontStretch, FontStyle, FontWeight},
    },
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

impl TextAttributeInterval {
    pub fn to_attrs_interval(&self) -> AttrsInterval {
        AttrsInterval {
            start: self.start,
            stop: self.end,
            val: Attrs {
                font_id: None,
                font_family: self.attributes.font_family.clone(),
                font_style: self.attributes.font_style,
                font_stretch: self.attributes.font_stretch,
                font_weight: self.attributes.font_weight,
                font_size: self.attributes.font_size,
                small_caps: self.attributes.small_caps,
                apply_kerning: self.attributes.apply_kerning,
            },
        }
    }
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
