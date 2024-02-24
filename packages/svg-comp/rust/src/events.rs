use dyn_comp_interaction::events::InteractionInputEvent;
use dyn_comp_svg_builder::events::SvgElementChangesEvent;
use dyn_comp_types::events::CompInputEvent;

#[derive(Debug, Clone, serde::Serialize, specta::Type)]
#[serde(tag = "type")]
pub enum SvgCompOutputEvent {
    ElementChanges(SvgElementChangesEvent),
}

#[derive(Debug, Clone, serde::Deserialize, specta::Type)]
#[serde(tag = "type")]
pub enum SvgCompInputEvent {
    Comp { event: CompInputEvent },
    Interaction { event: InteractionInputEvent },
}
