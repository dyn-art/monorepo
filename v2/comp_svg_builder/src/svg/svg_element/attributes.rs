use super::SvgElementId;

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(tag = "type")
)]
pub enum SvgAttribute {
    Id {
        id: SvgElementId,
    },
    Name {
        name: String,
    },
    Width {
        width: f32,
        unit: SvgMeasurementUnit,
    },
    Height {
        height: f32,
        unit: SvgMeasurementUnit,
    },
    Opacity {
        opacity: f32,
    },
    Transform {
        transform: SvgTransformAttribute,
    },
    #[cfg_attr(feature = "serde_support", serde(rename_all = "camelCase"))]
    PatternTransform {
        pattern_transform: SvgTransformAttribute,
    },
    D {
        d: String,
    },
    #[cfg_attr(feature = "serde_support", serde(rename_all = "camelCase"))]
    ClipPath {
        clip_path: SvgElementId,
    },
    Fill {
        fill: String,
    },
    ReferencedFill {
        id: SvgElementId,
    },
    #[cfg_attr(feature = "serde_support", serde(rename_all = "camelCase"))]
    PatternUnits {
        pattern_units: SvgUnitsVariant,
    },
    #[cfg_attr(feature = "serde_support", serde(rename_all = "camelCase"))]
    GradientUnits {
        gradient_units: SvgUnitsVariant,
    },
    Href {
        href: SvgHrefVariant,
    },
    #[cfg_attr(feature = "serde_support", serde(rename_all = "camelCase"))]
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
    #[cfg_attr(feature = "serde_support", serde(rename_all = "camelCase"))]
    StopColor {
        stop_color: String,
    },
    #[cfg_attr(feature = "serde_support", serde(rename_all = "camelCase"))]
    PointerEvents {
        pointer_events: SvgPointerEventsVariants,
    },
}

impl SvgAttribute {
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
            Self::PointerEvents { .. } => "pointer-events",
        }
    }

    pub fn to_svg_string(&self) -> String {
        match self {
            Self::Id { id } => id.to_string(),
            Self::Width { width, unit } => match unit {
                SvgMeasurementUnit::Pixel => width.to_string(),
                SvgMeasurementUnit::Percent => format!("{width}%"),
            },
            Self::Height { height, unit } => match unit {
                SvgMeasurementUnit::Pixel => height.to_string(),
                SvgMeasurementUnit::Percent => format!("{height}%"),
            },
            Self::Opacity { opacity } => opacity.to_string(),
            Self::Transform { transform }
            | Self::PatternTransform {
                pattern_transform: transform,
            } => match transform {
                SvgTransformAttribute::Matrix { a, b, c, d, tx, ty } => {
                    format!("matrix({a}, {b}, {c}, {d}, {tx}, {ty})")
                }
                SvgTransformAttribute::Rotate { rotation } => {
                    format!("rotate({rotation})")
                }
            },
            Self::D { d } => d.clone(),
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
                SvgUnitsVariant::ObjectBoundingBox => String::from("objectBoundingBox"),
                SvgUnitsVariant::UserSpaceOnUse => String::from("userSpaceOnUse"),
            },
            Self::Href { href } => match href {
                // SvgHrefVariant::Base64 {
                //     content,
                //     content_type,
                // } => format!("data:{};base64,{}", content_type.mime_type(), content),
                SvgHrefVariant::Base64 { .. } => String::from("todo"),
                SvgHrefVariant::Url { url } => url.clone(),
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
            Self::PointerEvents { pointer_events } => match pointer_events {
                SvgPointerEventsVariants::All => "all".to_string(),
                SvgPointerEventsVariants::None => "none".to_string(),
            },
        }
    }

    pub fn to_tuple(&self) -> (&'static str, String) {
        (self.key(), self.to_svg_string())
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(tag = "type")
)]
pub enum SvgTransformAttribute {
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

#[derive(Debug, Default, PartialEq, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum SvgMeasurementUnit {
    #[default]
    Pixel,
    Percent,
}

#[derive(Debug, Default, PartialEq, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum SvgUnitsVariant {
    #[default]
    UserSpaceOnUse,
    ObjectBoundingBox,
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum SvgHrefVariant {
    #[serde(rename_all = "camelCase")]
    Base64 {
        content: String,
        // content_type: ContentType, // TODO: Add ContentType struct in comp_types
    },
    Url {
        url: String,
    },
}

#[derive(Debug, Default, PartialEq, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum SvgPointerEventsVariants {
    #[default]
    None,
    All,
}
