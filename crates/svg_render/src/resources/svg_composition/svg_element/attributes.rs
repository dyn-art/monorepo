use dyn_composition::{
    modules::node::components::mixins::ContentType, utils::continuous_id::ContinuousId,
};
use serde::Serialize;
use specta::Type;

use super::mapper::map_path_commands_to_string;

#[derive(Debug, Serialize, Clone, Type)]
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
    PatternTransform {
        transform: SVGTransformAttribute,
    },
    D {
        d: SVGDAttribute,
    },
    #[serde(rename_all = "camelCase")]
    ClipPath {
        clip_path: ContinuousId,
    },
    Fill {
        fill: String,
    },
    ReferencedFill {
        id: ContinuousId,
    },
    Name {
        name: String,
    },
    #[serde(rename_all = "camelCase")]
    PatternUnits {
        pattern_units: SVGUnitsVariant,
    },
    #[serde(rename_all = "camelCase")]
    GradientUnits {
        gradient_units: SVGUnitsVariant,
    },
    Href {
        href: SVGHrefVariant,
    },
    #[serde(rename_all = "camelCase")]
    PreserveAspectRatio {
        preserve_aspect_ratio: String,
    },
    X1 {
        x1: f32,
    },
    Y1 {
        y1: f32,
    },
    X2 {
        x2: f32,
    },
    Y2 {
        y2: f32,
    },
    Offset {
        offset: f32,
    },
    #[serde(rename_all = "camelCase")]
    StopColor {
        stop_color: String,
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
            Self::PatternTransform { .. } => "patternTransform",
            Self::D { .. } => "d",
            Self::ClipPath { .. } => "clip-path",
            Self::Fill { .. } | Self::ReferencedFill { .. } => "fill",
            Self::Name { .. } => "name",
            Self::PatternUnits { .. } => "patternUnits",
            Self::GradientUnits { .. } => "gradientUnits",
            Self::Href { .. } => "href",
            Self::PreserveAspectRatio { .. } => "preserveAspectRatio",
            Self::X1 { .. } => "x1",
            Self::Y1 { .. } => "y1",
            Self::X2 { .. } => "x2",
            Self::Y2 { .. } => "y2",
            Self::Offset { .. } => "offset",
            Self::StopColor { .. } => "stop-color",
        }
    }

    pub fn to_svg_string(&self) -> String {
        match self {
            Self::Id { id } => id.to_string(),
            Self::Width { width, unit } => match unit {
                SVGMeasurementUnit::Pixel => width.to_string(),
                SVGMeasurementUnit::Percent => format!("{width}%"),
            },
            Self::Height { height, unit } => match unit {
                SVGMeasurementUnit::Pixel => height.to_string(),
                SVGMeasurementUnit::Percent => format!("{height}%"),
            },
            Self::Opacity { opacity } => opacity.to_string(),
            Self::Transform { transform } | Self::PatternTransform { transform } => match transform
            {
                SVGTransformAttribute::Matrix { a, b, c, d, tx, ty } => {
                    format!("matrix({a}, {b}, {c}, {d}, {tx}, {ty})")
                }
                SVGTransformAttribute::Rotate { rotation } => {
                    format!("rotate({rotation})")
                }
            },
            Self::D { d } => match d {
                SVGDAttribute::Meta { value } => map_path_commands_to_string(value),
                SVGDAttribute::String { value } => value.clone(),
            },
            Self::ClipPath { clip_path } => format!("url(#{clip_path})"),
            Self::Fill { fill } => fill.clone(),
            Self::ReferencedFill { id } => format!("url(#{id})"),
            Self::Name { name } => name.clone(),
            Self::PatternUnits {
                pattern_units: unit,
            }
            | Self::GradientUnits {
                gradient_units: unit,
            } => match unit {
                SVGUnitsVariant::ObjectBoundingBox => "objectBoundingBox".to_string(),
                SVGUnitsVariant::UserSpaceOnUse => "userSpaceOnUse".to_string(),
            },
            Self::Href { href } => match href {
                SVGHrefVariant::Base64 {
                    content,
                    content_type,
                } => format!("data:{};base64,{}", content_type.mime_type(), content),
                SVGHrefVariant::Url { url } => url.clone(),
            },
            Self::PreserveAspectRatio {
                preserve_aspect_ratio,
            } => preserve_aspect_ratio.clone(),
            Self::X1 { x1 } => x1.to_string(),
            Self::Y1 { y1 } => y1.to_string(),
            Self::X2 { x2 } => x2.to_string(),
            Self::Y2 { y2 } => y2.to_string(),
            Self::Offset { offset } => offset.to_string(),
            Self::StopColor { stop_color } => stop_color.clone(),
        }
    }
}

#[derive(Debug, Serialize, Clone, Type)]
#[serde(tag = "type")]
pub enum SVGDAttribute {
    Meta { value: Vec<SVGPathCommand> },
    String { value: String },
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
    Rotate {
        rotation: f32,
    },
}

#[derive(Debug, Serialize, Clone, Type)]
#[serde(tag = "type")]
pub enum SVGMeasurementUnit {
    Pixel,
    Percent,
}

#[derive(Debug, Serialize, Clone, Type)]
#[serde(tag = "type")]
pub enum SVGUnitsVariant {
    UserSpaceOnUse,
    ObjectBoundingBox,
}

#[derive(Debug, Serialize, Clone, Type)]
#[serde(tag = "type")]
pub enum SVGHrefVariant {
    #[serde(rename_all = "camelCase")]
    Base64 {
        content: String,
        content_type: ContentType,
    },
    Url {
        url: String,
    },
}
