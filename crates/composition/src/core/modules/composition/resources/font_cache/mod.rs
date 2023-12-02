use bevy_ecs::system::Resource;
use rustybuzz::Face;
use std::hash::Hash;
use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::Hasher,
};

use self::font::Font;

pub mod font;

#[derive(Resource, Default)]
pub struct FontCacheRes<'a> {
    font_content: HashMap<u64, CachedFont<'a>>,
}

impl<'a> FontCacheRes<'a> {
    pub fn insert(&mut self, font: Font, content: Vec<u8>) {
        self.insert_with_hash(None, font, content)
    }

    pub fn insert_with_hash(&mut self, hash: Option<u64>, font: Font, content: Vec<u8>) {
        let hash = hash.or_else(|| Some(FontCacheRes::calculate_hash(&font)));
        self.font_content.insert(
            hash.unwrap(),
            CachedFont {
                content,
                font,
                face: None,
            },
        );
    }

    pub fn get(&'a mut self, hash: &u64) -> Option<&'a CachedFont<'a>> {
        self.font_content.get(hash)
    }

    pub fn is_cached(&self, hash: &u64) -> bool {
        self.font_content.contains_key(hash)
    }

    fn calculate_hash<T: Hash>(value: &T) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        return hasher.finish();
    }
}

#[derive(Default)]
pub struct CachedFont<'a> {
    content: Vec<u8>,
    face: Option<Face<'a>>,
    font: Font,
}

impl<'a> CachedFont<'a> {
    pub fn get_or_create_face(&'a mut self) -> &'a Face<'a> {
        if self.face.is_none() {
            self.face = Face::from_slice(&self.content, 0);
        }
        self.face.as_ref().unwrap()
    }
}
