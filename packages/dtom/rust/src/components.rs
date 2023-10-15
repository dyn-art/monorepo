use bevy_ecs::prelude::*;

#[derive(Component, Debug)]
struct Shape;

#[derive(Component, Debug)]
struct Path {
    points: Vec<Anchor>,
}

#[derive(Component, Debug)]
pub struct Transform {
    translation: Vec2,
    rotation: i16,
    scale: Vec2,
}

#[derive(Debug)]
struct Anchor {
    position: Vec2,
    control: (Vec2, Vec2),
}

#[derive(Debug, Bundle)]
struct Rectangle {
    shape: Shape,
    path: Path,
    transform: Transform,
}

#[derive(Debug)]
struct Vec2(f32, f32);
