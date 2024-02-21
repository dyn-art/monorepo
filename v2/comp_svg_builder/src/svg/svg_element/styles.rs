#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, specta::Type),
    serde(tag = "type")
)]
pub enum SVGStyle {
    Display {
        display: SVGDisplayStyle,
    },
    #[cfg_attr(feature = "serde_support", serde(rename_all = "camelCase"))]
    BlendMode {
        blend_mode: SVGBlendMode,
    },
}

impl SVGStyle {
    pub fn key(&self) -> &'static str {
        match self {
            Self::Display { .. } => "display",
            Self::BlendMode { .. } => "mix-blend-mode",
        }
    }

    pub fn into_svg_string(&self) -> String {
        match self {
            Self::Display { display } => match display {
                SVGDisplayStyle::Block => String::from("block"),
                SVGDisplayStyle::None => String::from("none"),
            },
            Self::BlendMode { blend_mode } => match blend_mode {
                SVGBlendMode::Normal => String::from("normal"),
                SVGBlendMode::Multiply => String::from("multiply"),
                SVGBlendMode::Screen => String::from("screen"),
                SVGBlendMode::Overlay => String::from("overlay"),
                SVGBlendMode::Darken => String::from("darken"),
                SVGBlendMode::Lighten => String::from("lighten"),
                SVGBlendMode::ColorDodge => String::from("color-dodge"),
                SVGBlendMode::ColorBurn => String::from("color-burn"),
                SVGBlendMode::HardLight => String::from("hard-light"),
                SVGBlendMode::SoftLight => String::from("soft-light"),
                SVGBlendMode::Difference => String::from("difference"),
                SVGBlendMode::Exclusion => String::from("exclusion"),
                SVGBlendMode::Hue => String::from("hue"),
                SVGBlendMode::Saturation => String::from("saturation"),
                SVGBlendMode::Color => String::from("color"),
                SVGBlendMode::Luminosity => String::from("luminosity"),
            },
        }
    }

    pub fn into_tuple(&self) -> (&'static str, String) {
        (self.key(), self.into_svg_string())
    }
}

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(feature = "serde_support", derive(serde::Serialize, specta::Type))]
pub enum SVGDisplayStyle {
    #[default]
    Block,
    None,
}

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(feature = "serde_support", derive(serde::Serialize, specta::Type))]
pub enum SVGBlendMode {
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
