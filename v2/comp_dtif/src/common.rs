use crate::dtif_injector::DtifInjector;
use dyn_comp_common::common::{BlendMode, Fill, Opacity, Stroke};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct DtifStroke {
    pub fill: DtifFill,
    pub width: f32,
}

impl DtifStroke {
    pub fn to_storke(&self, dtif_injector: &DtifInjector) -> Option<Stroke> {
        self.fill.to_fill(dtif_injector).map(|fill| Stroke {
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
    pub fn to_fill(&self, dtif_injector: &DtifInjector) -> Option<Fill> {
        dtif_injector
            .get_sid_to_entity()
            .get(&self.paint_id)
            .map(|entity| Fill {
                paint: *entity,
                blend_mode: self.blend_mode,
                opacity: self.opacity,
            })
    }
}
