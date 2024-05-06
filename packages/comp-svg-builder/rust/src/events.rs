use crate::modules::watch::events::{
    CompositionChangeOutputEvent, CursorChangeOutputEvent, InteractionModeChangeOutputEvent,
    InteractionToolChangeOutputEvent, SelectionChangeOutputEvent, WatchedEntityChangesOutputEvent,
};
use dyn_comp_bundles::events::CompCoreInputEvent;
use dyn_comp_dtif::events::DtifInputEvent;
use dyn_comp_interaction::events::InteractionInputEvent;
use dyn_comp_svg_builder::events::SvgElementChangesOutputEvent;

#[derive(Debug, Clone, serde::Serialize, specta::Type)]
#[serde(tag = "type")]
pub enum SvgCompOutputEvent {
    SvgElementChange(SvgElementChangesOutputEvent),
    CompositionChange(CompositionChangeOutputEvent),
    WatchedEntityChange(WatchedEntityChangesOutputEvent),
    SelectionChange(SelectionChangeOutputEvent),
    InteractionModeChange(InteractionModeChangeOutputEvent),
    InteractionToolChange(InteractionToolChangeOutputEvent),
    CursorChange(CursorChangeOutputEvent),
}

#[derive(Debug, Clone, serde::Deserialize, specta::Type)]
#[serde(tag = "type")]
pub enum SvgCompInputEvent {
    Composition { event: CompCoreInputEvent },
    Interaction { event: InteractionInputEvent },
    Dtif { event: DtifInputEvent },
}