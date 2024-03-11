use crate::{
    asset::{Asset, AssetContent, AssetContentType, ImageAsset},
    asset_id::{AssetId, FontId, ImageId, InnerImageId},
};
use bevy_ecs::system::Resource;
use imagesize::{blob_size, image_type};
use slotmap::SlotMap;
use std::sync::Arc;

#[derive(Resource, Debug, Default)]
pub struct AssetDatabaseRes {
    font_db: fontdb::Database,
    image_db: SlotMap<InnerImageId, ImageAsset>,
}

impl AssetDatabaseRes {
    pub fn get_font_db(&self) -> &fontdb::Database {
        &self.font_db
    }

    pub fn get_image(&self, id: ImageId) -> Option<&ImageAsset> {
        self.image_db.get(id.0)
    }

    pub fn insert_asset(&mut self, asset: Asset) -> Option<AssetId> {
        match asset.content_type {
            AssetContentType::Ttf => {
                if let Some(font_id) = self.insert_as_font(asset) {
                    Some(AssetId::Font(font_id))
                } else {
                    None
                }
            }
            AssetContentType::Jpeg | AssetContentType::Png => {
                if let Some(image_id) = self.insert_as_image(asset) {
                    Some(AssetId::Image(image_id))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn insert_as_image(&mut self, asset: Asset) -> Option<ImageId> {
        match asset.content {
            AssetContent::Binary { content } => {
                if let Ok(image_type) = image_type(&content) {
                    if let Ok(image_size) = blob_size(&content) {
                        let image_asset = ImageAsset {
                            content,
                            width: u16::try_from(image_size.width).unwrap(), // TODO: Handle if too large image provided (> 65k pixel)
                            height: u16::try_from(image_size.height).unwrap(),
                            image_type,
                        };
                        Some(ImageId(self.image_db.insert(image_asset)))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn insert_as_font(&mut self, asset: Asset) -> Option<FontId> {
        match asset.content_type {
            AssetContentType::Ttf => match asset.content {
                AssetContent::Binary { content } => {
                    let font_face_ids = self
                        .font_db
                        .load_font_source(fontdb::Source::Binary(Arc::new(content)));
                    if font_face_ids.len() > 0 {
                        Some(FontId(font_face_ids))
                    } else {
                        None
                    }
                }
                _ => None,
            },
            _ => None,
        }
    }
}
