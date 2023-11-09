use bevy_ecs::{component::Component, entity::Entity};

#[derive(Component, Debug)]
pub struct CompositionMixin {
    pub version: String,
    pub name: String,
    pub width: f32,
    pub height: f32,
    pub root_node: Entity,
}
