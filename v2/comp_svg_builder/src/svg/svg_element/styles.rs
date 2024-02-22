#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(tag = "type")
)]
pub enum SvgStyle {
    Display {
        display: SvgDisplayStyle,
    },
    #[cfg_attr(feature = "serde_support", serde(rename_all = "camelCase"))]
    BlendMode {
        blend_mode: SvgBlendMode,
    },
}

impl SvgStyle {
    pub fn key(&self) -> &'static str {
        match self {
            Self::Display { .. } => "display",
            Self::BlendMode { .. } => "mix-blend-mode",
        }
    }

    pub fn into_svg_string(&self) -> String {
        match self {
            Self::Display { display } => match display {
                SvgDisplayStyle::Block => String::from("block"),
                SvgDisplayStyle::None => String::from("none"),
            },
            Self::BlendMode { blend_mode } => match blend_mode {
                SvgBlendMode::Normal => String::from("normal"),
                SvgBlendMode::Multiply => String::from("multiply"),
                SvgBlendMode::Screen => String::from("screen"),
                SvgBlendMode::Overlay => String::from("overlay"),
                SvgBlendMode::Darken => String::from("darken"),
                SvgBlendMode::Lighten => String::from("lighten"),
                SvgBlendMode::ColorDodge => String::from("color-dodge"),
                SvgBlendMode::ColorBurn => String::from("color-burn"),
                SvgBlendMode::HardLight => String::from("hard-light"),
                SvgBlendMode::SoftLight => String::from("soft-light"),
                SvgBlendMode::Difference => String::from("difference"),
                SvgBlendMode::Exclusion => String::from("exclusion"),
                SvgBlendMode::Hue => String::from("hue"),
                SvgBlendMode::Saturation => String::from("saturation"),
                SvgBlendMode::Color => String::from("color"),
                SvgBlendMode::Luminosity => String::from("luminosity"),
            },
        }
    }

    pub fn into_tuple(&self) -> (&'static str, String) {
        (self.key(), self.into_svg_string())
    }
}

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum SvgDisplayStyle {
    #[default]
    Block,
    None,
}

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum SvgBlendMode {
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
