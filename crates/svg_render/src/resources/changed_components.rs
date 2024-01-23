use std::collections::HashMap;

use bevy_ecs::{entity::Entity, system::Resource};
use dyn_composition::core::modules::node::components::types::{NodeType, PaintType};

use crate::mixin_change::{NodeMixinChange, PaintMixinChange};

#[derive(Resource, Debug, Default)]
pub struct ChangedComponentsRes {
    pub changed_nodes: HashMap<Entity, ChangedNode>,
    pub changed_paints: HashMap<Entity, ChangedPaint>,
}

#[derive(Debug, Clone)]
pub struct ChangedNode {
    pub node_type: NodeType,
    pub parent_id: Option<Entity>,
    pub changes: Vec<NodeMixinChange>,
}

#[derive(Debug)]
pub struct ChangedPaint {
    pub paint_type: PaintType,
    pub parent_id: Option<Entity>,
    pub changes: Vec<PaintMixinChange>,
}
