pub mod database;
pub mod font;

use crate::database::DatabaseExt;
use font::{
    info::{FontFamily, FontInfo},
    variant::{FontStretch, FontStyle, FontVariant},
    Font, FontId,
};
use std::collections::{hash_map::Entry, HashMap};

pub struct FontsBook {
    /// The underlying font database.
    db: fontdb::Database,
    /// Cache for loaded fonts from the database.
    fonts_cache: HashMap<FontId, Option<Font>>,
    /// Cache for font infos.
    font_info_cache: HashMap<FontInfo, FontId>,
    /// Cache for rustybuzz shape plans.
    font_shape_plan_cache: HashMap<ShapePlanKey, rustybuzz::ShapePlan>,
}

impl FontsBook {
    pub fn new() -> Self {
        Self {
            db: fontdb::Database::new(),
            fonts_cache: HashMap::new(),
            font_info_cache: HashMap::new(),
            font_shape_plan_cache: HashMap::new(),
        }
    }

    pub fn get_db(&self) -> &fontdb::Database {
        &self.db
    }

    pub fn get_db_mut(&mut self) -> &mut fontdb::Database {
        &mut self.db
    }

    pub fn load_system_fonts(&mut self) {
        self.db.load_system_fonts();
    }

    pub fn get_font_by_id(&mut self, id: FontId) -> Option<Font> {
        self.fonts_cache
            .entry(id)
            .or_insert_with(|| match self.db.load_font(id) {
                Some(font) => Some(font),
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

    pub fn get_font_by_id_no_cache(&self, id: FontId) -> Option<Font> {
        self.fonts_cache
            .get(&id)
            .cloned()
            .unwrap_or_else(|| match self.db.load_font(id) {
                Some(font) => Some(font),
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

    pub fn get_font_by_info(&mut self, info: FontInfo) -> Option<Font> {
        let id = match self.font_info_cache.entry(info) {
            Entry::Occupied(occ) => *occ.get(),
            Entry::Vacant(vac) => {
                let FontInfo {
                    family,
                    variant:
                        FontVariant {
                            stretch,
                            style,
                            weight,
                        },
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

                let stretch = match *stretch {
                    FontStretch::ULTRA_CONDENSED => fontdb::Stretch::UltraCondensed,
                    FontStretch::EXTRA_CONDENSED => fontdb::Stretch::ExtraCondensed,
                    FontStretch::CONDENSED => fontdb::Stretch::Condensed,
                    FontStretch::SEMI_CONDENSED => fontdb::Stretch::SemiCondensed,
                    FontStretch::NORMAL => fontdb::Stretch::Normal,
                    FontStretch::SEMI_EXPANDED => fontdb::Stretch::SemiExpanded,
                    FontStretch::EXPANDED => fontdb::Stretch::Expanded,
                    FontStretch::EXTRA_EXPANDED => fontdb::Stretch::ExtraExpanded,
                    FontStretch::ULTRA_EXPANDED => fontdb::Stretch::UltraExpanded,
                    _ => fontdb::Stretch::Normal,
                };

                let style = match style {
                    FontStyle::Normal => fontdb::Style::Normal,
                    FontStyle::Italic => fontdb::Style::Italic,
                    FontStyle::Oblique => fontdb::Style::Oblique,
                };

                let query = fontdb::Query {
                    families: &name_list,
                    weight: fontdb::Weight(weight.to_number()),
                    stretch,
                    style,
                };

                let maybe_id = self.db.query(&query);
                if maybe_id.is_none() {
                    log::warn!("Failed find font for query: {:?}", query);
                }
                maybe_id.map(|id| {
                    vac.insert(id);
                    id
                })?
            }
        };

        return self.get_font_by_id(id);
    }

    pub fn get_font_for_char(&mut self, _char: char, exclude_fonts: &[FontId]) -> Option<Font> {
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
            if !self
                .get_font_by_id_no_cache(face.id)
                .map(|font| font.has_char(_char))
                .unwrap_or(false)
            {
                continue;
            }

            let base_family = base_face
                .families
                .iter()
                .find(|(_, language)| *language == fontdb::Language::English_UnitedStates)
                .unwrap_or(&base_face.families[0]);

            let new_family = face
                .families
                .iter()
                .find(|(_, language)| *language == fontdb::Language::English_UnitedStates)
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
            font_id: font.get_id(),
            direction: buffer.direction(),
            script: buffer.script(),
            language: buffer.language(),
        };

        return match self.font_shape_plan_cache.entry(key) {
            Entry::Occupied(occ) => occ.into_mut(),
            Entry::Vacant(vac) => {
                let ShapePlanKey {
                    direction,
                    script,
                    language,
                    ..
                } = vac.key();

                let plan = rustybuzz::ShapePlan::new(
                    font.get_rustybuzz(),
                    *direction,
                    Some(*script),
                    language.as_ref(),
                    &[],
                );

                vac.insert(plan)
            }
        };
    }
}

impl Default for FontsBook {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct ShapePlanKey {
    font_id: FontId,
    direction: rustybuzz::Direction,
    script: rustybuzz::Script,
    language: Option<rustybuzz::Language>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::font::variant::FontWeight;

    #[test]
    fn e2e() {
        let mut fonts_book = FontsBook::new();
        fonts_book.load_system_fonts();
        let maybe_font = fonts_book.get_font_by_info(FontInfo {
            family: FontFamily::SansSerif,
            variant: FontVariant::new(FontStyle::Normal, FontWeight::REGULAR, FontStretch::NORMAL),
        });

        assert_eq!(maybe_font.is_some(), true);
    }
}
