use bevy_ecs::component::Component;
use dyn_utils::{
    properties::{color::Color, opacity::Opacity},
    units::ratio::Ratio,
};
use glam::Mat3;
use smallvec::SmallVec;

// TODO: Should Paint be directly embedded in FillStyle & StorkeStyle or exist as separate Entity?
// Embedding simplifies logic but keeping it separate allows reuse across multiple nodes (like texture).
// Is the added complexity justified?
//
// Or should the Paints be managed by a SlotMap or something like the Assets (Images, Fonts, ..)?
// https://bevyengine.org/examples-webgpu/3D%20Rendering/texture/

#[derive(Component, Debug, Copy, Clone)]
pub struct CompPaint {
    pub variant: CompPaintVariant,
}

#[derive(Debug, Copy, Clone)]
pub enum CompPaintVariant {
    Solid,
    Image,
    Gradient,
}

#[derive(Component, Debug, Default, Copy, Clone)]
pub struct SolidCompPaint {
    pub color: Color,
}

#[derive(Component, Debug, Default, Copy, Clone)]
pub struct ImageCompPaint {
    pub scale_mode: ImageScaleMode,
}

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(tag = "type")
)]
pub enum ImageScaleMode {
    /// Fills the area completely with the image.
    #[default]
    Fill,

    /// Fits the image within the area while maintaining its aspect ratio.
    Fit,

    /// Crops the image to fill the area.
    Crop {
        #[cfg_attr(feature = "serde_support", serde(default))]
        transform: Mat3,
    },

    /// Tiles the image within the area.
    #[cfg_attr(feature = "serde_support", serde(rename_all = "camelCase"))]
    Tile {
        #[cfg_attr(feature = "serde_support", serde(default))]
        rotation: f32,
        scaling_factor: f32,
    },
}

#[derive(Component, Debug, Clone)]
pub struct GradientCompPaint {
    pub variant: GradientVariant,
    pub stops: SmallVec<[GradientColorStop; 4]>,
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(tag = "type")
)]
pub enum GradientVariant {
    Linear {
        #[cfg_attr(feature = "serde_support", serde(default))]
        transform: Mat3,
    },
    Radial {
        #[cfg_attr(feature = "serde_support", serde(default))]
        transform: Mat3,
    },
}

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct GradientColorStop {
    /// The position of the color stop in the gradient, ranging from 0.0 to 1.0.
    pub position: Ratio,

    /// The color of the stop.
    pub color: Color,

    /// The opacity of the stop.
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub opacity: Opacity,
}
