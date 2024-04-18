use super::SvgElementId;
use glam::{Mat4, Vec4};

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(tag = "type")
)]
pub enum SvgAttribute {
    // Identifiers and References
    Id {
        id: SvgElementId,
    },
    Class {
        class: String,
    },
    Href {
        href: SvgHrefAttribute,
    },

    // Dimensional Properties
    Width {
        width: f32,
        unit: SvgMeasurementUnit,
    },
    Height {
        height: f32,
        unit: SvgMeasurementUnit,
    },
    X {
        x: f32,
        unit: SvgMeasurementUnit,
    },
    Y {
        y: f32,
        unit: SvgMeasurementUnit,
    },
    DX {
        dx: f32,
    },
    DY {
        dy: f32,
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
    Radius {
        radius: f32,
    },

    // Transformations and Positioning
    Transform {
        transform: SvgTransformAttribute,
    },
    #[cfg_attr(feature = "serde_support", serde(rename_all = "camelCase"))]
    PatternTransform {
        pattern_transform: SvgTransformAttribute,
    },

    // Styling and Appearance
    Fill {
        fill: SvgAttributeColor,
    },
    Filter {
        filter: SvgAttributeFilter,
    },
    D {
        d: SvgPathAttribute,
    },
    #[cfg_attr(feature = "serde_support", serde(rename_all = "camelCase"))]
    ClipPath {
        clip_path: SvgElementId,
    },
    #[cfg_attr(feature = "serde_support", serde(rename_all = "camelCase"))]
    PatternUnits {
        pattern_units: SvgUnits,
    },
    #[cfg_attr(feature = "serde_support", serde(rename_all = "camelCase"))]
    GradientUnits {
        gradient_units: SvgUnits,
    },
    #[cfg_attr(feature = "serde_support", serde(rename_all = "camelCase"))]
    FilterUnits {
        filter_units: SvgUnits,
    },
    #[cfg_attr(feature = "serde_support", serde(rename_all = "camelCase"))]
    PreserveAspectRatio {
        preserve_aspect_ratio: String,
    },
    #[cfg_attr(feature = "serde_support", serde(rename_all = "camelCase"))]
    StopColor {
        stop_color: SvgAttributeColor,
    },
    #[cfg_attr(feature = "serde_support", serde(rename_all = "camelCase"))]
    StopOpacity {
        stop_opacity: f32,
    },
    #[cfg_attr(feature = "serde_support", serde(rename_all = "camelCase"))]
    FloodOpacity {
        flood_opacity: f32,
    },
    #[cfg_attr(feature = "serde_support", serde(rename_all = "camelCase"))]
    ColorInterpolationFilters {
        color_interpolation_filters: String,
    },
    #[cfg_attr(feature = "serde_support", serde(rename_all = "camelCase"))]
    NumOctaves {
        num_octaves: u8,
    },
    #[cfg_attr(feature = "serde_support", serde(rename_all = "camelCase"))]
    BaseFrequency {
        base_frequency: f32,
    },
    Mode {
        mode: SvgAttributeMode,
    },

    // Functional and Miscellaneous Attributes
    K1 {
        k1: f32,
    },
    K2 {
        k2: f32,
    },
    Offset {
        offset: f32,
    },
    Slope {
        slope: f32,
    },
    #[cfg_attr(feature = "serde_support", serde(rename_all = "camelCase"))]
    StdDeviation {
        std_deviation: f32,
    },
    #[cfg_attr(feature = "serde_support", serde(rename_all = "camelCase"))]
    TableValues {
        table_values: Vec<f32>,
    },
    In {
        value: SvgAttributeIn,
    },
    In2 {
        value: SvgAttributeIn,
    },
    Type {
        value: SvgAttributeType,
    },
    Result {
        result: String,
    },
    Values {
        values: SvgAttributeValues,
    },
    Operator {
        operator: SvgAttributeOperator,
    },
}

impl SvgAttribute {
    pub fn key(&self) -> &'static str {
        match self {
            Self::Id { .. } => "id",
            Self::Class { .. } => "class",
            Self::Href { .. } => "href",

            Self::Width { .. } => "width",
            Self::Height { .. } => "height",
            Self::X { .. } => "x",
            Self::Y { .. } => "y",
            Self::DX { .. } => "dx",
            Self::DY { .. } => "dy",
            Self::X1 { .. } => "x1",
            Self::Y1 { .. } => "y1",
            Self::X2 { .. } => "x2",
            Self::Y2 { .. } => "y2",
            Self::Radius { .. } => "radius",

            Self::Transform { .. } => "transform",
            Self::PatternTransform { .. } => "patternTransform",

            Self::Fill { .. } => "fill",
            Self::Filter { .. } => "filter",
            Self::D { .. } => "d",
            Self::ClipPath { .. } => "clip-path",
            Self::PatternUnits { .. } => "patternUnits",
            Self::GradientUnits { .. } => "gradientUnits",
            Self::FilterUnits { .. } => "filterUnits",
            Self::PreserveAspectRatio { .. } => "preserveAspectRatio",
            Self::StopColor { .. } => "stop-color",
            Self::StopOpacity { .. } => "stop-opacity",
            Self::FloodOpacity { .. } => "flood-opacity",
            Self::ColorInterpolationFilters { .. } => "color-interpolation-filters",
            Self::NumOctaves { .. } => "numOctaves",
            Self::BaseFrequency { .. } => "baseFrequency",
            Self::Mode { .. } => "mode",

            Self::K1 { .. } => "k1",
            Self::K2 { .. } => "k2",
            Self::Offset { .. } => "offset",
            Self::Slope { .. } => "slope",
            Self::StdDeviation { .. } => "stdDeviation",
            Self::TableValues { .. } => "tableValues",
            Self::In { .. } => "in",
            Self::In2 { .. } => "in2",
            Self::Type { .. } => "type",
            Self::Result { .. } => "result",
            Self::Values { .. } => "values",
            Self::Operator { .. } => "operator",
        }
    }

    pub fn to_svg_string(&self) -> String {
        match self {
            Self::Id { id } => id.to_string(),
            Self::Class { class } => class.clone(),
            Self::Href { href } => match href {
                SvgHrefAttribute::Base64 {
                    content,
                    content_type,
                } => format!("data:{};base64,{}", content_type.mime_type(), content),
                SvgHrefAttribute::Url { url } => url.clone(),
            },
            Self::Width { width: value, unit }
            | Self::Height {
                height: value,
                unit,
            }
            | Self::X { x: value, unit }
            | Self::Y { y: value, unit } => match unit {
                SvgMeasurementUnit::Pixel => value.to_string(),
                SvgMeasurementUnit::Percent => format!("{value}%"),
            },
            Self::DX { dx } => dx.to_string(),
            Self::DY { dy } => dy.to_string(),
            Self::X1 { x1 } => x1.to_string(),
            Self::Y1 { y1 } => y1.to_string(),
            Self::X2 { x2 } => x2.to_string(),
            Self::Y2 { y2 } => y2.to_string(),
            Self::Radius { radius } => radius.to_string(),

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
            Self::Filter { filter } => match filter {
                SvgAttributeFilter::Reference { id } => format!("url(#{id})"),
                SvgAttributeFilter::None => String::from("none"),
            },
            Self::D { d } => d.0.clone(),
            Self::ClipPath { clip_path } => format!("url(#{clip_path})"),
            Self::PatternUnits {
                pattern_units: unit,
            }
            | Self::GradientUnits {
                gradient_units: unit,
            }
            | Self::FilterUnits { filter_units: unit } => match unit {
                SvgUnits::ObjectBoundingBox => String::from("objectBoundingBox"),
                SvgUnits::UserSpaceOnUse => String::from("userSpaceOnUse"),
            },
            Self::PreserveAspectRatio {
                preserve_aspect_ratio,
            } => preserve_aspect_ratio.clone(),
            Self::StopOpacity {
                stop_opacity: opacity,
            }
            | Self::FloodOpacity {
                flood_opacity: opacity,
            } => opacity.to_string(),
            Self::ColorInterpolationFilters {
                color_interpolation_filters,
            } => color_interpolation_filters.to_string(),
            Self::NumOctaves { num_octaves } => num_octaves.to_string(),
            Self::BaseFrequency { base_frequency } => base_frequency.to_string(),
            Self::Mode { mode } => match mode {
                SvgAttributeMode::Normal => String::from("normal"),
                SvgAttributeMode::Other(other) => other.clone(),
            },

            Self::K1 { k1 } => k1.to_string(),
            Self::K2 { k2 } => k2.to_string(),
            Self::Offset { offset } => offset.to_string(),
            Self::Slope { slope } => slope.to_string(),
            Self::StdDeviation {
                std_deviation: deviation,
            } => deviation.to_string(),
            Self::TableValues {
                table_values: values,
            } => values.iter().map(|&id| id.to_string() + " ").collect(),
            Self::In { value } | Self::In2 { value } => match value {
                SvgAttributeIn::SourceAlpha => String::from("SourceAlpha"),
                SvgAttributeIn::SourceGraphic => String::from("SourceGraphic"),
                SvgAttributeIn::Other(other) => other.clone(),
            },
            Self::Type { value } => match value {
                SvgAttributeType::Matrix => String::from("matrix"),
                SvgAttributeType::Other(other) => other.clone(),
            },
            Self::Result { result } => result.clone(),
            Self::Values { values } => match values {
                SvgAttributeValues::ColorMatrix(matrix) => matrix.to_string(),
            },
            Self::Operator { operator } => match operator {
                SvgAttributeOperator::Dilate => String::from("dilate"),
                SvgAttributeOperator::Other(other) => other.clone(),
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
    Svg,
}

impl SvgHrefContentType {
    pub const fn mime_type(&self) -> &'static str {
        match self {
            SvgHrefContentType::Jpeg => "image/jpeg",
            SvgHrefContentType::Png => "image/png",
            SvgHrefContentType::Svg => "image/svg+xml",
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

#[derive(Debug, PartialEq, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum SvgAttributeFilter {
    Reference { id: SvgElementId },
    None,
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum SvgAttributeValues {
    ColorMatrix(ColorMatrix),
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum SvgAttributeIn {
    SourceAlpha,
    SourceGraphic,
    Other(String),
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum SvgAttributeOperator {
    Dilate,
    Other(String),
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum SvgAttributeType {
    Matrix,
    Other(String),
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum SvgAttributeMode {
    Normal,
    Other(String),
}

// https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feColorMatrix
// [r,0,0,0], // red
// [0,g,0,0], // green
// [0,0,b,0], // blue
// [0,0,0,1], // multiplyer
#[derive(Debug, PartialEq, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct ColorMatrix(pub Mat4);

impl ColorMatrix {
    pub fn from_rgba(r: u8, g: u8, b: u8, a: f32) -> Self {
        Self(Mat4::from_cols(
            Vec4::new(0.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 0.0),
            Vec4::new(
                f32::from(r) / 255.0,
                f32::from(g) / 255.0,
                f32::from(b) / 255.0,
                a,
            ),
        ))
    }

    pub fn to_string(&self) -> String {
        format!(
            "{} {} {} {} 0 {} {} {} {} 0 {} {} {} {} 0 {} {} {} {} 0",
            self.0.row(0).x,
            self.0.row(0).y,
            self.0.row(0).z,
            self.0.row(0).w,
            self.0.row(1).x,
            self.0.row(1).y,
            self.0.row(1).z,
            self.0.row(1).w,
            self.0.row(2).x,
            self.0.row(2).y,
            self.0.row(2).z,
            self.0.row(2).w,
            self.0.row(3).x,
            self.0.row(3).y,
            self.0.row(3).z,
            self.0.row(3).w
        )
    }
}
