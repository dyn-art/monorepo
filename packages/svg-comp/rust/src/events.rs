use dyn_comp_interaction::events::InteractionInputEvent;
use dyn_comp_svg_builder::events::SvgElementChangesOutputEvent;
use dyn_comp_types::events::CompCoreInputEvent;

use crate::modules::watch::events::{SelectionChangeOutputEvent, WatchedEntityChangesOutputEvent};

#[derive(Debug, Clone, serde::Serialize, specta::Type)]
#[serde(tag = "type")]
pub enum SvgCompOutputEvent {
    ElementChanges(SvgElementChangesOutputEvent),
    WatchedEntityChanges(WatchedEntityChangesOutputEvent),
    SelectionChange(SelectionChangeOutputEvent),
}

#[derive(Debug, Clone, serde::Deserialize, specta::Type)]
#[serde(tag = "type")]
pub enum SvgCompInputEvent {
    Comp { event: CompCoreInputEvent },
    Interaction { event: InteractionInputEvent },
}
