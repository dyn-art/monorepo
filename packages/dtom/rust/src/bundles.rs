use bevy_ecs::bundle::Bundle;

use super::shapes::{Path, Shape, Transform, Vec2};

#[derive(Debug, Bundle)]
pub struct RectangleBundle {
    pub shape: Shape,
    pub path: Path,
    pub transform: Transform,
}

impl Default for RectangleBundle {
    fn default() -> Self {
        Self {
            shape: Shape,
            path: Path { points: Vec::new() },
            transform: Transform {
                translation: Vec2 { x: 0.0, y: 0.0 },
                rotation: 0,
            },
        }
    }
}
