use bevy_ecs::entity::Entity;
use dyn_comp_types::common::{BlendMode, Fill, Opacity, Stroke};
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct DtifStroke {
    pub fill: DtifFill,
    pub width: f32,
}

impl DtifStroke {
    pub fn to_storke(&self, sid_to_entity: &HashMap<String, Entity>) -> Option<Stroke> {
        self.fill.to_fill(sid_to_entity).map(|fill| Stroke {
            width: self.width,
            fill,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct DtifFill {
    pub paint_id: String,
    pub blend_mode: BlendMode,
    pub opacity: Opacity,
}

impl DtifFill {
    pub fn to_fill(&self, sid_to_entity: &HashMap<String, Entity>) -> Option<Fill> {
        sid_to_entity.get(&self.paint_id).map(|entity| Fill {
            paint: *entity,
            blend_mode: self.blend_mode,
            opacity: self.opacity,
        })
    }
}
