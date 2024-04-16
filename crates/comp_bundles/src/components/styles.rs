use bevy_ecs::component::Component;

#[derive(Component, Debug, Copy, Clone)]
pub struct CompStyle {
    pub variant: CompStyleVariant,
}

#[derive(Debug, Copy, Clone)]
pub enum CompStyleVariant {
    Fill,
    Stroke,
    Shadow,
}

#[derive(Component, Debug, Default, Copy, Clone)]
pub struct FillCompStyle;

#[derive(Component, Debug, Default, Clone)]
pub struct StrokeCompStyle {
    pub stroke: tiny_skia_path::Stroke,
}
