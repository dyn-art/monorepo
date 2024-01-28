use bevy_ecs::entity::Entity;
use dyn_composition::utils::continuous_id::ContinuousId;
use serde::Serialize;
use serde_with::serde_as;
use specta::Type;

use super::{attributes::SVGAttribute, styles::SVGStyle};

/// Emitted when a new SVGElement is created.
#[serde_as]
#[derive(Debug, Serialize, Clone, Type)]
pub struct ElementCreated {
    #[serde(rename = "tagName")]
    pub tag_name: &'static str,
    pub attributes: Vec<SVGAttribute>,
    pub styles: Vec<SVGStyle>,
    #[serde(rename = "parentId")]
    pub parent_id: Option<ContinuousId>,
    #[serde(rename = "isBundleRoot")]
    pub is_bundle_root: bool,
    pub entity: Option<Entity>,
}

/// Emitted when a SVGElement is deleted.
#[derive(Debug, Serialize, Clone, Type)]
pub struct ElementDeleted {}

/// Emitted when a SVGElement (child) is append to another SVGElement (parent).
#[derive(Debug, Serialize, Clone, Type)]
pub struct ElementAppended {
    #[serde(rename = "parentId")]
    pub parent_id: ContinuousId,
}

/// Emitted when an attribute of an SVGElement is updated.
#[derive(Debug, Serialize, Clone, Type)]
pub struct AttributeUpdated {
    #[serde(rename = "newValue")]
    pub new_value: SVGAttribute,
}

/// Emitted when an attribute of a SVGElement is removed.
#[derive(Debug, Serialize, Clone, Type)]
pub struct AttributeRemoved {
    pub key: &'static str,
}

/// Emitted when a style property of a SVGElement is updated.
#[derive(Debug, Serialize, Clone, Type)]
pub struct StyleUpdated {
    #[serde(rename = "newValue")]
    pub new_value: SVGStyle,
}

/// Emitted when a style property of a SVGElement is removed.
#[derive(Debug, Serialize, Clone, Type)]
pub struct StyleRemoved {
    key: &'static str,
}
