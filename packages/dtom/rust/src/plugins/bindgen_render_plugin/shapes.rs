use bevy_ecs::prelude::*;

#[derive(Component, Debug)]
pub struct Shape;

#[derive(Component, Debug)]
pub struct Path {
    pub points: Vec<Vec2>,
}

#[derive(Component, Debug)]
pub struct Transform {
    pub translation: Vec2,
    pub rotation: i16,
}

#[derive(Component, Debug)]
pub struct Fill {
    pub paint_ids: Vec<u32>,
}

#[derive(Component, Debug)]
pub struct SolidPaint {
    pub color: (u8, u8, u8),
}

#[derive(Debug)]
pub struct Vec2(pub f32, pub f32);
