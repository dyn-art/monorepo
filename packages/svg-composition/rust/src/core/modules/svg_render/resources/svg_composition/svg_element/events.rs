use serde::Serialize;
use serde_with::serde_as;
use specta::Type;

use super::{attributes::SVGAttribute, styles::SVGStyle};

#[derive(Debug, Serialize, Clone, Type)]
#[serde(tag = "type")]
pub enum RenderChange {
    ElementCreated(ElementCreated),
    ElementDeleted(ElementDeleted),
    AttributeUpdated(AttributeUpdated),
    AttributeRemoved(AttributeRemoved),
    StyleUpdated(StyleUpdated),
    StyleRemoved(StyleRemoved),
}

/// Emitted when a new SVGElement is created
#[serde_as]
#[derive(Debug, Serialize, Clone, Type)]
pub struct ElementCreated {
    #[serde(rename = "tagName")]
    pub tag_name: &'static str,
    pub attributes: Vec<SVGAttribute>,
    pub styles: Vec<SVGStyle>,
    #[serde(rename = "parentId")]
    pub parent_id: Option<u32>, // Optional parent ID, if it's a child element
}

/// Emitted when an SVGElement is deleted
#[derive(Debug, Serialize, Clone, Type)]
pub struct ElementDeleted {}

/// Emitted when an attribute of an SVGElement is updated
#[derive(Debug, Serialize, Clone, Type)]
pub struct AttributeUpdated {
    #[serde(rename = "newValue")]
    pub new_value: SVGAttribute,
}

/// Emitted when an attribute of an SVGElement is removed
#[derive(Debug, Serialize, Clone, Type)]
pub struct AttributeRemoved {
    key: &'static str,
}

/// Emitted when a style property of an SVGElement is updated
#[derive(Debug, Serialize, Clone, Type)]
pub struct StyleUpdated {
    #[serde(rename = "newValue")]
    pub new_value: SVGStyle,
}

/// Emitted when a style property of an SVGElement is removed
#[derive(Debug, Serialize, Clone, Type)]
pub struct StyleRemoved {
    key: &'static str,
}
