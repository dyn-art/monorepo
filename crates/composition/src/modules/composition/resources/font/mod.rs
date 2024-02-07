use fontdb::{Database, Source, ID};
use rustybuzz::ttf_parser;
use std::{rc::Rc, sync::Arc};
use ttf_parser::GlyphId;
use unicode_script::UnicodeScript;

use self::usvg::{
    geom::IsValidLength,
    text::Font,
    text_to_paths::{
        outline_cluster, resolve_font, shape_text, DatabaseExt, FontsCache, Glyph, OutlinedCluster,
        ResolvedFont,
    },
};

pub mod usvg;

// No-Send Ressource (https://bevy-cheatbook.github.io/programming/non-send.html)
// because of Rc in FontsCache for now
#[derive(Default)]
pub struct FontRes {
    pub context: FontContext,
}

#[derive(Default)]
pub struct FontContext {
    fontdb: Database,
    fonts_cache: FontsCache,
}

impl FontContext {
    pub fn add_font_source(&mut self, content: Vec<u8>) {
        self.fontdb
            .load_font_source(Source::Binary(Arc::new(content)));
    }

    pub fn resolve_font(&mut self, font: &Font) -> Option<Rc<ResolvedFont>> {
        if !self.fonts_cache.contains_key(font) {
            if let Some(resolved_font) = resolve_font(font, &self.fontdb) {
                self.fonts_cache
                    .insert(font.clone(), Rc::new(resolved_font));
            }
        }

        return self.fonts_cache.get(font).cloned();
    }

    pub fn load_font(&self, id: ID) -> Option<ResolvedFont> {
        self.fontdb.load_font(id)
    }

    pub fn outline(&self, id: ID, glyph_id: GlyphId) -> Option<tiny_skia_path::Path> {
        self.fontdb.outline(id, glyph_id)
    }

    pub fn has_char(&self, id: ID, c: char) -> bool {
        self.fontdb.has_char(id, c)
    }

    /// Text shaping with font fallback.
    pub fn shape_text(
        &self,
        text: &str,
        font: Rc<ResolvedFont>,
        small_caps: bool,
        apply_kerning: bool,
    ) -> Vec<Glyph> {
        shape_text(text, font, small_caps, apply_kerning, &self.fontdb)
    }

    /// Outlines a glyph cluster.
    ///
    /// Uses one or more `Glyph`s to construct an `OutlinedCluster`.
    pub fn outline_cluster(&self, glyphs: &[Glyph], text: &str, font_size: f32) -> OutlinedCluster {
        outline_cluster(glyphs, text, font_size, &self.fontdb)
    }

    /// Applies the `letter-spacing` property to a text chunk clusters.
    ///
    /// [In the CSS spec](https://www.w3.org/TR/css-text-3/#letter-spacing-property).
    pub fn apply_letter_spacing(clusters: &mut [OutlinedCluster], letter_spacing: f32) {
        let num_clusters = clusters.len();
        for (i, cluster) in clusters.iter_mut().enumerate() {
            // Spacing must be applied only to characters that belongs to the script
            // that supports spacing.
            // We are checking only the first code point, since it should be enough.
            // https://www.w3.org/TR/css-text-3/#cursive-tracking
            let script = cluster.codepoint.script();
            if Self::script_supports_letter_spacing(script) {
                // A space after the last cluster should be ignored,
                // since it affects the bbox and text alignment.
                if i != num_clusters - 1 {
                    cluster.advance += letter_spacing;
                }

                // If the cluster advance became negative - clear it.
                // This is an UB so we can do whatever we want, and we mimic Chrome's behavior.
                if !cluster.advance.is_valid_length() {
                    cluster.width = 0.0;
                    cluster.advance = 0.0;
                    cluster.path = None;
                }
            }
        }
    }

    /// Applies the `word-spacing` property to a text chunk clusters.
    ///
    /// [In the CSS spec](https://www.w3.org/TR/css-text-3/#propdef-word-spacing).
    pub fn apply_word_spacing(clusters: &mut [OutlinedCluster], word_spacing: f32) {
        for cluster in clusters {
            if Self::is_word_separator_char(cluster.codepoint) {
                // Technically, word spacing 'should be applied half on each
                // side of the character', but it doesn't affect us in any way,
                // so we are ignoring this.
                cluster.advance += word_spacing;

                // After word spacing, `advance` can be negative.
            }
        }
    }

    /// Checks that selected script supports letter spacing.
    ///
    /// [In the CSS spec](https://www.w3.org/TR/css-text-3/#cursive-tracking).
    ///
    /// The list itself is from: https://github.com/harfbuzz/harfbuzz/issues/64
    fn script_supports_letter_spacing(script: unicode_script::Script) -> bool {
        use unicode_script::Script;

        !matches!(
            script,
            Script::Arabic
                | Script::Syriac
                | Script::Nko
                | Script::Manichaean
                | Script::Psalter_Pahlavi
                | Script::Mandaic
                | Script::Mongolian
                | Script::Phags_Pa
                | Script::Devanagari
                | Script::Bengali
                | Script::Gurmukhi
                | Script::Modi
                | Script::Sharada
                | Script::Syloti_Nagri
                | Script::Tirhuta
                | Script::Ogham
        )
    }

    /// Checks that the selected character is a word separator.
    ///
    /// According to: https://www.w3.org/TR/css-text-3/#word-separator
    pub fn is_word_separator_char(c: char) -> bool {
        matches!(
            c as u32,
            0x0020 | 0x00A0 | 0x1361 | 0x010100 | 0x010101 | 0x01039F | 0x01091F
        )
    }

    pub fn is_linebreak_char(c: char) -> bool {
        matches!(c, '\n')
    }
}
