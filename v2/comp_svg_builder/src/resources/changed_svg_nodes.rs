use bevy_ecs::{entity::Entity, system::Resource};

use crate::svg::svg_element::element_changes::SVGElementChanges;

#[derive(Resource, Debug, Default)]
pub struct ChangedSVGNodesRes {
    pub changes: Vec<ChangedSVGNode>,
}

#[derive(Debug, Clone)]
pub struct ChangedSVGNode {
    pub parent_entity: Option<Entity>,
    pub entity: Entity,
    pub changes: Vec<SVGElementChanges>,
    pub index: usize,
}
