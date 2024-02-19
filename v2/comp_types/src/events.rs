use bevy_ecs::entity::Entity;
use glam::Vec2;

use crate::shared::Size;

pub trait InputEvent {
    fn send_to_ecs(self, world: &mut World);
}

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct CompositionResized {
    pub size: Size,
}

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct CompositionViewportChanged {
    pub physical_position: Vec2,
    pub physical_size: Vec2,
}

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct EntityMoved {
    pub entity: Entity,
    pub dx: f32,
    pub dy: f32,
}

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct EntitySetPosition {
    pub entity: Entity,
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct EntityDeleted {
    pub entity: Entity,
}
