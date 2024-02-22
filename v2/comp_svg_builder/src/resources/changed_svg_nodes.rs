use bevy_ecs::{entity::Entity, system::Resource};

use crate::svg::svg_element::element_changes::SvgElementChanges;

#[derive(Resource, Debug, Default)]
pub struct ChangedSvgNodesRes {
    pub changes: Vec<ChangedSvgNode>,
}

#[derive(Debug, Clone)]
pub struct ChangedSvgNode {
    pub parent_entity: Option<Entity>,
    pub entity: Entity,
    pub changes: Vec<SvgElementChanges>,
    pub index: usize,
}
