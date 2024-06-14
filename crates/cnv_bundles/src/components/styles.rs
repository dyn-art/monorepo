use bevy_ecs::component::Component;
use dyn_utils::{properties::color::Color, units::abs::Abs};
use glam::Vec2;

#[derive(Component, Debug, Copy, Clone)]
pub struct CnvStyle {
    pub variant: CnvStyleVariant,
}

#[derive(Debug, Copy, Clone)]
pub enum CnvStyleVariant {
    Fill,
    Stroke,
    DropShadow,
}

#[derive(Component, Debug, Default, Copy, Clone)]
pub struct FillCnvStyle;

#[derive(Component, Debug, Default, Clone)]
pub struct StrokeCnvStyle {
    pub stroke: tiny_skia_path::Stroke,
}

#[derive(Component, Debug, Default, Clone)]
pub struct DropShadowCnvStyle {
    pub color: Color,
    pub position: Vec2,
    pub spread: Abs,
    pub blur: Abs,
}
