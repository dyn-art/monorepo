use serde::Serialize;
use specta::Type;

use crate::resources::svg_composition::events::{SizeChanged, ViewBoxChanged};

/// Represents the different types of events that can be emitted by the SVGComposition
/// to synchronize its state with the frontend.
#[derive(Debug, Serialize, Clone, Type)]
#[serde(tag = "type")]
pub enum CompositionChange {
    SizeChanged(SizeChanged),
    ViewBoxChanged(ViewBoxChanged),
}
