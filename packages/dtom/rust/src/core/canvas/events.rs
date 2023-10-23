use bevy_ecs::event::Event;
use glam::Vec2;

// #[derive(Event, Debug)]
// pub struct CanvasResized {
//     pub width: f32,
//     pub height: f32,
// }

#[derive(Event, Debug)]
pub struct CursorMovedOnCanvas {
    pub position: Vec2,
}

#[derive(Event, Debug)]
pub struct CursorEnteredCanvas;

#[derive(Event, Debug)]
pub struct CursorExitedCanvas;
