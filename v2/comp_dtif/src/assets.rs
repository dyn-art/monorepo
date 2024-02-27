#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(tag = "type")]
pub enum Asset {
    Font(Content),
    Image(Content),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(tag = "type")]
pub enum Content {
    /// Content stored as binary data.
    Binary { content: Vec<u8> },
    /// Content referenced by a URL.
    Url { url: String },
}
