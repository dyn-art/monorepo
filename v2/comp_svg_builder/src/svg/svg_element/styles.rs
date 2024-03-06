#[derive(Debug, PartialEq, Clone)]
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
        blend_mode: SvgBlendModeStyle,
    },
    Opacity {
        opacity: f32,
    },
    Fill {
        fill: SvgFillStyle,
    },
    #[cfg_attr(feature = "serde_support", serde(rename_all = "camelCase"))]
    PointerEvents {
        pointer_events: SvgPointerEventsStyle,
    },
}

impl SvgStyle {
    pub fn key(&self) -> &'static str {
        match self {
            Self::Display { .. } => "display",
            Self::BlendMode { .. } => "mix-blend-mode",
            Self::Opacity { .. } => "opacity",
            Self::Fill { .. } => "fill",
            Self::PointerEvents { .. } => "pointer-events",
        }
    }

    pub fn to_svg_string(&self) -> String {
        match self {
            Self::Display { display } => match display {
                SvgDisplayStyle::Block => String::from("block"),
                SvgDisplayStyle::None => String::from("none"),
            },
            Self::BlendMode { blend_mode } => match blend_mode {
                SvgBlendModeStyle::Normal => String::from("normal"),
                SvgBlendModeStyle::Multiply => String::from("multiply"),
                SvgBlendModeStyle::Screen => String::from("screen"),
                SvgBlendModeStyle::Overlay => String::from("overlay"),
                SvgBlendModeStyle::Darken => String::from("darken"),
                SvgBlendModeStyle::Lighten => String::from("lighten"),
                SvgBlendModeStyle::ColorDodge => String::from("color-dodge"),
                SvgBlendModeStyle::ColorBurn => String::from("color-burn"),
                SvgBlendModeStyle::HardLight => String::from("hard-light"),
                SvgBlendModeStyle::SoftLight => String::from("soft-light"),
                SvgBlendModeStyle::Difference => String::from("difference"),
                SvgBlendModeStyle::Exclusion => String::from("exclusion"),
                SvgBlendModeStyle::Hue => String::from("hue"),
                SvgBlendModeStyle::Saturation => String::from("saturation"),
                SvgBlendModeStyle::Color => String::from("color"),
                SvgBlendModeStyle::Luminosity => String::from("luminosity"),
            },
            Self::Fill { fill } => match fill {
                SvgFillStyle::RGB { red, green, blue } => {
                    format!("rgb({red}, {green}, {blue})")
                }
                SvgFillStyle::RGBA {
                    red,
                    green,
                    blue,
                    alpha,
                } => {
                    format!("rgb({red}, {green}, {blue}, {alpha})")
                }
            },
            Self::Opacity { opacity } => opacity.to_string(),
            Self::PointerEvents { pointer_events } => match pointer_events {
                SvgPointerEventsStyle::All => "all".to_string(),
                SvgPointerEventsStyle::None => "none".to_string(),
            },
        }
    }

    pub fn to_tuple(&self) -> (&'static str, String) {
        (self.key(), self.to_svg_string())
    }
}

#[derive(Debug, Default, PartialEq, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum SvgDisplayStyle {
    #[default]
    Block,
    None,
}

#[derive(Debug, Default, PartialEq, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum SvgBlendModeStyle {
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

#[derive(Debug, PartialEq, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum SvgFillStyle {
    RGB {
        red: u8,
        green: u8,
        blue: u8,
    },
    RGBA {
        red: u8,
        green: u8,
        blue: u8,
        alpha: f32,
    },
}

#[derive(Debug, Default, PartialEq, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum SvgPointerEventsStyle {
    #[default]
    None,
    All,
}
