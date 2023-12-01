use std::collections::HashMap;

use bevy_ecs::{entity::Entity, system::Resource};

use crate::core::mixin_change::MixinChange;

#[derive(Resource, Debug, Default)]
pub struct ChangedComponentsRes {
    pub changed_entities: HashMap<Entity, Vec<MixinChange>>,
}
