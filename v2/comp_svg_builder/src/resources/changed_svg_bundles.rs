use crate::svg::svg_element::element_changes::SvgElementChanges;
use bevy_ecs::{entity::Entity, system::Resource};

#[derive(Resource, Debug, Default)]
pub struct ChangedSvgBundlesRes {
    changes: Vec<ChangedSvgBundle>,
}

impl ChangedSvgBundlesRes {
    pub fn drain(&mut self) -> Vec<ChangedSvgBundle> {
        self.changes.drain(..).collect()
    }

    pub fn push_change(&mut self, change: ChangedSvgBundle) {
        self.changes.push(change)
    }
}

#[derive(Debug, Clone)]
pub struct ChangedSvgBundle {
    pub parent_entity: Option<Entity>,
    pub entity: Entity,
    pub elements_changes: Vec<SvgElementChanges>,
    pub index: usize,
}
