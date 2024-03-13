use super::component_change::ComponentChange;
use bevy_ecs::entity::Entity;
use dyn_comp_common::common::{Size, Viewport};

#[derive(Debug, Clone, serde::Serialize, specta::Type)]
pub struct WatchedEntityChangesOutputEvent {
    pub entity: Entity,
    pub changes: Vec<ComponentChange>,
}

#[derive(Debug, Clone, serde::Serialize, specta::Type)]
pub struct SelectionChangeOutputEvent {
    pub selected: Vec<Entity>,
}

#[derive(Debug, Clone, serde::Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct CompositionChangeOutputEvent {
    pub root_nodes: Vec<Entity>,
    pub viewport: Viewport,
    pub size: Size,
}
