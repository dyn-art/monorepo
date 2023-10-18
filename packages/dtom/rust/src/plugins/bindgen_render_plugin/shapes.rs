use bevy_ecs::prelude::*;
use serde::Serialize;

#[derive(Component, Serialize, Clone, Debug)]
pub struct Shape;

#[derive(Component, Serialize, Clone, Debug)]
pub struct Path {
    pub points: Vec<Vec2>,
}

#[derive(Component, Serialize, Clone, Debug)]
pub struct Transform {
    pub translation: Vec2,
    pub rotation: i16,
}

#[derive(Component, Serialize, Clone, Debug)]
pub struct Fill {
    pub paint_ids: Vec<u32>,
}

#[derive(Component, Serialize, Clone, Debug)]
pub struct SolidPaint {
    pub color: (u8, u8, u8),
}

#[derive(Debug, Serialize, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
