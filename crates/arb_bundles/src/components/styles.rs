use bevy_ecs::component::Component;
use dyn_utils::{properties::color::Color, units::abs::Abs};
use glam::Vec2;

#[derive(Component, Debug, Copy, Clone)]
pub struct ArbStyle {
    pub variant: ArbStyleVariant,
}

#[derive(Debug, Copy, Clone)]
pub enum ArbStyleVariant {
    Fill,
    Stroke,
    DropShadow,
}

#[derive(Component, Debug, Default, Copy, Clone)]
pub struct FillArbStyle;

#[derive(Component, Debug, Default, Clone)]
pub struct StrokeArbStyle {
    pub stroke: tiny_skia_path::Stroke,
}

#[derive(Component, Debug, Default, Clone)]
pub struct DropShadowArbStyle {
    pub color: Color,
    pub position: Vec2,
    pub spread: Abs,
    pub blur: Abs,
}
