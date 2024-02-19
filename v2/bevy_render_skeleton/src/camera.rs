use bevy_ecs::prelude::*;
use glam::UVec2;

// https://docs.rs/bevy/latest/bevy/render/camera/struct.Viewport.html
#[derive(Debug, Default, Copy, Clone)]
pub struct Viewport {
    pub physical_position: UVec2,
    pub physical_size: UVec2,
}

// https://docs.rs/bevy/latest/bevy/render/camera/struct.Viewport.html
#[derive(Component, Debug, Default, Copy, Clone)]
pub struct Camera {
    pub viewport: Viewport,
}
