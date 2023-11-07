//! Events received by the Composition

use bevy_ecs::{entity::Entity, event::Event};
use glam::Vec2;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Deserialize, Type, Clone)]
#[serde(tag = "type")]
pub enum InputEvent {
    // Cursor Events
    CursorDownOnEntity(CursorDownOnEntity),
    CursorMovedOnComposition(CursorMovedOnComposition),
    CursorEnteredComposition(CursorEnteredComposition),
    CursorExitedComposition(CursorExitedComposition),

    // Entity Events
    EntityMoved(EntityMoved),
    EntitySetPosition(EntitySetPosition),
}

// =============================================================================
// Cursor Events
// =============================================================================

#[derive(Event, Debug, Deserialize, Type, Clone)]
pub struct CursorMovedOnComposition {
    pub position: Vec2,
}

#[derive(Event, Debug, Deserialize, Type, Clone)]
pub struct CursorEnteredComposition;

#[derive(Event, Debug, Deserialize, Type, Clone)]
pub struct CursorExitedComposition;

#[derive(Event, Debug, Serialize, Deserialize, Type, Clone)]
pub struct CursorDownOnEntity {
    pub entity: Entity,
}

// =============================================================================
// Entity Events
// =============================================================================

#[derive(Event, Debug, Deserialize, Type, Clone)]
pub struct EntityMoved {
    pub entity: Entity,
    pub dx: f32,
    pub dy: f32,
}

#[derive(Event, Debug, Deserialize, Type, Clone)]
pub struct EntitySetPosition {
    pub entity: Entity,
    pub x: f32,
    pub y: f32,
}
