use crate::font::{Font, FontId};
use std::sync::Arc;

pub trait DatabaseExt {
    fn load_font(&self, id: FontId) -> Option<Font>;
}

impl DatabaseExt for fontdb::Database {
    fn load_font(&self, id: FontId) -> Option<Font> {
        let face_info = self.face(id)?;

        let data: Arc<dyn AsRef<[u8]> + Sync + Send> = match &face_info.source {
            fontdb::Source::File(path) => {
                let data = std::fs::read(path).ok()?;
                Arc::new(data)
            }
            fontdb::Source::Binary(data) => Arc::clone(data),
            fontdb::Source::SharedFile(_path, data) => Arc::clone(data),
        };

        return Font::new(id, data, face_info.index);
    }
}
