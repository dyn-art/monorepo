use bevy_ecs::{entity::Entity, system::Resource};
use std::collections::{HashMap, HashSet};

#[derive(Resource, Debug, Default)]
pub struct WatchedEntitiesRes {
    watched_entities: HashMap<Entity, HashSet<WatchableComponentVariant>>,
}

impl WatchedEntitiesRes {
    pub fn watch_entity(
        &mut self,
        entity: Entity,
        to_watch_components: Vec<WatchableComponentVariant>,
    ) {
        self.watched_entities
            .entry(entity)
            .or_insert_with(HashSet::new)
            .extend(to_watch_components);
    }

    pub fn unregister_entity(&mut self, entity: Entity) -> bool {
        self.watched_entities.remove(&entity).is_some()
    }

    pub fn get_watched_entities(&self) -> &HashMap<Entity, HashSet<WatchableComponentVariant>> {
        &self.watched_entities
    }
}

#[derive(
    Debug, Hash, Eq, PartialEq, Clone, Copy, serde::Serialize, serde::Deserialize, specta::Type,
)]
pub enum WatchableComponentVariant {
    Size,
    Transform,
    GlobalTransform,
}
