use crate::modules::watch::component_change::ComponentChange;
use bevy_ecs::{entity::Entity, system::Resource};
use std::collections::HashMap;

#[derive(Resource, Debug, Default)]
pub struct ChangedComponentsRes {
    pub changed_entities: HashMap<Entity, Vec<ComponentChange>>,
}

impl ChangedComponentsRes {
    pub fn push_change(&mut self, entity: Entity, component_change: ComponentChange) {
        self.changed_entities
            .entry(entity)
            .or_insert_with(Vec::new)
            .push(component_change);
    }

    pub fn drain(&mut self) -> HashMap<Entity, Vec<ComponentChange>> {
        self.changed_entities.drain().collect()
    }
}
