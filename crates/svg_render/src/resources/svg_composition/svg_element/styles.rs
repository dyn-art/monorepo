use serde::Serialize;
use specta::Type;

#[derive(Debug, Serialize, Clone, Type)]
// Using struct variants over tuples to use serde tag feature which enables efficient property access in TypeScript,
// allowing for faster and simpler type checks, e.g., `change.type === "Opacity"`
#[serde(tag = "type")]
pub enum SVGStyle {
    Display {
        display: SVGDisplayStyle,
    },
    BlendMode {
        #[serde(rename = "blendMode")]
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

    pub fn to_svg_string(&self) -> String {
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
}

#[derive(Debug, Serialize, Clone, Type)]
#[serde(tag = "type")]
pub enum SVGDisplayStyle {
    Block,
    None,
}

#[derive(Debug, Serialize, Clone, Type)]
#[serde(tag = "type")]
pub enum SVGBlendMode {
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
