use std::collections::HashMap;

use bevy_ecs::{entity::Entity, system::Resource};
use dyn_composition::core::modules::node::components::{
    mixins::{DimensionMixin, Paint},
    types::NodeType,
};

use crate::mixin_change::MixinChange;

#[derive(Resource, Debug, Default)]
pub struct ChangedComponentsRes {
    pub changed_nodes: HashMap<Entity, ChangedNode>,
    pub changed_paints: HashMap<Entity, ChangedPaint>,
}

#[derive(Debug, Clone)]
pub struct ChangedNode {
    pub node_type: NodeType,
    pub parent_id: Option<Entity>,
    pub changes: Vec<MixinChange>,
}

#[derive(Debug)]
pub struct ChangedPaint {
    pub paint: Paint,
    pub parent_id: Option<Entity>,
    pub parent_dimension: Option<DimensionMixin>,
}
