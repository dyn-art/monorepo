use bevy_ecs::system::Resource;
use owned_ttf_parser::AsFaceRef;
use std::hash::Hash;
use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::Hasher,
};

use self::font::FontMetadata;

pub mod font;

#[derive(Resource, Default)]
pub struct FontCacheRes {
    fonts: HashMap<u64, CachedFont>,
}

impl FontCacheRes {
    pub fn insert(&mut self, font_metadata: FontMetadata, content: Vec<u8>) {
        self.insert_with_hash(None, font_metadata, content)
    }

    pub fn insert_with_hash(
        &mut self,
        hash: Option<u64>,
        font_metadata: FontMetadata,
        content: Vec<u8>,
    ) {
        let hash = hash.or_else(|| Some(FontCacheRes::calculate_hash(&font_metadata)));
        self.fonts.insert(
            hash.unwrap(),
            CachedFont {
                content,
                metadata: font_metadata,
                face: None,
            },
        );
    }

    pub fn get(&self, hash: &u64) -> Option<&CachedFont> {
        self.fonts.get(hash)
    }

    pub fn get_mut(&mut self, hash: &u64) -> Option<&mut CachedFont> {
        self.fonts.get_mut(hash)
    }

    pub fn is_cached(&self, hash: &u64) -> bool {
        self.fonts.contains_key(hash)
    }

    fn calculate_hash<T: Hash>(value: &T) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        return hasher.finish();
    }
}

#[derive(Default)]
pub struct CachedFont {
    pub content: Vec<u8>,
    // https://github.com/RazrFalcon/ttf-parser/issues/37
    pub face: Option<owned_ttf_parser::OwnedFace>,
    pub metadata: FontMetadata,
}

impl CachedFont {
    // TODO: Figure out whether cloning or reconstructing the ttf_face is more performant
    pub fn get_or_create_face(&mut self) -> Option<rustybuzz::Face> {
        if self.face.is_none() {
            self.face = owned_ttf_parser::OwnedFace::from_vec(self.content.clone(), 0).ok();
        }
        return self.face.as_ref().and_then(|owned_face| {
            let face_ref = owned_face.as_face_ref();
            Some(rustybuzz::Face::from_face(face_ref.clone()))
        });
    }

    pub fn create_face(&self) -> Option<rustybuzz::Face> {
        rustybuzz::Face::from_slice(&self.content, 0)
    }
}
