use crate::svg_element::SVGElementId;
use bevy_ecs::prelude::Entity;

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, specta::Type),
    serde(tag = "type")
)]
pub enum ElementChangeEvent {
    ElementCreated(ElementCreatedEvent),
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct ElementCreatedEvent {
    pub tag_name: &'static str,
    pub attributes: Vec<String>,
    pub styles: Vec<String>,
    pub parent_id: Option<SVGElementId>,
    pub entity: Option<Entity>,
}
