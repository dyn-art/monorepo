use std::collections::HashMap;

use serde::Serialize;
use serde_with::serde_as;
use specta::Type;

use super::attributes::SVGAttribute;

#[derive(Debug, Serialize, Clone, Type)]
pub enum RenderChange {
    ElementCreated(ElementCreated),
    ElementDeleted(ElementDeleted),
    AttributeUpdated(AttributeUpdated),
    StyleUpdated(StyleUpdated),
    ElementUpdated(ElementUpdated),
}

/// Emitted when a new SVGElement is created
#[serde_as]
#[derive(Debug, Serialize, Clone, Type)]
pub struct ElementCreated {
    #[serde(rename = "tagName")]
    pub tag_name: &'static str,
    pub attributes: Vec<SVGAttribute>,
    #[serde_as(as = "Vec<(_, _)>")]
    pub styles: HashMap<String, String>,
    #[serde(rename = "parentId")]
    pub parent_id: Option<u32>, // Optional parent ID, if it's a child element
}

/// Emitted when an SVGElement is deleted
#[derive(Debug, Serialize, Clone, Type)]
pub struct ElementDeleted;

/// Emitted when an attribute of an SVGElement is updated
#[derive(Debug, Serialize, Clone, Type)]
pub struct AttributeUpdated {
    pub name: &'static str,
    #[serde(rename = "newValue")]
    pub new_value: Option<SVGAttribute>, // None indicates removal of the attribute
}

/// Emitted when a style property of an SVGElement is updated
#[derive(Debug, Serialize, Clone, Type)]
pub struct StyleUpdated {
    pub name: String,
    #[serde(rename = "newValue")]
    pub new_value: Option<String>, // None indicates removal of the style
}

/// Emitted for bulk updates to an SVGElement
#[derive(Debug, Serialize, Clone, Type)]
pub struct ElementUpdated {
    #[serde(rename = "updatedAttributes")]
    pub updated_attributes: HashMap<String, String>,
    #[serde(rename = "updatedStyles")]
    pub updated_styles: HashMap<String, String>,
}
