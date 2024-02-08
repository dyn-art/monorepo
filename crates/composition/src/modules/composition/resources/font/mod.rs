use fontdb::{Database, Source, ID};
use rustybuzz::ttf_parser;
use std::{rc::Rc, sync::Arc};
use ttf_parser::GlyphId;
use unicode_script::UnicodeScript;

use self::usvg::{
    geom::IsValidLength,
    text::{BaselineShift, Font, LengthAdjust, TextFlow, WritingMode},
    text_to_paths::{
        apply_writing_mode, outline_cluster, resolve_baseline_shift, resolve_font,
        script_supports_letter_spacing, shape_text, DatabaseExt, FontsCache, Glyph,
        OutlinedCluster, ResolvedFont,
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

    pub fn resolve_font(&mut self, font: &Font) -> Option<&Rc<ResolvedFont>> {
        if !self.fonts_cache.contains_key(font) {
            if let Some(resolved_font) = resolve_font(font, &self.fontdb) {
                self.fonts_cache
                    .insert(font.clone(), Rc::new(resolved_font));
            }
        }

        return self.fonts_cache.get(font);
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

    pub fn apply_writing_mode(clusters: &mut [OutlinedCluster], writing_mode: WritingMode) {
        apply_writing_mode(writing_mode, clusters)
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
            if script_supports_letter_spacing(script) {
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

    pub fn apply_length_adjust(
        clusters: &mut [OutlinedCluster],
        length_adjust: LengthAdjust,
        text_length: Option<f32>,
        text_flow: TextFlow,
    ) {
        let is_horizontal = matches!(text_flow, TextFlow::Linear);

        let target_width = match text_length {
            Some(v) => v,
            None => return,
        };

        let mut width = 0.0;
        let mut cluster_indexes = Vec::new();
        for (index, _) in clusters.iter().enumerate() {
            cluster_indexes.push(index);
        }
        // Complex scripts can have mutli-codepoint clusters therefore we have to remove duplicates.
        cluster_indexes.sort();
        cluster_indexes.dedup();

        for i in &cluster_indexes {
            // Use the original cluster `width` and not `advance`.
            // This method essentially discards any `word-spacing` and `letter-spacing`.
            width += clusters[*i].width;
        }

        if cluster_indexes.is_empty() {
            return;
        }

        if length_adjust == LengthAdjust::Spacing {
            let factor = if cluster_indexes.len() > 1 {
                (target_width - width) / (cluster_indexes.len() - 1) as f32
            } else {
                0.0
            };

            for i in cluster_indexes {
                clusters[i].advance = clusters[i].width + factor;
            }
        } else {
            let factor = target_width / width;
            // Prevent multiplying by zero.
            if factor < 0.001 {
                return;
            }

            for i in cluster_indexes {
                clusters[i].transform = clusters[i].transform.pre_scale(factor, 1.0);

                // Technically just a hack to support the current text-on-path algorithm.
                if !is_horizontal {
                    clusters[i].advance *= factor;
                    clusters[i].width *= factor;
                }
            }
        }
    }

    pub fn resolve_baseline_shift(
        baselines: &[BaselineShift],
        font: &ResolvedFont,
        font_size: f32,
    ) -> f32 {
        resolve_baseline_shift(baselines, font, font_size)
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
