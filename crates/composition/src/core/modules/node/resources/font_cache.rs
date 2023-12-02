// TODO

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Serialize, Deserialize, Debug, Type)]
pub struct Font {
    pub name: String,
    pub family: String,
    pub style: FontStyle,
    weight: u16,
    hash: String,
}

#[derive(Serialize, Deserialize, Debug, Type)]
pub struct FontWithContent {
    pub font: Font,
    pub preview_url: Option<String>,
    pub content: FontContent,
}

#[derive(Serialize, Deserialize, Debug, Type)]
pub enum FontContent {
    Url(String),
    Binary(Vec<u8>),
}

#[derive(Serialize, Deserialize, Debug, Type)]
pub enum FontStyle {
    Italic,
    Normal,
}

pub struct FontCacheRes {
    font_content: HashMap<String, Vec<u8>>,
    default_font: Option<Font>,
}
