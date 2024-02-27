use crate::ToEcsBundleImpl;
use bevy_ecs::entity::Entity;
use dyn_comp_types::{
    bundles::SolidPaintBundle,
    common::Color,
    paints::{CompPaint, SolidCompPaint},
};
use std::collections::HashMap;

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

    fn to_ecs_bundle(&self, _: &HashMap<String, Entity>) -> Self::Bundle {
        SolidPaintBundle {
            paint: CompPaint::default(),
            solid: SolidCompPaint { color: self.color },
        }
    }
}
