use crate::{
    attrs::{Attrs, FontAttrs, FontFamily, FontStretch, FontStyle},
    font::{Font, FontId},
};
use rustybuzz::ttf_parser;
use std::{
    collections::{hash_map::Entry, HashMap},
    sync::Arc,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct ShapePlanKey {
    font_id: FontId,
    direction: rustybuzz::Direction,
    script: rustybuzz::Script,
    language: Option<rustybuzz::Language>,
}

pub struct FontsCache {
    db: fontdb::Database,
    fonts_cache: HashMap<FontId, Option<Arc<Font>>>,
    font_attrs_cache: HashMap<FontAttrs, FontId>,
    font_shape_plan_cache: HashMap<ShapePlanKey, rustybuzz::ShapePlan>,
}

impl FontsCache {
    pub fn new() -> Self {
        Self {
            db: fontdb::Database::new(),
            fonts_cache: HashMap::new(),
            font_attrs_cache: HashMap::new(),
            font_shape_plan_cache: HashMap::new(),
        }
    }

    pub fn db(&self) -> &fontdb::Database {
        &self.db
    }

    pub fn db_mut(&mut self) -> &mut fontdb::Database {
        &mut self.db
    }

    pub fn load_system_fonts(&mut self) {
        self.db.load_system_fonts();
        self.db.set_serif_family("Times New Roman");
        self.db.set_sans_serif_family("Arial");
        self.db.set_cursive_family("Comic Sans MS");
        self.db.set_fantasy_family("Impact");
        self.db.set_monospace_family("Courier New");
    }

    pub fn get_font_by_id(&mut self, id: FontId) -> Option<Arc<Font>> {
        self.fonts_cache
            .entry(id)
            .or_insert_with(|| match self.db.load_font(id) {
                Some(font) => Some(Arc::new(font)),
                None => {
                    log::warn!(
                        "Failed to load font '{}'",
                        self.db.face(id)?.post_script_name
                    );
                    None
                }
            })
            .clone()
    }

    pub fn get_font_by_attrs(&mut self, attrs: &Attrs) -> Option<Arc<Font>> {
        let mut font: Option<Arc<Font>> = None;
        if let Some(font_id) = attrs.get_font_id() {
            font = self.get_font_by_id(font_id);
        }
        if font.is_none() {
            font = self.get_font_by_font_attrs(FontAttrs::from_attrs(&attrs));
        }
        return font;
    }

    pub fn get_font_by_font_attrs(&mut self, font_attrs: FontAttrs) -> Option<Arc<Font>> {
        let id = match self.font_attrs_cache.entry(font_attrs) {
            Entry::Occupied(occ) => *occ.get(),
            Entry::Vacant(vac) => {
                let FontAttrs {
                    family,
                    style,
                    stretch,
                    weight,
                } = vac.key();

                let mut name_list = Vec::new();
                name_list.push(match &family {
                    FontFamily::Serif => fontdb::Family::Serif,
                    FontFamily::SansSerif => fontdb::Family::SansSerif,
                    FontFamily::Cursive => fontdb::Family::Cursive,
                    FontFamily::Fantasy => fontdb::Family::Fantasy,
                    FontFamily::Monospace => fontdb::Family::Monospace,
                    FontFamily::Named(s) => fontdb::Family::Name(s),
                });

                // Use the default font as fallback.
                name_list.push(fontdb::Family::Serif);

                let stretch = match stretch {
                    FontStretch::UltraCondensed => fontdb::Stretch::UltraCondensed,
                    FontStretch::ExtraCondensed => fontdb::Stretch::ExtraCondensed,
                    FontStretch::Condensed => fontdb::Stretch::Condensed,
                    FontStretch::SemiCondensed => fontdb::Stretch::SemiCondensed,
                    FontStretch::Normal => fontdb::Stretch::Normal,
                    FontStretch::SemiExpanded => fontdb::Stretch::SemiExpanded,
                    FontStretch::Expanded => fontdb::Stretch::Expanded,
                    FontStretch::ExtraExpanded => fontdb::Stretch::ExtraExpanded,
                    FontStretch::UltraExpanded => fontdb::Stretch::UltraExpanded,
                };

                let style = match style {
                    FontStyle::Normal => fontdb::Style::Normal,
                    FontStyle::Italic => fontdb::Style::Italic,
                    FontStyle::Oblique => fontdb::Style::Oblique,
                };

                let query = fontdb::Query {
                    families: &name_list,
                    weight: fontdb::Weight(*weight),
                    stretch,
                    style,
                };

                self.db.query(&query).map(|id| {
                    vac.insert(id);
                    id
                })?
            }
        };

        return self.get_font_by_id(id);
    }

    pub fn get_font_for_char(&mut self, c: char, exclude_fonts: &[FontId]) -> Option<Arc<Font>> {
        let base_font_id = exclude_fonts[0];
        let mut maybe_face_id = None;

        // Iterate over fonts and check if any of them support the specified char
        for face in self.db.faces() {
            // Ignore fonts, that were used for shaping already
            if exclude_fonts.contains(&face.id) {
                continue;
            }

            // Check that the new face has the same style
            let base_face = self.db.face(base_font_id)?;
            if base_face.style != face.style
                && base_face.weight != face.weight
                && base_face.stretch != face.stretch
            {
                continue;
            }

            // Check that the new face contains the char
            if !self.db.has_char(face.id, c) {
                continue;
            }

            let base_family = base_face
                .families
                .iter()
                .find(|f| f.1 == fontdb::Language::English_UnitedStates)
                .unwrap_or(&base_face.families[0]);

            let new_family = face
                .families
                .iter()
                .find(|f| f.1 == fontdb::Language::English_UnitedStates)
                .unwrap_or(&base_face.families[0]);

            log::warn!("Fallback from {} to {}.", base_family.0, new_family.0);

            maybe_face_id = Some(face.id);
            break;
        }

        return if let Some(id) = maybe_face_id {
            self.get_font_by_id(id)
        } else {
            None
        };
    }

    pub fn get_shape_plan(
        &mut self,
        font: &Font,
        buffer: &rustybuzz::UnicodeBuffer,
    ) -> &rustybuzz::ShapePlan {
        let key = ShapePlanKey {
            font_id: font.id(),
            direction: buffer.direction(),
            script: buffer.script(),
            language: buffer.language(),
        };
        match self.font_shape_plan_cache.entry(key) {
            Entry::Occupied(occ) => occ.into_mut(),
            Entry::Vacant(vac) => {
                let ShapePlanKey {
                    direction,
                    script,
                    language,
                    ..
                } = vac.key();

                let plan = rustybuzz::ShapePlan::new(
                    font.owned_face(),
                    *direction,
                    Some(*script),
                    language.as_ref(),
                    &[],
                );

                vac.insert(plan)
            }
        }
    }
}

trait DatabaseExt {
    fn load_font(&self, id: FontId) -> Option<Font>;
    fn has_char(&self, id: FontId, c: char) -> bool;
}

impl DatabaseExt for fontdb::Database {
    fn load_font(&self, id: FontId) -> Option<Font> {
        let info = self.face(id)?;

        let data: Arc<dyn AsRef<[u8]> + Sync + Send> = match &info.source {
            fontdb::Source::File(path) => {
                let data = std::fs::read(path).ok()?;
                Arc::new(data)
            }
            fontdb::Source::Binary(data) => Arc::clone(data),
            fontdb::Source::SharedFile(_path, data) => Arc::clone(data),
        };

        return Font::new(id, info.index, data);
    }

    fn has_char(&self, id: FontId, c: char) -> bool {
        let res = self.with_face_data(id, |font_data, face_index| -> Option<bool> {
            let font = ttf_parser::Face::parse(font_data, face_index).ok()?;
            font.glyph_index(c)?;
            Some(true)
        });

        res == Some(Some(true))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attrs::Attrs;
    use rustybuzz::UnicodeBuffer;
    use std::ops::Range;

    #[test]
    fn e2e() {
        let mut fonts_cache = FontsCache::new();
        fonts_cache.load_system_fonts();
        let maybe_font = fonts_cache.get_font_by_font_attrs(FontAttrs::from_attrs(
            &Attrs::new().font_family(FontFamily::Serif).font_weight(400),
        ));

        if let Some(font) = &maybe_font {
            let text = String::from("Hello World");
            let (glyphs, missing, _) = font.shape_text(
                &text,
                Range {
                    start: 0,
                    end: text.len(),
                },
                UnicodeBuffer::new(),
                &mut fonts_cache,
            );

            assert_eq!(glyphs.is_empty(), false);
            assert_eq!(missing.is_empty(), true);
        }

        assert_eq!(maybe_font.is_some(), true);
    }
}
