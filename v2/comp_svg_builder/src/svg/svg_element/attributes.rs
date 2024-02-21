use super::SVGElementId;

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, specta::Type),
    serde(tag = "type")
)]
pub enum SVGAttribute {
    Id {
        id: SVGElementId,
    },
    Name {
        name: String,
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
    #[cfg_attr(feature = "serde_support", serde(rename_all = "camelCase"))]
    PatternTransform {
        pattern_transform: SVGTransformAttribute,
    },
    D {
        d: String,
    },
    #[cfg_attr(feature = "serde_support", serde(rename_all = "camelCase"))]
    ClipPath {
        clip_path: SVGElementId,
    },
    Fill {
        fill: String,
    },
    ReferencedFill {
        id: SVGElementId,
    },
    #[cfg_attr(feature = "serde_support", serde(rename_all = "camelCase"))]
    PatternUnits {
        pattern_units: SVGUnitsVariant,
    },
    #[cfg_attr(feature = "serde_support", serde(rename_all = "camelCase"))]
    GradientUnits {
        gradient_units: SVGUnitsVariant,
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
        }
    }

    pub fn into_svg_string(&self) -> String {
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
            Self::Transform { transform }
            | Self::PatternTransform {
                pattern_transform: transform,
            } => match transform {
                SVGTransformAttribute::Matrix { a, b, c, d, tx, ty } => {
                    format!("matrix({a}, {b}, {c}, {d}, {tx}, {ty})")
                }
                SVGTransformAttribute::Rotate { rotation } => {
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
                SVGUnitsVariant::ObjectBoundingBox => String::from("objectBoundingBox"),
                SVGUnitsVariant::UserSpaceOnUse => String::from("userSpaceOnUse"),
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
    derive(serde::Serialize, specta::Type),
    serde(tag = "type")
)]
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

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(feature = "serde_support", derive(serde::Serialize, specta::Type))]
pub enum SVGMeasurementUnit {
    #[default]
    Pixel,
    Percent,
}

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(feature = "serde_support", derive(serde::Serialize, specta::Type))]
pub enum SVGUnitsVariant {
    #[default]
    UserSpaceOnUse,
    ObjectBoundingBox,
}
