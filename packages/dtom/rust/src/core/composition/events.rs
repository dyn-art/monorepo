use bevy_ecs::event::Event;
use glam::Vec2;

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
