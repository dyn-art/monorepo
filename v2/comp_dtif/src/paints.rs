use crate::{dtif_injector::DtifInjector, ToEcsBundleImpl};
use dyn_comp_asset::asset_id::AssetId;
use dyn_comp_common::{
    bundles::{ImagePaintBundle, SolidPaintBundle},
    common::{BlendMode, Color, ImageScaleMode, Opacity},
    mixins::{BlendModeMixin, ImageAssetMixin, OpacityMixin},
    paints::{CompPaint, CompPaintVariant, ImageCompPaint, SolidCompPaint},
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(tag = "type")]
pub enum Paint {
    Solid(SolidPaint),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct SolidPaint {
    pub color: Color,
    #[serde(default)]
    pub blend_mode: BlendMode,
    #[serde(default)]
    pub opacity: Opacity,
}

impl ToEcsBundleImpl for SolidPaint {
    type Bundle = SolidPaintBundle;

    fn to_ecs_bundle(&self, _: &DtifInjector) -> Self::Bundle {
        SolidPaintBundle {
            paint: CompPaint {
                variant: CompPaintVariant::Solid,
            },
            solid: SolidCompPaint { color: self.color },
            blend_mode: BlendModeMixin(self.blend_mode),
            opacity: OpacityMixin(self.opacity),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ImagePaint {
    pub asset_id: String,
    #[serde(default)]
    pub scale_mode: ImageScaleMode,
    #[serde(default)]
    pub blend_mode: BlendMode,
    #[serde(default)]
    pub opacity: Opacity,
}

impl ToEcsBundleImpl for ImagePaint {
    type Bundle = ImagePaintBundle;

    fn to_ecs_bundle(&self, dtif_injector: &DtifInjector) -> Self::Bundle {
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
            blend_mode: BlendModeMixin(self.blend_mode),
            opacity: OpacityMixin(self.opacity),
        }
    }
}
