use bevy_ecs::component::Component;

use crate::common::Color;

#[derive(Component, Debug, Copy, Clone)]
pub struct CompPaint {
    pub variant: CompPaintVariant,
}

#[derive(Debug, Copy, Clone)]
pub enum CompPaintVariant {
    Solid,
}

#[derive(Component, Debug, Default, Copy, Clone)]
pub struct SolidCompPaint {
    pub color: Color,
}
