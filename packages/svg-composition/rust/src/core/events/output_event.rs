use serde::Serialize;
use specta::Type;

use crate::core::modules::svg_render::resources::svg_composition::svg_element::events::RenderChange;

#[derive(Debug, Serialize, Clone, Type)]
#[serde(tag = "type")]
pub enum OutputEvent {
    RenderUpdate(RenderUpdateEvent),
}

#[derive(Debug, Serialize, Clone, Type)]
pub struct RenderUpdateEvent {
    pub id: u32,
    pub updates: Vec<RenderChange>,
}
