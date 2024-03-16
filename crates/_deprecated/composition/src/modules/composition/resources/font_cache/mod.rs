use bevy_ecs::system::Resource;
use owned_ttf_parser::AsFaceRef;
use std::collections::HashMap;

use self::font::FontMetadata;

pub mod font;

#[derive(Resource, Default)]
pub struct FontCacheRes {
    fonts: HashMap<u64, CachedFont>,
}

impl FontCacheRes {
    pub fn insert(&mut self, id: u64, font_metadata: FontMetadata, content: Vec<u8>) {
        self.fonts.insert(
            id,
            CachedFont {
                data: CachedFontData::Content(content),
                metadata: font_metadata,
            },
        );
    }

    pub fn get(&self, hash: &u64) -> Option<&CachedFont> {
        self.fonts.get(hash)
    }

    pub fn get_mut(&mut self, hash: &u64) -> Option<&mut CachedFont> {
        self.fonts.get_mut(hash)
    }

    pub fn create_buzz_face(&self, hash: &u64) -> Option<rustybuzz::Face> {
        if let Some(cached_font) = self.fonts.get(hash) {
            return cached_font.create_buzz_face();
        }
        return None;
    }

    pub fn load_ttfp_face(&mut self, hash: &u64) {
        if let Some(cached_font) = self.fonts.get_mut(hash) {
            return cached_font.load_ttfp_face();
        }
    }
}

#[derive(Default)]
pub struct CachedFont {
    pub data: CachedFontData,
    pub metadata: FontMetadata,
}

pub enum CachedFontData {
    Content(Vec<u8>),
    // https://github.com/RazrFalcon/ttf-parser/issues/37
    Face(owned_ttf_parser::OwnedFace),
}

impl Default for CachedFontData {
    fn default() -> Self {
        Self::Content(Vec::new())
    }
}

impl CachedFont {
    pub fn load_ttfp_face(&mut self) {
        if let CachedFontData::Content(ref content) = self.data {
            if let Ok(owned_face) = owned_ttf_parser::OwnedFace::from_vec(content.clone(), 0) {
                self.data = CachedFontData::Face(owned_face);
            }
        }
    }

    pub fn create_buzz_face(&self) -> Option<rustybuzz::Face> {
        return match &self.data {
            CachedFontData::Face(owned_face) => {
                Some(rustybuzz::Face::from_face(owned_face.as_face_ref().clone()))
            }
            CachedFontData::Content(content) => rustybuzz::Face::from_slice(&content, 0),
        };
    }
}
