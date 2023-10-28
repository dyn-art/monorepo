use bevy_ecs::{entity::Entity, event::Event};
use glam::Vec2;

// =============================================================================
// Cursor Events
// =============================================================================

// #[derive(Event, Debug)]
// pub struct CompositionResized {
//     pub width: f32,
//     pub height: f32,
// }

#[derive(Event, Debug)]
pub struct CursorMovedOnComposition {
    pub position: Vec2,
}

#[derive(Event, Debug)]
pub struct CursorEnteredComposition;

#[derive(Event, Debug)]
pub struct CursorExitedComposition;

// =============================================================================
// Entity Events
// =============================================================================

#[derive(Event, Debug)]
pub struct EntityMoved {
    pub entity: Entity,
    pub dx: f32,
    pub dy: f32,
}
