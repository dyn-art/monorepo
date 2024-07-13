use bevy_ecs::component::Component;
use glam::{Mat3, Vec2};
use serde::Serialize;
use specta::Type;

// =============================================================================
// SVG Image Paint
// =============================================================================

#[derive(Component, Clone, Debug, Serialize, Type)]
pub struct SVGImagePaint {
    pub scale_mode: SVGImagePaintScaleMode,
}

#[derive(Clone, Debug, Serialize, Type)]
pub enum SVGImagePaintScaleMode {
    Fill,
    Fit,
    Crop {
        transform: Mat3,
        image_width: f32,
        image_height: f32,
    },
    Tile {
        rotation: f32,
        tile_width: f32,
        tile_height: f32,
    },
}

// =============================================================================
// SVG Gradient Paint
// =============================================================================

#[derive(Component, Clone, Debug, Serialize, Type)]
pub struct SVGGradientPaint {
    pub variant: SVGGradientPaintVariant,
}

#[derive(Clone, Debug, Serialize, Type)]
pub enum SVGGradientPaintVariant {
    Linear {
        start: Vec2,
        end: Vec2,
    },
    Radial {
        center: Vec2,
        radius: Vec2,
        rotation: f32,
    },
}
