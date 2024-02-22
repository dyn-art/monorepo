use super::SvgElementId;

#[derive(Debug, Clone)]
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
        }
    }

    pub fn into_svg_string(&self) -> String {
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
        }
    }

    pub fn into_tuple(&self) -> (&'static str, String) {
        (self.key(), self.into_svg_string())
    }
}

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum SvgMeasurementUnit {
    #[default]
    Pixel,
    Percent,
}

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum SvgUnitsVariant {
    #[default]
    UserSpaceOnUse,
    ObjectBoundingBox,
}
