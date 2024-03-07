use bevy_ecs::component::Component;
use smallvec::SmallVec;

use crate::common::{Color, GradientColorStop, GradientVariant, ImageScaleMode};

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

#[derive(Component, Debug, Clone)]
pub struct GradientCompPaint {
    pub variant: GradientVariant,
    pub stops: SmallVec<[GradientColorStop; 4]>,
}
