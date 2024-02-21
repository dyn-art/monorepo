use bevy_ecs::prelude::Entity;

use super::SVGElementId;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_support", derive(serde::Serialize, specta::Type))]
pub struct SVGElementChanges {
    pub id: SVGElementId,
    pub changes: Vec<SVGElementChange>,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, specta::Type),
    serde(tag = "type")
)]
pub enum SVGElementChange {
    ElementCreated(SVGElementCreatedChange),
    ElementDeleted(SVGElementDeletedChange),
    ElementAppended(SVGElementAppendedChange),
    AttributeUpdated(SVGAttributeUpdatedChange),
    AttributeRemoved(SVGAttributeRemovedChange),
    StyleUpdated(SVGStyleUpdatedChange),
    StyleRemoved(SVGStyleRemovedChange),
}

/// Emitted when a new SVGElement is created.
#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct SVGElementCreatedChange {
    pub tag_name: &'static str,
    pub attributes: Vec<(&'static str, String)>,
    pub styles: Vec<(&'static str, String)>,
    pub parent_id: Option<SVGElementId>,
    pub entity: Option<Entity>,
}

/// Emitted when a new SVGElement is deleted.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_support", derive(serde::Serialize, specta::Type))]
pub struct SVGElementDeletedChange {}

/// Emitted when a SVGElement (child) is append to another SVGElement (parent).
#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct SVGElementAppendedChange {
    pub parent_id: SVGElementId,
}

/// Emitted when an attribute of an SVGElement is updated.
#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct SVGAttributeUpdatedChange {
    pub key: &'static str,
    pub new_value: String,
}

/// Emitted when an attribute of a SVGElement is removed.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_support", derive(serde::Serialize, specta::Type))]
pub struct SVGAttributeRemovedChange {
    pub key: &'static str,
}

/// Emitted when a style property of a SVGElement is updated.
#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct SVGStyleUpdatedChange {
    pub key: &'static str,
    pub new_value: String,
}

/// Emitted when a style property of a SVGElement is removed.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_support", derive(serde::Serialize, specta::Type))]
pub struct SVGStyleRemovedChange {
    pub key: &'static str,
}
