use std::collections::HashMap;

use bevy_ecs::{entity::Entity, system::Resource};
use serde::Deserialize;
use specta::Type;
use std::collections::HashSet;

#[derive(Resource, Debug, Default)]
pub struct TrackedEntities {
    pub entities: HashMap<Entity, HashSet<TrackableMixinType>>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Hash, Type)]
#[serde(tag = "type")]
pub enum TrackableMixinType {
    Dimension,
    RelativeTransform,
}
