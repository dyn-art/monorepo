use bevy_ecs::{entity::Entity, system::Resource};

use crate::svg::svg_element::element_changes::SvgElementChanges;

#[derive(Resource, Debug, Default)]
pub struct ChangedSvgNodesRes {
    changes: Vec<ChangedSvgNode>,
}

impl ChangedSvgNodesRes {
    pub fn drain(&mut self) -> Vec<ChangedSvgNode> {
        self.changes.drain(..).collect()
    }

    pub fn push_change(&mut self, change: ChangedSvgNode) {
        self.changes.push(change)
    }
}

#[derive(Debug, Clone)]
pub struct ChangedSvgNode {
    pub parent_entity: Option<Entity>,
    pub entity: Entity,
    pub changes: Vec<SvgElementChanges>,
    pub index: usize,
}
