use dyn_composition::core::utils::continuous_id::ContinuousId;
use serde::Serialize;
use specta::Type;

use crate::{composition_change::CompositionChange, element_change::ElementChange};

#[derive(Debug, Serialize, Clone, Type)]
#[serde(tag = "type")]
pub enum SVGRenderOutputEvent {
    CompositionUpdate(CompositionUpdateEvent),
    ElementUpdate(ElementUpdateEvent),
}

#[derive(Debug, Serialize, Clone, Type)]
pub struct CompositionUpdateEvent {
    pub updates: Vec<CompositionChange>,
}

#[derive(Debug, Serialize, Clone, Type)]
pub struct ElementUpdateEvent {
    pub id: ContinuousId,
    pub updates: Vec<ElementChange>,
}
