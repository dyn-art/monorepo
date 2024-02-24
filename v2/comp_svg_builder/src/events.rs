use crate::svg::svg_element::element_changes::SvgElementChanges;

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, specta::Type),
    serde(tag = "type")
)]
pub enum SvgBuilderOutputEvent {
    ElementChanges(SvgElementChangesOutputEvent),
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_support", derive(serde::Serialize, specta::Type))]
pub struct SvgElementChangesOutputEvent {
    pub changes: SvgElementChanges,
}
