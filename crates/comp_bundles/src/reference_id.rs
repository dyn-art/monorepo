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
    Entity {
        entity: Entity,
    },
    #[cfg_attr(feature = "serde_support", serde(rename_all = "camelCase"))]
    ReferenceId {
        reference_id: ReferenceId,
    },
}

impl ReferenceIdOrEntity {
    pub fn entity(entity: Entity) -> Self {
        Self::Entity { entity }
    }

    pub fn reference_id(reference_id: ReferenceId) -> Self {
        Self::ReferenceId { reference_id }
    }

    pub fn get_entity(
        &self,
        reference_id_to_entity: &HashMap<ReferenceId, Entity>,
    ) -> Option<Entity> {
        match self {
            ReferenceIdOrEntity::Entity { entity } => Some(*entity),
            ReferenceIdOrEntity::ReferenceId { reference_id } => {
                reference_id_to_entity.get(reference_id).copied()
            }
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
    #[cfg_attr(feature = "serde_support", serde(rename_all = "camelCase"))]
    ImageId { image_id: ImageId },
    #[cfg_attr(feature = "serde_support", serde(rename_all = "camelCase"))]
    ReferenceId { reference_id: ReferenceId },
}

impl ReferenceIdOrImageId {
    pub fn image_id(image_id: ImageId) -> Self {
        Self::ImageId { image_id }
    }

    pub fn reference_id(reference_id: ReferenceId) -> Self {
        Self::ReferenceId { reference_id }
    }

    pub fn get_image_id(
        &self,
        reference_id_to_asset_id: &HashMap<ReferenceId, AssetId>,
    ) -> Option<ImageId> {
        match self {
            ReferenceIdOrImageId::ImageId { image_id } => Some(*image_id),
            ReferenceIdOrImageId::ReferenceId { reference_id } => {
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
