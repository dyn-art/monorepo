use dyn_composition::core::modules::composition::resources::composition::ViewBox;
use serde::Serialize;
use specta::Type;

/// Emitted when the size of the Composition is changed.
#[derive(Debug, Serialize, Clone, Type)]
pub struct SizeChanged {
    pub width: f32,
    pub height: f32,
}

/// Emitted when the view box of the Composition is changed.
#[derive(Debug, Serialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct ViewBoxChanged {
    pub view_box: ViewBox,
}
