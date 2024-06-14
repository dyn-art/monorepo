use crate::modules::watch::events::{
    CanvasChangeOutputEvent, CursorChangeOutputEvent, InteractionModeChangeOutputEvent,
    InteractionToolChangeOutputEvent, SelectionChangeOutputEvent, WatchedEntityChangesOutputEvent,
};
use dyn_cnv_bundles::events::CoreInputEvent;
use dyn_cnv_interaction::events::InteractionInputEvent;
use dyn_cnv_svg_builder::events::SvgElementChangesOutputEvent;

#[derive(Debug, Clone, serde::Serialize, specta::Type)]
#[serde(tag = "type")]
pub enum SvgCnvOutputEvent {
    SvgElementChange(SvgElementChangesOutputEvent),
    CanvasChange(CanvasChangeOutputEvent),
    WatchedEntityChange(WatchedEntityChangesOutputEvent),
    SelectionChange(SelectionChangeOutputEvent),
    InteractionModeChange(InteractionModeChangeOutputEvent),
    InteractionToolChange(InteractionToolChangeOutputEvent),
    CursorChange(CursorChangeOutputEvent),
}

#[derive(Debug, Clone, serde::Deserialize, specta::Type)]
#[serde(tag = "type")]
pub enum SvgCnvInputEvent {
    Core { event: CoreInputEvent },
    Interaction { event: InteractionInputEvent },
}
