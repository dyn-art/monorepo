use crate::{
    asset::{Asset, AssetContent, AssetContentType},
    asset_id::{AnyAssetId, AssetId, FontId, InnerAnyAssetId},
};
use bevy_ecs::system::Resource;
use slotmap::SlotMap;
use std::sync::Arc;

#[derive(Resource, Debug, Default)]
pub struct AssetDatabaseRes {
    font_db: fontdb::Database,
    any_asset_db: SlotMap<InnerAnyAssetId, Asset>,
}

impl AssetDatabaseRes {
    pub fn get_raw_asset(&self, key: AnyAssetId) -> Option<&Asset> {
        self.any_asset_db.get(key.0)
    }

    pub fn get_font_db(&self) -> &fontdb::Database {
        &self.font_db
    }

    pub fn insert_asset(&mut self, asset: Asset) -> Option<AssetId> {
        match asset.content_type {
            AssetContentType::TTF => {
                if let Some(font_id) = self.insert_as_font(asset) {
                    Some(AssetId::Font(font_id))
                } else {
                    None
                }
            }
            _ => Some(AssetId::Any(self.insert_any_asset(asset))),
        }
    }

    pub fn insert_any_asset(&mut self, asset: Asset) -> AnyAssetId {
        let inner_asset_id = self.any_asset_db.insert(asset);
        return AnyAssetId(inner_asset_id);
    }

    pub fn insert_as_font(&mut self, asset: Asset) -> Option<FontId> {
        match asset.content_type {
            AssetContentType::TTF => match asset.content {
                AssetContent::Binary { content } => Some(FontId(
                    self.font_db
                        .load_font_source(fontdb::Source::Binary(Arc::new(content))),
                )),
                _ => None,
            },
            _ => None,
        }
    }
}
