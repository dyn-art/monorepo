use serde::Serialize;
use specta::Type;

/// Emitted when the size of the Composition is changed.
#[derive(Debug, Serialize, Clone, Type)]
pub struct SizeChanged {
    width: f32,
    height: f32,
}

/// Emitted when the view box of the Composition is changed.
#[derive(Debug, Serialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct ViewBoxChanged {
    width: f32,
    height: f32,
    min_x: f32,
    min_y: f32,
}
