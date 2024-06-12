#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(
    feature = "specta_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct Rect<T> {
    pub left: T,
    pub right: T,
    pub top: T,
    pub bottom: T,
}
