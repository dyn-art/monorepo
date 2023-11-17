use dyn_composition::core::modules::{
    composition::events::CoreInputEvent, interactive_composition::events::InteractionInputEvent,
};
use serde::Deserialize;
use specta::Type;

#[derive(Debug, Deserialize, Type, Clone)]
#[serde(tag = "type")]
pub enum AnyInputEvent {
    Core(AnyCoreInputEvent),
    Interaction(AnyInteractionInputEvent),
}

#[derive(Debug, Deserialize, Type, Clone)]
pub struct AnyCoreInputEvent {
    pub events: Vec<CoreInputEvent>,
}

#[derive(Debug, Deserialize, Type, Clone)]
pub struct AnyInteractionInputEvent {
    pub events: Vec<InteractionInputEvent>,
}
