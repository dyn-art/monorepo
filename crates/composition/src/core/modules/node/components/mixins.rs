use bevy_ecs::prelude::*;
use glam::{Mat3, Vec2};
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Component, Serialize, Deserialize, Clone, Debug, Type)]
pub struct RectangleCornerMixin {
    #[serde(rename = "topLeftRadius")]
    pub top_left_radius: u8,

    #[serde(rename = "topRightRadius")]
    pub top_right_radius: u8,

    #[serde(rename = "bottomRightRadius")]
    pub bottom_right_radius: u8,

    #[serde(rename = "bottomLeftRadius")]
    pub bottom_left_radius: u8,
}

impl Default for RectangleCornerMixin {
    fn default() -> Self {
        Self {
            top_left_radius: 0,
            top_right_radius: 0,
            bottom_right_radius: 0,
            bottom_left_radius: 0,
        }
    }
}

#[derive(Component, Serialize, Deserialize, Clone, Debug, Type)]
pub struct ChildrenMixin(pub Vec<Entity>);

impl Default for ChildrenMixin {
    fn default() -> Self {
        Self(vec![])
    }
}

#[derive(Component, Serialize, Deserialize, Clone, Debug, Type)]
pub struct DimensionMixin {
    pub width: u32,
    pub height: u32,
}

impl Default for DimensionMixin {
    fn default() -> Self {
        Self {
            width: 100,
            height: 100,
        }
    }
}

#[derive(Component, Serialize, Deserialize, Clone, Debug, Type)]
pub struct RelativeTransformMixin(pub Mat3);

impl Default for RelativeTransformMixin {
    fn default() -> Self {
        Self(Mat3::default())
    }
}

#[derive(Component, Serialize, Deserialize, Clone, Debug, Type)]
pub struct NodeCompositionMixin {
    #[serde(rename = "isVisible")]
    pub is_visible: bool,

    #[serde(rename = "isLocked")]
    pub is_locked: bool,
}

impl Default for NodeCompositionMixin {
    fn default() -> Self {
        Self {
            is_visible: true,
            is_locked: false,
        }
    }
}

#[derive(Component, Serialize, Deserialize, Clone, Debug, Type)]
pub struct BlendMixin {
    #[serde(rename = "blendMode")]
    pub blend_mode: BlendMode,

    pub opacity: f32, // 0 - 1

    #[serde(rename = "isMask")]
    pub is_mask: bool,
}

impl Default for BlendMixin {
    fn default() -> Self {
        Self {
            blend_mode: BlendMode::Normal,
            opacity: 1.0,
            is_mask: false,
        }
    }
}

#[derive(Component, Serialize, Deserialize, Clone, Debug, Type)]
pub struct PathMixin {
    pub vertices: Vec<Anchor>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct Anchor {
    pub position: Vec2,
    pub command: AnchorCommand,
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub enum AnchorCommand {
    MoveTo,
    LineTo,
    CurveTo {
        #[serde(rename = "controlPoint1")]
        control_point_1: Vec2,

        #[serde(rename = "controlPoint2")]
        control_point_2: Vec2,
    },
    ArcTo {
        radius: Vec2,

        #[serde(rename = "xAxisRotation")]
        x_axis_rotation: f32,

        #[serde(rename = "largeArcFlag")]
        large_arc_flag: bool,

        #[serde(rename = "sweepFlag")]
        sweep_flag: bool,
    },
    ClosePath,
}

// =============================================================================
// Effects
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub enum BlendMode {
    PassThrough,
    Normal,
    Darken,
    Multiply,
    LinearBurn,
    ColorBurn,
    Lighten,
    Screen,
    LinearDodge,
    ColorDodge,
    Overlay,
    SoftLight,
    HardLight,
    Difference,
    Exclusion,
    Hue,
    Saturation,
    Color,
    Luminosity,
}
