use bevy_ecs::entity::Entity;
use dyn_composition::utils::continuous_id::ContinuousId;
use serde::Serialize;
use specta::Type;

use crate::resources::svg_composition::svg_element::{attributes::SVGAttribute, styles::SVGStyle};

#[derive(Debug, Serialize, Clone, Type)]
#[serde(tag = "type")]
pub enum ElementChange {
    ElementCreated(ElementCreated),
    ElementDeleted(ElementDeleted),
    ElementAppended(ElementAppended),
    AttributeUpdated(AttributeUpdated),
    AttributeRemoved(AttributeRemoved),
    StyleUpdated(StyleUpdated),
    StyleRemoved(StyleRemoved),
}

/// Emitted when a new SVGElement is created.
#[derive(Debug, Serialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct ElementCreated {
    pub tag_name: &'static str,
    pub attributes: Vec<SVGAttribute>,
    pub styles: Vec<SVGStyle>,
    pub parent_id: Option<ContinuousId>,
    pub is_bundle_root: bool,
    pub entity: Entity,
}

/// Emitted when a SVGElement is deleted.
#[derive(Debug, Serialize, Clone, Type)]
pub struct ElementDeleted {}

/// Emitted when a SVGElement (child) is append to another SVGElement (parent).
#[derive(Debug, Serialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct ElementAppended {
    pub parent_id: ContinuousId,
}

/// Emitted when an attribute of an SVGElement is updated.
#[derive(Debug, Serialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct AttributeUpdated {
    pub new_value: SVGAttribute,
}

/// Emitted when an attribute of a SVGElement is removed.
#[derive(Debug, Serialize, Clone, Type)]
pub struct AttributeRemoved {
    pub key: &'static str,
}

/// Emitted when a style property of a SVGElement is updated.
#[derive(Debug, Serialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct StyleUpdated {
    pub new_value: SVGStyle,
}

/// Emitted when a style property of a SVGElement is removed.
#[derive(Debug, Serialize, Clone, Type)]
pub struct StyleRemoved {
    key: &'static str,
}
