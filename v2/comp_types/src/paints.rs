use bevy_ecs::component::Component;

use crate::common::{Color, ImageScaleMode};

#[derive(Component, Debug, Copy, Clone)]
pub struct CompPaint {
    pub variant: CompPaintVariant,
}

#[derive(Debug, Copy, Clone)]
pub enum CompPaintVariant {
    Solid,
    Image,
}

#[derive(Component, Debug, Default, Copy, Clone)]
pub struct SolidCompPaint {
    pub color: Color,
}

#[derive(Component, Debug, Default, Copy, Clone)]
pub struct ImageCompPaint {
    pub scale_mode: ImageScaleMode,
}
