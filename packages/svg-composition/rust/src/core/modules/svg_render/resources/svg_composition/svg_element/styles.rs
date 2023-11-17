use serde::Serialize;
use specta::Type;

#[derive(Debug, Serialize, Clone, Type)]
// Using struct variants over tuples to use serde tag feature which enables efficient property access in TypeScript,
// allowing for faster and simpler type checks, e.g., `change.type === "Opacity"`
#[serde(tag = "type")]
pub enum SVGStyle {
    Display { display: SVGDisplayStyle },
}

impl SVGStyle {
    pub fn key(&self) -> &'static str {
        match self {
            Self::Display { .. } => "display",
        }
    }

    pub fn to_svg_string(&self) -> String {
        match self {
            Self::Display { display } => match display {
                SVGDisplayStyle::Block => String::from("block"),
                SVGDisplayStyle::None => String::from("none"),
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
