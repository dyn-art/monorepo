use dyn_utils::properties::size::Size;
use glam::Vec2;

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct Viewport {
    pub physical_position: Vec2,
    pub physical_size: Size,
}
