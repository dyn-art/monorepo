use super::SvgElementId;
use bevy_ecs::prelude::Entity;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_support", derive(serde::Serialize, specta::Type))]
pub struct SvgElementChanges {
    pub id: SvgElementId,
    pub changes: Vec<SvgElementChange>,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, specta::Type),
    serde(tag = "type")
)]
pub enum SvgElementChange {
    ElementCreated(SvgElementCreatedChange),
    ElementDeleted(SvgElementDeletedChange),
    ElementAppended(SvgElementAppendedChange),
    AttributeUpdated(SvgAttributeUpdatedChange),
    AttributeRemoved(SvgAttributeRemovedChange),
    StyleUpdated(SvgStyleUpdatedChange),
    StyleRemoved(SvgStyleRemovedChange),
    ElementChildrenReordered(SvgElementChildrenReorderedChange),
}

/// Emitted when a new SvgElement is created.
#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct SvgElementCreatedChange {
    pub tag_name: &'static str,
    pub attributes: Vec<(&'static str, String)>,
    pub styles: Vec<(&'static str, String)>,
    pub parent_id: Option<SvgElementId>,
    pub entity: Option<Entity>,
}

/// Emitted when a new SvgElement is deleted.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_support", derive(serde::Serialize, specta::Type))]
pub struct SvgElementDeletedChange {}

/// Emitted when a SvgElement (child) is append to another SvgElement (parent).
#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct SvgElementAppendedChange {
    pub parent_id: SvgElementId,
}

/// Emitted when an attribute of an SvgElement is updated.
#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct SvgAttributeUpdatedChange {
    pub key: &'static str,
    pub new_value: String,
}

/// Emitted when an attribute of a SvgElement is removed.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_support", derive(serde::Serialize, specta::Type))]
pub struct SvgAttributeRemovedChange {
    pub key: &'static str,
}

/// Emitted when a style property of a SvgElement is updated.
#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct SvgStyleUpdatedChange {
    pub key: &'static str,
    pub new_value: String,
}

/// Emitted when a style property of a SvgElement is removed.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_support", derive(serde::Serialize, specta::Type))]
pub struct SvgStyleRemovedChange {
    pub key: &'static str,
}

/// Emitted when children of a SvgElement are reordered.
#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct SvgElementChildrenReorderedChange {
    pub new_order: Vec<SvgElementId>,
}
