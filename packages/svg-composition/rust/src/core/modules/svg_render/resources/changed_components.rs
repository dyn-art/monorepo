use std::collections::HashMap;

use bevy_ecs::{entity::Entity, system::Resource};
use dyn_composition::core::modules::node::components::{mixins::Paint, types::NodeType};

use crate::core::mixin_change::MixinChange;

#[derive(Resource, Debug, Default)]
pub struct ChangedComponents {
    pub changed_nodes: HashMap<Entity, ChangedNode>,
    pub changed_paints: HashMap<Entity, Paint>,
}

#[derive(Debug, Default)]
pub struct ChangedNode {
    pub node_type: NodeType,
    pub parent_id: Option<Entity>,
    pub changes: Vec<MixinChange>,
}
