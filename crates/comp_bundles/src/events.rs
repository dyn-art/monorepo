use crate::properties::Viewport;
use bevy_ecs::{entity::Entity, event::Event, world::World};
use dyn_utils::{properties::size::Size, units::angle::Angle};
use std::fmt::Debug;

pub trait InputEvent {
    fn send_into_ecs(self, world: &mut World);
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(tag = "type")
)]
pub enum CompCoreInputEvent {
    // Composition
    ResizeComposition(ResizeCompositionInputEvent),
    SetCompositionViewport(SetCompositionViewportInputEvent),

    // Node
    FocusRootNodes(FocusRootNodesInputEvent),

    // Entity
    DeleteEntity(DeleteEntityInputEvent),
    MoveEntity(MoveEntityInputEvent),
    SetEntityPosition(SetEntityPositionInputEvent),
    SetEntityRotation(SetEntityRotationInputEvent),
}

impl InputEvent for CompCoreInputEvent {
    fn send_into_ecs(self, world: &mut World) {
        match self {
            // Composition
            CompCoreInputEvent::ResizeComposition(event) => {
                world.send_event(event);
            }
            CompCoreInputEvent::SetCompositionViewport(event) => {
                world.send_event(event);
            }

            // Node
            CompCoreInputEvent::FocusRootNodes(event) => {
                world.send_event(event);
            }

            // Entity
            CompCoreInputEvent::DeleteEntity(event) => {
                world.send_event(event);
            }
            CompCoreInputEvent::MoveEntity(event) => {
                world.send_event(event);
            }
            CompCoreInputEvent::SetEntityPosition(event) => {
                world.send_event(event);
            }
            CompCoreInputEvent::SetEntityRotation(event) => {
                world.send_event(event);
            }
        }
    }
}

// =============================================================================
// Composition
// =============================================================================

#[derive(Event, Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct ResizeCompositionInputEvent {
    pub size: Size,
}

#[derive(Event, Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct SetCompositionViewportInputEvent {
    pub viewport: Viewport,
}

// =============================================================================
// Node
// =============================================================================

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct FocusRootNodesInputEvent;

// =============================================================================
// Entity
// =============================================================================

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct MoveEntityInputEvent {
    pub entity: Entity,
    pub dx: f32,
    pub dy: f32,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct SetEntityPositionInputEvent {
    pub entity: Entity,
    pub x: f32,
    pub y: f32,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct DeleteEntityInputEvent {
    pub entity: Entity,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct SetEntityRotationInputEvent {
    pub entity: Entity,
    pub rotation_deg: Angle,
}
