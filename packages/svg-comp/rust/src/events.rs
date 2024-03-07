use crate::modules::watch::events::{
    CompositionChangeOutputEvent, SelectionChangeOutputEvent, WatchedEntityChangesOutputEvent,
};
use dyn_comp_common::events::CompCoreInputEvent;
use dyn_comp_interaction::events::InteractionInputEvent;
use dyn_comp_svg_builder::events::SvgElementChangesOutputEvent;

#[derive(Debug, Clone, serde::Serialize, specta::Type)]
#[serde(tag = "type")]
pub enum SvgCompOutputEvent {
    SvgElementChanges(SvgElementChangesOutputEvent),
    CompositionChange(CompositionChangeOutputEvent),
    WatchedEntityChanges(WatchedEntityChangesOutputEvent),
    SelectionChange(SelectionChangeOutputEvent),
}

#[derive(Debug, Clone, serde::Deserialize, specta::Type)]
#[serde(tag = "type")]
pub enum SvgCompInputEvent {
    Comp { event: CompCoreInputEvent },
    Interaction { event: InteractionInputEvent },
}
