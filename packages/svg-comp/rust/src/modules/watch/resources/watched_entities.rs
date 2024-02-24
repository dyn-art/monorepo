use bevy_ecs::{entity::Entity, system::Resource};
use std::collections::{HashMap, HashSet};

#[derive(Resource, Debug, Default)]
pub struct WatchedEntitiesRes {
    pub watched_entities: HashMap<Entity, HashSet<WatchableMixinVariant>>,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, specta::Type)]
pub enum WatchableMixinVariant {
    Dimension,
    Transform,
}
