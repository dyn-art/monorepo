use bevy_ecs::component::Component;
use dyn_utils::{
    properties::{color::Color, opacity::Opacity},
    units::{abs::Abs, angle::Angle, ratio::Ratio},
};

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
    pub opacity: Opacity,
    pub angle_deg: Angle,
    pub distance: Abs,
    pub spread: Abs,
    pub size: Abs,
    pub contour: Vec<Abs>,
    pub noise: Ratio,
}
