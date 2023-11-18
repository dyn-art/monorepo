use std::collections::HashMap;

use bevy_ecs::{entity::Entity, system::Resource};
use dyn_composition::core::modules::node::components::types::NodeType;

use crate::core::mixin_change::MixinChange;

#[derive(Resource, Debug, Default)]
pub struct ChangedComponents {
    pub changes: HashMap<Entity, ChangedComponent>,
}

#[derive(Debug, Default)]
pub struct ChangedComponent {
    pub node_type: NodeType,
    pub parent_id: Option<Entity>,
    pub changes: Vec<MixinChange>,
}
