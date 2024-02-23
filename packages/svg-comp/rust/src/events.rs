use dyn_comp_interaction::events::InteractionInputEvent;
use dyn_comp_svg_builder::events::SvgElementChangesEvent;
use dyn_comp_types::events::CompInputEvent;

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
