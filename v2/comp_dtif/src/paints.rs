use crate::ToEcsBundleImpl;
use dyn_comp_types::{
    bundles::SolidPaintBundle,
    common::Color,
    paints::{CompPaint, CompPaintVariant, SolidCompPaint},
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
}

impl ToEcsBundleImpl for SolidPaint {
    type Bundle = SolidPaintBundle;

    fn to_ecs_bundle(&self) -> Self::Bundle {
        SolidPaintBundle {
            paint: CompPaint {
                variant: CompPaintVariant::Solid,
            },
            solid: SolidCompPaint { color: self.color },
        }
    }
}
