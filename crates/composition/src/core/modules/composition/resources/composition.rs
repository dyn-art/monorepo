use bevy_ecs::{entity::Entity, system::Resource};
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Resource, Debug)]
pub struct CompositionRes {
    pub version: String,
    pub name: String,
    pub root_node: Entity,
    pub view_box: ViewBox,
    pub width: f32,
    pub height: f32,
}

// https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/viewBox
#[derive(Debug, Serialize, Deserialize, Type, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ViewBox {
    pub width: f32,
    pub height: f32,
    pub min_x: f32,
    pub min_y: f32,
}
