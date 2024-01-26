use std::collections::HashMap;

use bevy_ecs::{entity::Entity, system::Resource};
use dyn_composition::core::modules::node::components::types::{NodeType, PaintType};

use crate::mixin_change::{NodeMixinChange, PaintMixinChange};

#[derive(Resource, Debug, Default)]
pub struct ChangedComponentsRes {
    pub changed_entities: HashMap<Entity, ChangedEntity>,
}

#[derive(Debug, Clone)]
pub enum ChangedEntity {
    Node(ChangedNode),
    Paint(ChangedPaint),
}

#[derive(Debug, Clone)]
pub struct ChangedNode {
    pub node_type: NodeType,
    pub parent_id: Option<Entity>,
    pub changes: Vec<NodeMixinChange>,
}

#[derive(Debug, Clone)]
pub struct ChangedPaint {
    pub paint_type: PaintType,
    pub parent_id: Option<Entity>,
    pub changes: Vec<PaintMixinChange>,
}
