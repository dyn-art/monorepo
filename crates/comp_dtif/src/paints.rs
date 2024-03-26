use crate::{dtif_injector::DtifInjector, ToEcsBundleImpl};
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

impl ToEcsBundleImpl for SolidPaint {
    type Bundle = SolidPaintBundle;

    fn to_ecs_bundle(&self, _: &DtifInjector) -> Self::Bundle {
        Self::Bundle {
            paint: CompPaint {
                variant: CompPaintVariant::Solid,
            },
            solid: SolidCompPaint { color: self.color },
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ImagePaint {
    pub asset_id: String,
    #[serde(default)]
    pub scale_mode: ImageScaleMode,
}

impl ToEcsBundleImpl for ImagePaint {
    type Bundle = ImagePaintBundle;

    fn to_ecs_bundle(&self, dtif_injector: &DtifInjector) -> Self::Bundle {
        Self::Bundle {
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct GradientPaint {
    pub variant: GradientVariant,
    pub stops: Vec<GradientColorStop>,
}

impl ToEcsBundleImpl for GradientPaint {
    type Bundle = GradientPaintBundle;

    fn to_ecs_bundle(&self, _: &DtifInjector) -> Self::Bundle {
        Self::Bundle {
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
