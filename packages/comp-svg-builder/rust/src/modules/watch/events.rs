use super::component_change::ComponentChange;
use bevy_ecs::entity::Entity;
use dyn_comp_bundles::properties::Viewport;
use dyn_comp_interaction::resources::comp_interaction::{InteractionMode, InteractionTool};
use dyn_utils::properties::size::Size;

#[derive(Debug, Clone, serde::Serialize, specta::Type)]
pub struct WatchedEntityChangesOutputEvent {
    pub entity: Entity,
    pub changes: Vec<ComponentChange>,
}

#[derive(Debug, Clone, serde::Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct SelectionChangeOutputEvent {
    pub selected_entities: Vec<Entity>,
}

#[derive(Debug, Clone, serde::Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct CompositionChangeOutputEvent {
    pub root_nodes: Vec<Entity>,
    pub viewport: Viewport,
    pub size: Size,
}

#[derive(Debug, Clone, serde::Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct InteractionToolChangeOutputEvent {
    pub interaction_tool: InteractionTool,
}

#[derive(Debug, Clone, serde::Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct InteractionModeChangeOutputEvent {
    pub interaction_mode: InteractionModeLabel,
}

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone, serde::Serialize, specta::Type)]
pub enum InteractionModeLabel {
    #[default]
    None,
    Pressing,
    Translating,
    Resizing,
    Rotating,
    Dragging,
    Inserting,
}

impl From<&InteractionMode> for InteractionModeLabel {
    fn from(interaction_mode: &InteractionMode) -> Self {
        match interaction_mode {
            InteractionMode::None => Self::None,
            InteractionMode::Pressing { .. } => Self::Pressing,
            InteractionMode::Translating { .. } => Self::Translating,
            InteractionMode::Resizing { .. } => Self::Resizing,
            InteractionMode::Rotating { .. } => Self::Rotating,
            InteractionMode::Dragging { .. } => Self::Dragging,
            InteractionMode::Inserting { .. } => Self::Inserting,
        }
    }
}

#[derive(Debug, Default, PartialEq, Copy, Clone, serde::Serialize, specta::Type)]
pub struct CursorChangeOutputEvent {
    pub cursor: Cursor,
}

#[derive(Debug, Default, PartialEq, Copy, Clone, serde::Serialize, specta::Type)]
#[serde(tag = "type")]
pub enum Cursor {
    #[default]
    Default,
    Grabbing,
    Crosshair,
    #[serde(rename_all = "camelCase")]
    Resize {
        rotation_deg: f32,
    },
    #[serde(rename_all = "camelCase")]
    Rotate {
        rotation_deg: f32,
    },
}
