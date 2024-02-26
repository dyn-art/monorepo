use crate::svg::svg_element::element_changes::SvgElementChanges;

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, specta::Type),
    serde(tag = "type")
)]
pub enum SvgBuilderOutputEvent {
    /// Represents incremental changes to an SVG element.
    /// Emitted when the "output_svg_element_changes" feature is enabled.
    SvgElementChanges(SvgElementChangesOutputEvent),
    /// Contains the complete SVG as a string.
    /// Emitted when the "output_svg_string" feature is enabled.
    SvgString(SvgStringOutputEvent),
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_support", derive(serde::Serialize, specta::Type))]
pub struct SvgElementChangesOutputEvent {
    pub changes: SvgElementChanges,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_support", derive(serde::Serialize, specta::Type))]
pub struct SvgStringOutputEvent {
    pub value: String,
}
