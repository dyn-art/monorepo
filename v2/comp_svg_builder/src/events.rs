use crate::svg::svg_element::element_changes::SVGElementChanges;

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, specta::Type),
    serde(tag = "type")
)]
pub enum SVGRenderOutputEvent {
    ElementChanges(SVGElementChangesEvent),
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_support", derive(serde::Serialize, specta::Type))]
pub struct SVGElementChangesEvent(pub SVGElementChanges);
