use bevy_ecs::entity::Entity;
use dyn_comp_asset::asset_id::{AssetId, ImageId};
use std::collections::HashMap;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct ReferenceId(String);

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(tag = "type")
)]
pub enum ReferenceIdOrEntity {
    Entity { value: Entity },
    ReferenceId { value: ReferenceId },
}

impl ReferenceIdOrEntity {
    pub fn get_entity(
        &self,
        reference_id_to_entity: &HashMap<ReferenceId, Entity>,
    ) -> Option<Entity> {
        match self {
            ReferenceIdOrEntity::Entity { value: entity } => Some(*entity),
            ReferenceIdOrEntity::ReferenceId {
                value: reference_id,
            } => reference_id_to_entity.get(reference_id).copied(),
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(tag = "type")
)]
pub enum ReferenceIdOrImageId {
    ImageId { value: ImageId },
    ReferenceId { value: ReferenceId },
}

impl ReferenceIdOrImageId {
    pub fn get_image_id(
        &self,
        reference_id_to_asset_id: &HashMap<ReferenceId, AssetId>,
    ) -> Option<ImageId> {
        match self {
            ReferenceIdOrImageId::ImageId { value: image_id } => Some(*image_id),
            ReferenceIdOrImageId::ReferenceId {
                value: reference_id,
            } => {
                if let Some(asset_id) = reference_id_to_asset_id.get(reference_id) {
                    match asset_id {
                        AssetId::Image(image_id) => Some(*image_id),
                        _ => None,
                    }
                } else {
                    None
                }
            }
        }
    }
}
