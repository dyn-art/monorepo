use bevy_ecs::entity::Entity;
use dyn_composition::core::modules::composition::resources::composition::ViewBox;
use dyn_svg_render::{events::output_event::ElementChangeEvent, mixin_change::MixinChange};
use serde::Serialize;
use specta::Type;

#[derive(Debug, Serialize, Clone, Type)]
#[serde(tag = "type")]
pub enum OutputEvent {
    ElementUpdate(ElementChangeEvent),
    CompositionUpdate(CompositionChangeEvent),
    TrackUpdate(TrackUpdateEvent),
    SelectionChange(SelectionChangeEvent),
    InteractionModeChange(InteractionModeChangeEvent),
    CursorChange(CursorChangeEvent),
}

#[derive(Debug, Serialize, Clone, Type)]
pub struct CompositionChangeEvent {
    pub changes: Vec<CompositionChange>,
}

/// Represents the different types of events that can be emitted by the SVGComposition
/// to synchronize its state with the frontend.
#[derive(Debug, Serialize, Clone, Type)]
#[serde(tag = "type")]
pub enum CompositionChange {
    SizeChanged(SizeChanged),
    ViewBoxChanged(ViewBoxChanged),
}

/// Emitted when the size of the Composition is changed.
#[derive(Debug, Serialize, Clone, Type)]
pub struct SizeChanged {
    pub width: f32,
    pub height: f32,
}

/// Emitted when the view box of the Composition is changed.
#[derive(Debug, Serialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct ViewBoxChanged {
    pub view_box: ViewBox,
}

#[derive(Debug, Serialize, Clone, Type)]
pub struct TrackUpdateEvent {
    pub id: Entity,
    pub updates: Vec<MixinChange>,
}

#[derive(Debug, Serialize, Clone, Type)]
pub struct SelectionChangeEvent {
    pub selected: Vec<Entity>,
}

#[derive(Debug, Serialize, Clone, Type)]
pub struct InteractionModeChangeEvent {
    #[serde(rename = "interactionMode")]
    pub interaction_mode: InteractionModeForFrontend,
}

#[derive(Debug, Serialize, Clone, Type, PartialEq)]
#[serde(tag = "type")]
pub enum InteractionModeForFrontend {
    None,
    Pressing,
    Translating,
    Resizing,
    Rotating,
}

impl Default for InteractionModeForFrontend {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Serialize, Clone, Type)]
pub struct CursorChangeEvent {
    pub cursor: CursorForFrontend,
}

#[derive(Debug, Serialize, Clone, Type, PartialEq)]
#[serde(tag = "type")]
pub enum CursorForFrontend {
    Default,
    Resize {
        #[serde(rename = "rotationInDegrees")]
        rotation_in_degrees: f32,
    },
    Rotate {
        #[serde(rename = "rotationInDegrees")]
        rotation_in_degrees: f32,
    },
}

impl Default for CursorForFrontend {
    fn default() -> Self {
        Self::Default
    }
}
