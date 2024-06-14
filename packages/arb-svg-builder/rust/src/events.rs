use crate::modules::watch::events::{
    ArtboardChangeOutputEvent, CursorChangeOutputEvent, InteractionModeChangeOutputEvent,
    InteractionToolChangeOutputEvent, SelectionChangeOutputEvent, WatchedEntityChangesOutputEvent,
};
use dyn_arb_bundles::events::CoreInputEvent;
use dyn_arb_interaction::events::InteractionInputEvent;
use dyn_arb_svg_builder::events::SvgElementChangesOutputEvent;

#[derive(Debug, Clone, serde::Serialize, specta::Type)]
#[serde(tag = "type")]
pub enum SvgArbOutputEvent {
    SvgElementChange(SvgElementChangesOutputEvent),
    ArtboardChange(ArtboardChangeOutputEvent),
    WatchedEntityChange(WatchedEntityChangesOutputEvent),
    SelectionChange(SelectionChangeOutputEvent),
    InteractionModeChange(InteractionModeChangeOutputEvent),
    InteractionToolChange(InteractionToolChangeOutputEvent),
    CursorChange(CursorChangeOutputEvent),
}

#[derive(Debug, Clone, serde::Deserialize, specta::Type)]
#[serde(tag = "type")]
pub enum SvgArbInputEvent {
    Core { event: CoreInputEvent },
    Interaction { event: InteractionInputEvent },
}
