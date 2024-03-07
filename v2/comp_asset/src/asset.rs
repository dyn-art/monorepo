use imagesize::ImageType;

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct Asset {
    pub content: AssetContent,
    pub content_type: AssetContentType,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(tag = "type")
)]
pub enum AssetContent {
    /// Content stored as binary data.
    Binary { content: Vec<u8> },
    /// Content referenced by a URL.
    Url { url: String },
}

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum AssetContentType {
    #[default]
    Unknown,
    // Image
    Jpeg,
    Png,
    // Vector
    Svg,
    // Font
    Ttf,
}

#[derive(Debug, Clone)]
pub struct ImageAsset {
    pub content: Vec<u8>,
    pub width: usize,
    pub height: usize,
    pub image_type: ImageType,
}
