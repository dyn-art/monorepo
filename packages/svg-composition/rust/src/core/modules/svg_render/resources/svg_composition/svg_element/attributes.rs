use dyn_composition::core::utils::continuous_id::ContinuousId;
use serde::Serialize;
use specta::Type;

#[derive(Debug, Serialize, Clone, Type)]
#[serde(tag = "type")]
pub enum SVGMeasurementUnit {
    Pixel,
    Percent,
}

#[derive(Debug, Serialize, Clone, Type)]
// Using struct variants over tuples to use serde tag feature which enables efficient property access in TypeScript,
// allowing for faster and simpler type checks, e.g., `change.type === "Width"`
#[serde(tag = "type")]
pub enum SVGAttribute {
    Id {
        id: ContinuousId,
    },
    Width {
        width: f32,
        unit: SVGMeasurementUnit,
    },
    Height {
        height: f32,
        unit: SVGMeasurementUnit,
    },
    Opacity {
        opacity: f32,
    },
    Transform {
        transform: SVGTransformAttribute,
    },
    D {
        d: Vec<SVGPathCommand>,
    },
    ClipPath {
        #[serde(rename = "clipPath")]
        clip_path: ContinuousId,
    },
    Fill {
        fill: String,
    },
    Name {
        name: String,
    },
}

impl SVGAttribute {
    pub fn key(&self) -> &'static str {
        match self {
            Self::Id { .. } => "id",
            Self::Width { .. } => "width",
            Self::Height { .. } => "height",
            Self::Opacity { .. } => "opacity",
            Self::Transform { .. } => "transform",
            Self::D { .. } => "d",
            Self::ClipPath { .. } => "clip-path",
            Self::Fill { .. } => "fill",
            Self::Name { .. } => "name",
        }
    }

    pub fn to_svg_string(&self) -> String {
        match self {
            Self::Id { id } => id.to_string(),
            Self::Width { width, unit } => match unit {
                SVGMeasurementUnit::Pixel => width.to_string(),
                SVGMeasurementUnit::Percent => format!("{width}%")
            },
            Self::Height { height , unit} => match unit {
                SVGMeasurementUnit::Pixel => height.to_string(),
                SVGMeasurementUnit::Percent => format!("{height}%")
            },
            Self::Opacity { opacity } => opacity.to_string(),
            Self::Transform { transform } => match transform {
                SVGTransformAttribute::Matrix { a, b, c, d, tx, ty } => {
                    format!("matrix({a}, {b}, {c}, {d}, {tx}, {ty})")
                }
            },
            Self::D { d } => d
                .iter()
                .map(|command| match command {
                    SVGPathCommand::MoveTo { x, y } => format!("M{x} {y}"),
                    SVGPathCommand::LineTo { x, y } => format!("L{x} {y}"),
                    SVGPathCommand::CurveTo {
                        cx1,
                        cy1,
                        cx2,
                        cy2,
                        x,
                        y,
                    } => {
                        format!("C{cx1} {cy1} {cx2} {cy2} {x} {y}")
                    }
                    SVGPathCommand::ArcTo {
                        rx,
                        ry,
                        x_axis_rotation,
                        large_arc_flag,
                        sweep_flag,
                        x,
                        y,
                    } => {
                        let parsed_large_arc_flag = *large_arc_flag as u8;
                        let parsed_sweep_flag = *sweep_flag as u8;
                        format!(
                        "A{rx} {ry} {x_axis_rotation} {parsed_large_arc_flag} {parsed_sweep_flag} {x} {y}"
                    )},
                    SVGPathCommand::ClosePath => "Z".to_string(),
                })
                .collect::<Vec<_>>()
                .join(" "),
            Self::ClipPath { clip_path } => format!("url(#{clip_path})"),
            Self::Fill { fill } => fill.clone(),
            Self::Name { name } => name.clone(),
        }
    }
}

// https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/d
#[derive(Debug, Serialize, Clone, Type)]
#[serde(tag = "type")]
pub enum SVGPathCommand {
    MoveTo {
        x: f32,
        y: f32,
    },
    LineTo {
        x: f32,
        y: f32,
    },
    CurveTo {
        cx1: f32,
        cy1: f32,
        cx2: f32,
        cy2: f32,
        x: f32,
        y: f32,
    },
    ArcTo {
        rx: f32,
        ry: f32,
        #[serde(rename = "xAxisRotation")]
        x_axis_rotation: f32,
        #[serde(rename = "largeArcFlag")]
        large_arc_flag: bool,
        #[serde(rename = "sweepFlag")]
        sweep_flag: bool,
        x: f32,
        y: f32,
    },
    ClosePath,
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/transform-function
#[derive(Debug, Serialize, Clone, Type)]
#[serde(tag = "type")]
pub enum SVGTransformAttribute {
    Matrix {
        a: f32,
        b: f32,
        c: f32,
        d: f32,
        tx: f32,
        ty: f32,
    },
}
