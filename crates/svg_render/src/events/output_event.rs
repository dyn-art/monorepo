use dyn_composition::core::utils::continuous_id::ContinuousId;
use serde::Serialize;
use specta::Type;

use crate::render_change::RenderChange;

#[derive(Debug, Serialize, Clone, Type)]
pub struct RenderUpdateEvent {
    pub id: ContinuousId,
    pub updates: Vec<RenderChange>,
}
