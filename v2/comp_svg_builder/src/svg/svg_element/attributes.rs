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
    Class {
        class: String,
    },
    Width {
        width: f32,
        unit: SvgMeasurementUnit,
    },
    Height {
        height: f32,
        unit: SvgMeasurementUnit,
    },
    Transform {
        transform: SvgTransformAttribute,
    },
    #[cfg_attr(feature = "serde_support", serde(rename_all = "camelCase"))]
    PatternTransform {
        pattern_transform: SvgTransformAttribute,
    },
    D {
        d: SvgPathAttribute,
    },
    #[cfg_attr(feature = "serde_support", serde(rename_all = "camelCase"))]
    ClipPath {
        clip_path: SvgElementId,
    },
    Fill {
        fill: SvgAttributeColor,
    },
    #[cfg_attr(feature = "serde_support", serde(rename_all = "camelCase"))]
    PatternUnits {
        pattern_units: SvgUnits,
    },
    #[cfg_attr(feature = "serde_support", serde(rename_all = "camelCase"))]
    GradientUnits {
        gradient_units: SvgUnits,
    },
    Href {
        href: SvgHrefAttribute,
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
        stop_color: SvgAttributeColor,
    },
    #[cfg_attr(feature = "serde_support", serde(rename_all = "camelCase"))]
    StopOpacity {
        stop_opacity: f32,
    },
}

impl SvgAttribute {
    pub fn key(&self) -> &'static str {
        match self {
            Self::Id { .. } => "id",
            Self::Class { .. } => "class",
            Self::Width { .. } => "width",
            Self::Height { .. } => "height",
            Self::Transform { .. } => "transform",
            Self::PatternTransform { .. } => "patternTransform",
            Self::D { .. } => "d",
            Self::ClipPath { .. } => "clip-path",
            Self::Fill { .. } => "fill",
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
            Self::StopOpacity { .. } => "stop-opacity",
        }
    }

    pub fn to_svg_string(&self) -> String {
        match self {
            Self::Id { id } => id.to_string(),
            Self::Class { class } => class.clone(),
            Self::Width { width, unit } => match unit {
                SvgMeasurementUnit::Pixel => width.to_string(),
                SvgMeasurementUnit::Percent => format!("{width}%"),
            },
            Self::Height { height, unit } => match unit {
                SvgMeasurementUnit::Pixel => height.to_string(),
                SvgMeasurementUnit::Percent => format!("{height}%"),
            },
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
            Self::D { d } => d.0.clone(),
            Self::ClipPath { clip_path } => format!("url(#{clip_path})"),
            Self::Fill { fill: color } | Self::StopColor { stop_color: color } => match color {
                SvgAttributeColor::RGB { red, green, blue } => {
                    format!("rgb({red}, {green}, {blue})")
                }
                SvgAttributeColor::RGBA {
                    red,
                    green,
                    blue,
                    alpha,
                } => {
                    format!("rgb({red}, {green}, {blue}, {alpha})")
                }
                SvgAttributeColor::Reference { id } => format!("url(#{id})"),
                SvgAttributeColor::None => String::from("none"),
            },
            Self::PatternUnits {
                pattern_units: unit,
            }
            | Self::GradientUnits {
                gradient_units: unit,
            } => match unit {
                SvgUnits::ObjectBoundingBox => String::from("objectBoundingBox"),
                SvgUnits::UserSpaceOnUse => String::from("userSpaceOnUse"),
            },
            Self::Href { href } => match href {
                SvgHrefAttribute::Base64 {
                    content,
                    content_type,
                } => format!("data:{};base64,{}", content_type.mime_type(), content),
                SvgHrefAttribute::Base64 { .. } => String::from("todo"),
                SvgHrefAttribute::Url { url } => url.clone(),
            },
            Self::PreserveAspectRatio {
                preserve_aspect_ratio,
            } => preserve_aspect_ratio.clone(),
            Self::X1 { x1 } => x1.to_string(),
            Self::Y1 { y1 } => y1.to_string(),
            Self::X2 { x2 } => x2.to_string(),
            Self::Y2 { y2 } => y2.to_string(),
            Self::Offset { offset } => offset.to_string(),
            Self::StopOpacity { stop_opacity } => stop_opacity.to_string(),
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

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum SvgHrefAttribute {
    #[serde(rename_all = "camelCase")]
    Base64 {
        content: String,
        content_type: SvgHrefContentType,
    },
    Url {
        url: String,
    },
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum SvgHrefContentType {
    Jpeg,
    Png,
}

impl SvgHrefContentType {
    pub const fn mime_type(&self) -> &'static str {
        match self {
            SvgHrefContentType::Jpeg => "image/jpeg",
            SvgHrefContentType::Png => "image/png",
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct SvgPathAttribute(pub String);

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
pub enum SvgUnits {
    #[default]
    UserSpaceOnUse,
    ObjectBoundingBox,
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum SvgAttributeColor {
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
    Reference {
        id: SvgElementId,
    },
    None,
}
