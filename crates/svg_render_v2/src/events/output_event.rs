use dyn_composition::core::utils::continuous_id::ContinuousId;
use serde::Serialize;
use specta::Type;

use crate::element_change::ElementChange;

#[derive(Debug, Serialize, Clone, Type)]
#[serde(tag = "type")]
pub enum SVGRenderOutputEvent {
    ElementChange(ElementChangeEvent),
}

#[derive(Debug, Serialize, Clone, Type)]
pub struct ElementChangeEvent {
    pub id: ContinuousId,
    pub changes: Vec<ElementChange>,
}
