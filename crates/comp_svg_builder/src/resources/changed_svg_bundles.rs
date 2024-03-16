use crate::svg::svg_element::element_changes::SvgElementChanges;
use bevy_ecs::{entity::Entity, system::Resource};

#[derive(Resource, Debug, Default)]
pub struct ChangedSvgBundlesRes {
    changes: Vec<ChangedSvgBundle>,
    deferred_changes: Vec<SvgElementChanges>,
}

impl ChangedSvgBundlesRes {
    pub fn drain_changes(&mut self) -> Vec<ChangedSvgBundle> {
        self.changes.drain(..).collect()
    }

    pub fn push_change(&mut self, change: ChangedSvgBundle) {
        self.changes.push(change)
    }

    pub fn drain_deferred_changes(&mut self) -> Vec<SvgElementChanges> {
        self.deferred_changes.drain(..).collect()
    }

    pub fn push_deferred_change(&mut self, change: SvgElementChanges) {
        self.deferred_changes.push(change)
    }
}

#[derive(Debug, Clone)]
pub struct ChangedSvgBundle {
    pub parent_entity: Option<Entity>,
    pub entity: Entity,
    pub elements_changes: Vec<SvgElementChanges>,
    pub index: usize,
}
