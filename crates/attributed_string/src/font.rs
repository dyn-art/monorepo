use crate::{
    fonts_cache::{FontsCache, OwnedFace},
    glyph::Glyph,
    path_builder::PathBuilder,
};
use glam::Vec2;
use rustybuzz::ttf_parser;
use std::{collections::HashSet, ops::Range, sync::Arc};

pub struct Font {
    pub(crate) id: fontdb::ID,
    pub(crate) rustybuzz: OwnedFace,
    pub(crate) data: Arc<dyn AsRef<[u8]> + Send + Sync>,
}

impl std::fmt::Debug for Font {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Font")
            .field("id", &self.id)
            .finish_non_exhaustive()
    }
}

impl Font {
    #[inline]
    pub fn id(&self) -> fontdb::ID {
        self.id
    }

    #[inline]
    pub fn data(&self) -> &[u8] {
        (*self.data).as_ref()
    }

    #[inline]
    pub fn rustybuzz(&self) -> &rustybuzz::Face<'_> {
        self.rustybuzz.borrow_dependent()
    }

    #[inline]
    pub fn font_scale(&self) -> f32 {
        self.rustybuzz().units_per_em() as f32
    }

    #[inline]
    pub fn scale(&self, font_size: f32) -> f32 {
        font_size / self.font_scale()
    }

    #[inline]
    pub fn ascent(&self, font_size: f32) -> f32 {
        self.rustybuzz().ascender() as f32 * self.scale(font_size)
    }

    #[inline]
    pub fn descent(&self, font_size: f32) -> f32 {
        self.rustybuzz().descender() as f32 * self.scale(font_size)
    }

    #[inline]
    pub fn height(&self, font_size: f32) -> f32 {
        self.ascent(font_size) - self.descent(font_size)
    }

    pub fn outline(&self, glyph_id: ttf_parser::GlyphId) -> Option<tiny_skia_path::Path> {
        let mut builder = PathBuilder {
            builder: tiny_skia_path::PathBuilder::new(),
        };
        self.rustybuzz().outline_glyph(glyph_id, &mut builder)?;
        builder.builder.finish()
    }

    pub fn shape_text_with_fallback(
        &self,
        text: &str,
        range: Range<usize>,
        buffer: rustybuzz::UnicodeBuffer,
        fonts_cache: &mut FontsCache,
    ) -> (Vec<Glyph>, rustybuzz::UnicodeBuffer) {
        let mut current_buffer = buffer;

        // Shape text
        let (mut glyphs, missing, buffer) =
            self.shape_text(text, range.clone(), current_buffer, fonts_cache);
        current_buffer = buffer;

        // Remember all fonts used for shaping
        let mut used_fonts = vec![self.id];

        let mut resolved_set: HashSet<usize> = HashSet::new();
        let text_bytes = text.as_bytes();
        for &index in &missing {
            let fallback_font =
                match fonts_cache.get_font_for_char(text_bytes[index] as char, &used_fonts) {
                    Some(font) => font,
                    None => continue,
                };

            // Shape text again, using a new font
            let (mut fallback_glyphs, fallback_missing, buffer) =
                fallback_font.shape_text(text, range.clone(), current_buffer, fonts_cache);
            current_buffer = buffer;

            // Identify resolved glyphs
            let resolved: Vec<_> = missing
                .iter()
                .filter(|item| !fallback_missing.contains(item))
                .collect();

            // Apply resolved glyphs
            for &i in resolved {
                if resolved_set.contains(&i) {
                    glyphs[i] = fallback_glyphs.swap_remove(i);
                    resolved_set.insert(i);
                }
            }

            // Chech whether all glyphs have been resolved already
            if missing.len() == resolved_set.len() {
                break;
            }

            // Remember this font
            used_fonts.push(fallback_font.id);
        }

        return (glyphs, current_buffer);
    }

    pub fn shape_text(
        &self,
        text: &str,
        range: Range<usize>,
        mut buffer: rustybuzz::UnicodeBuffer,
        fonts_cache: &mut FontsCache,
    ) -> (Vec<Glyph>, Vec<usize>, rustybuzz::UnicodeBuffer) {
        let mut glyphs = Vec::new();
        let mut missing = Vec::new();
        let run = &text[range.clone()];

        // Prepare buffer for this run
        buffer.push_str(run);
        buffer.guess_segment_properties();

        let rtl = matches!(buffer.direction(), rustybuzz::Direction::RightToLeft);
        let ascent = self.rustybuzz().ascender() as f32 / self.font_scale();
        let descent = -self.rustybuzz().descender() as f32 / self.font_scale();

        let shape_plan = fonts_cache.get_shape_plan(self, &buffer);
        let glyph_buffer = rustybuzz::shape_with_plan(self.rustybuzz(), shape_plan, buffer);
        let glyph_infos = glyph_buffer.glyph_infos();
        let glyph_positions = glyph_buffer.glyph_positions();

        glyphs.reserve(glyph_infos.len());
        for (info, pos) in glyph_infos.iter().zip(glyph_positions.iter()) {
            let advance = Vec2::new(pos.x_advance as f32, pos.y_advance as f32) / self.font_scale();
            let offset = Vec2::new(pos.x_offset as f32, pos.y_offset as f32) / self.font_scale();

            let start_glyph = range.start + info.cluster as usize;

            if info.glyph_id == 0 {
                missing.push(start_glyph);
            }

            glyphs.push(Glyph {
                range: Range {
                    start: start_glyph,
                    end: range.end, // Set later
                },
                advance,
                offset,
                ascent,
                descent,
                font_id: self.id(),
                glyph_id: ttf_parser::GlyphId(
                    info.glyph_id.try_into().expect("Failed to cast glyph id!"),
                ),
            });
        }

        // Adjust end of glyphs
        if rtl {
            for i in 1..glyphs.len() {
                let next_start = glyphs[i - 1].range.start;
                let next_end = glyphs[i - 1].range.end;
                let prev = &mut glyphs[i];
                if prev.range.start == next_start {
                    prev.range.end = next_end;
                } else {
                    prev.range.end = next_start;
                }
            }
        } else {
            for i in (1..glyphs.len()).rev() {
                let next_start = glyphs[i].range.start;
                let next_end = glyphs[i].range.end;
                let prev = &mut glyphs[i - 1];
                if prev.range.start == next_start {
                    prev.range.end = next_end;
                } else {
                    prev.range.end = next_start;
                }
            }
        }

        return (glyphs, missing, glyph_buffer.clear());
    }
}
