use std::collections::HashMap;

use bevy_ecs::{entity::Entity, system::Resource};
use std::collections::HashSet;

use crate::core::modules::track::mixin_change::MixinType;

#[derive(Resource, Debug, Default)]
pub struct TrackedEntitiesRes {
    pub tracked_entities: HashMap<Entity, HashSet<MixinType>>,
}
