use bevy_ecs::prelude::*;
use glam::{Mat3, Vec2};
use serde::Serialize;
#[cfg(feature = "cli")]
use specta::Type;

#[cfg_attr(feature = "cli", derive(Type))]
#[derive(Component, Serialize, Clone, Debug)]
pub struct RectangleCornerMixin {
    pub top_left_radius: i16,
    pub top_right_radius: i16,
    pub bottom_right_radius: i16,
    pub bottom_left_radius: i16,
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

#[cfg_attr(feature = "cli", derive(Type))]
#[derive(Component, Serialize, Clone, Debug)]
pub struct ChildrenMixin {
    pub children: Vec<Entity>,
}

impl Default for ChildrenMixin {
    fn default() -> Self {
        Self { children: vec![] }
    }
}

#[cfg_attr(feature = "cli", derive(Type))]
#[derive(Component, Serialize, Clone, Debug)]
pub struct LayoutMixin {
    pub width: usize,
    pub height: usize,
    pub relative_transform: Mat3,
}

impl Default for LayoutMixin {
    fn default() -> Self {
        Self {
            width: 100,
            height: 100,
            relative_transform: Mat3::default(),
        }
    }
}

#[cfg_attr(feature = "cli", derive(Type))]
#[derive(Component, Serialize, Clone, Debug)]
pub struct CompositionMixin {
    is_visible: bool,
    is_locked: bool,
}

impl Default for CompositionMixin {
    fn default() -> Self {
        Self {
            is_visible: true,
            is_locked: false,
        }
    }
}

#[cfg_attr(feature = "cli", derive(Type))]
#[derive(Component, Serialize, Clone, Debug)]
pub struct BlendMixin {
    blend_mode: BlendMode,
    opacity: u8,
    is_mask: bool,
}

impl Default for BlendMixin {
    fn default() -> Self {
        Self {
            blend_mode: BlendMode::Normal,
            opacity: 255,
            is_mask: false,
        }
    }
}

#[cfg_attr(feature = "cli", derive(Type))]
#[derive(Component, Serialize, Clone, Debug)]
pub struct PathMixin {
    pub vertices: Vec<Anchor>,
}

#[cfg_attr(feature = "cli", derive(Type))]
#[derive(Serialize, Clone, Debug)]
pub struct Anchor {
    pub position: Vec2,
    pub command: AnchorCommand,
}

#[cfg_attr(feature = "cli", derive(Type))]
#[derive(Serialize, Clone, Debug)]
pub enum AnchorCommand {
    MoveTo,
    LineTo,
    CurveTo {
        control_point_1: Vec2,
        control_point_2: Vec2,
    },
    ArcTo {
        radius: Vec2,
        x_axis_rotation: f32,
        large_arc_flag: bool,
        sweep_flag: bool,
    },
    ClosePath,
}

// =============================================================================
// Effects
// =============================================================================

#[cfg_attr(feature = "cli", derive(Type))]
#[derive(Debug, Clone, Serialize)]
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
