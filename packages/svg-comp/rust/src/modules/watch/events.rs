use bevy_ecs::entity::Entity;

#[derive(Debug, Clone, serde::Serialize, specta::Type)]
pub struct WatchedEntityChangesOutputEvent {
    entity: Entity,
    changes: Vec<()>,
}

#[derive(Debug, Clone, serde::Serialize, specta::Type)]
pub struct SelectionChangeOutputEvent {
    pub selected: Vec<Entity>,
}
