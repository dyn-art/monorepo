use std::collections::HashMap;

use bevy_ecs::{entity::Entity, system::Resource};
use dyn_composition::core::modules::node::components::types::NodeType;

use super::RenderChange;

#[derive(Resource, Default, Debug)]
pub struct ChangedComponents {
    pub changes: HashMap<Entity, (NodeType, Vec<RenderChange>)>,
}
