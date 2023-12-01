use bevy_ecs::{entity::Entity, system::Resource};

#[derive(Resource, Debug)]
pub struct CompositionRes {
    pub version: String,
    pub name: String,
    pub width: f32,
    pub height: f32,
    pub root_node: Entity,
}
