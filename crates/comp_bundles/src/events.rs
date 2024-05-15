use crate::properties::{TextAttributeInterval, Viewport};
use bevy_ecs::{entity::Entity, event::Event, world::World};
use dyn_attributed_string::layout::{HorizontalTextAlignment, LineWrap, VerticalTextAlignment};
use dyn_utils::{properties::size::Size, units::angle::Angle};
use std::fmt::Debug;

pub trait InputEvent {
    fn send_into_ecs(self, world: &mut World);
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(tag = "type")
)]
pub enum CompCoreInputEvent {
    // Composition
    UpdateCompositionSize(UpdateCompositionSizeInputEvent),
    UpdateCompositionViewport(UpdateCompositionViewportInputEvent),

    // Node
    FocusRootNodes(FocusRootNodesInputEvent),

    // Entity
    DeleteEntity(DeleteEntityInputEvent),
    MoveEntity(MoveEntityInputEvent),
    UpdateEntityPosition(UpdateEntityPositionInputEvent),
    UpdateEntityRotation(UpdateEntityRotationInputEvent),
    UpdateEntityText(UpdateEntityTextInputEvent),
    UpdateEntityVisibility(UpdateEntityVisibilityInputEvent),
}

impl InputEvent for CompCoreInputEvent {
    fn send_into_ecs(self, world: &mut World) {
        match self {
            // Composition
            CompCoreInputEvent::UpdateCompositionSize(event) => {
                world.send_event(event);
            }
            CompCoreInputEvent::UpdateCompositionViewport(event) => {
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
            CompCoreInputEvent::UpdateEntityPosition(event) => {
                world.send_event(event);
            }
            CompCoreInputEvent::UpdateEntityRotation(event) => {
                world.send_event(event);
            }
            CompCoreInputEvent::UpdateEntityText(event) => {
                world.send_event(event);
            }
            CompCoreInputEvent::UpdateEntityVisibility(event) => {
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
pub struct UpdateCompositionSizeInputEvent {
    pub size: Size,
}

#[derive(Event, Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct UpdateCompositionViewportInputEvent {
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
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub dx: Option<f32>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub dy: Option<f32>,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct UpdateEntityPositionInputEvent {
    pub entity: Entity,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub x: Option<f32>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub y: Option<f32>,
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
pub struct UpdateEntityRotationInputEvent {
    pub entity: Entity,
    pub rotation_deg: Angle,
}

#[derive(Event, Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct UpdateEntityTextInputEvent {
    pub entity: Entity,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub text: Option<String>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub attributes: Option<Vec<TextAttributeInterval>>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub line_wrap: Option<LineWrap>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub horizontal_text_alignment: Option<HorizontalTextAlignment>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub vertical_text_alignment: Option<VerticalTextAlignment>,
}

#[derive(Event, Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct UpdateEntityVisibilityInputEvent {
    pub entity: Entity,
    pub visible: bool,
}
