use bevy_ecs::component::Component;
use dyn_utils::{
    properties::color::Color,
    units::{abs::Abs, ratio::Ratio},
};
use glam::Vec2;

#[derive(Component, Debug, Copy, Clone)]
pub struct CompStyle {
    pub variant: CompStyleVariant,
}

#[derive(Debug, Copy, Clone)]
pub enum CompStyleVariant {
    Fill,
    Stroke,
    DropShadow,
}

#[derive(Component, Debug, Default, Copy, Clone)]
pub struct FillCompStyle;

#[derive(Component, Debug, Default, Clone)]
pub struct StrokeCompStyle {
    pub stroke: tiny_skia_path::Stroke,
}

#[derive(Component, Debug, Default, Clone)]
pub struct DropShadowCompStyle {
    pub color: Color,
    pub position: Vec2,
    pub spread: Abs,
    pub blur: Abs,
    pub contour: Vec<Abs>,
    pub noise: Ratio,
}
