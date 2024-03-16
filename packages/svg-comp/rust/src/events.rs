use crate::modules::watch::events::{
    CompositionChangeOutputEvent, InteractionModeChangeOutputEvent, SelectionChangeOutputEvent,
    WatchedEntityChangeOutputEvent,
};
use dyn_comp_common::events::CompCoreInputEvent;
use dyn_comp_interaction::events::InteractionInputEvent;
use dyn_comp_svg_builder::events::SvgElementChangeOutputEvent;

#[derive(Debug, Clone, serde::Serialize, specta::Type)]
#[serde(tag = "type")]
pub enum SvgCompOutputEvent {
    SvgElementChange(SvgElementChangeOutputEvent),
    CompositionChange(CompositionChangeOutputEvent),
    WatchedEntityChange(WatchedEntityChangeOutputEvent),
    SelectionChange(SelectionChangeOutputEvent),
    InteractionModeChange(InteractionModeChangeOutputEvent),
}

#[derive(Debug, Clone, serde::Deserialize, specta::Type)]
#[serde(tag = "type")]
pub enum SvgCompInputEvent {
    Composition { event: CompCoreInputEvent },
    Interaction { event: InteractionInputEvent },
}
