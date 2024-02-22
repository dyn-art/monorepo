use dyn_comp::prelude::CompInputEvent;
use dyn_comp_interaction::events::InteractionInputEvent;
use dyn_comp_svg_builder::events::SvgElementChangesEvent;

#[derive(Debug, Clone, serde::Serialize, specta::Type)]
#[serde(tag = "type")]
pub enum SvgCompOutputEvent {
    ElementChanges(SvgElementChangesEvent),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub enum SvgCompInputEvent {
    Comp(CompInputEvent),
    Interaction(InteractionInputEvent),
}
