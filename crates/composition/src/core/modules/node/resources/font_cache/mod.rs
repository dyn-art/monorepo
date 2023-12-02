use bevy_ecs::system::Resource;
use rustybuzz::Face;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use self::font::Font;

pub mod font;

#[derive(Resource, Default)]
pub struct FontCacheRes<'a> {
    font_content: HashMap<String, CachedFont<'a>>,
    default_font: Option<Font>,
}

#[derive(Default)]
pub struct CachedFont<'a> {
    content: Vec<u8>,
    face: Option<Face<'a>>,
}

impl<'a> CachedFont<'a> {
    pub fn get_or_load_face(&'a mut self) -> &'a Face<'a> {
        if self.face.is_none() {
            self.face = Face::from_slice(&self.content, 0);
        }
        self.face.as_ref().unwrap()
    }
}
