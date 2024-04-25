use crate::{dtif_handler::DtifHandler, SpawnBundleImpl};
use bevy_ecs::world::{EntityWorldMut, World};
use dyn_comp_asset::asset_id::AssetId;
use dyn_comp_bundles::{
    components::{
        mixins::ImageAssetMixin,
        paints::{
            CompPaint, CompPaintVariant, GradientColorStop, GradientCompPaint, GradientVariant,
            ImageCompPaint, ImageScaleMode, SolidCompPaint,
        },
    },
    GradientPaintBundle, ImagePaintBundle, SolidPaintBundle,
};
use dyn_utils::properties::color::Color;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(tag = "type")]
pub enum Paint {
    Solid(SolidPaint),
    Image(ImagePaint),
    Gradient(GradientPaint),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct SolidPaint {
    pub color: Color,
}

impl SolidPaint {
    fn to_ecs_bundle(&self) -> SolidPaintBundle {
        SolidPaintBundle {
            paint: CompPaint {
                variant: CompPaintVariant::Solid,
            },
            solid: SolidCompPaint { color: self.color },
        }
    }
}

impl SpawnBundleImpl for SolidPaint {
    fn spawn<'a>(&self, _: &DtifHandler, world: &'a mut World) -> EntityWorldMut<'a> {
        world.spawn(self.to_ecs_bundle())
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ImagePaint {
    pub asset_id: String,
    #[serde(default)]
    pub scale_mode: ImageScaleMode,
}

impl ImagePaint {
    fn to_ecs_bundle(&self, dtif_injector: &DtifHandler) -> ImagePaintBundle {
        ImagePaintBundle {
            paint: CompPaint {
                variant: CompPaintVariant::Image,
            },
            image: ImageCompPaint {
                scale_mode: self.scale_mode,
            },
            asset: ImageAssetMixin(
                if let Some(asset_id) = dtif_injector.get_sid_to_asset_id().get(&self.asset_id) {
                    match asset_id {
                        AssetId::Image(image_id) => Some(*image_id),
                        _ => None,
                    }
                } else {
                    None
                },
            ),
        }
    }
}

impl SpawnBundleImpl for ImagePaint {
    fn spawn<'a>(&self, dtif_injector: &DtifHandler, world: &'a mut World) -> EntityWorldMut<'a> {
        world.spawn(self.to_ecs_bundle(dtif_injector))
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct GradientPaint {
    pub variant: GradientVariant,
    pub stops: Vec<GradientColorStop>,
}

impl GradientPaint {
    fn to_ecs_bundle(&self) -> GradientPaintBundle {
        GradientPaintBundle {
            paint: CompPaint {
                variant: CompPaintVariant::Gradient,
            },
            gradient: GradientCompPaint {
                variant: self.variant,
                stops: self.stops.iter().copied().collect(),
            },
        }
    }
}

impl SpawnBundleImpl for GradientPaint {
    fn spawn<'a>(&self, _: &DtifHandler, world: &'a mut World) -> EntityWorldMut<'a> {
        world.spawn(self.to_ecs_bundle())
    }
}
