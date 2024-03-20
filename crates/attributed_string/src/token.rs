use crate::{
    attribute::Attribute,
    font::resolve_font_from_cache,
    usvg::{
        byte_index::ByteIndex,
        database::FontsCache,
        glyph::{Glyph, GlyphClusters},
        outline_cluster,
        outlined_cluster::OutlinedCluster,
        shape_text,
    },
};
use rust_lapper::{Interval, Lapper};
use std::ops::Range;

/// Represents a segment of text extracted during parsing.
///
/// Each `Token` holds a portion of text, its position within the original text,
/// and its categorized type based on the parsing logic.
#[derive(Clone)]
pub struct Token {
    /// The category of this token, defining its role and significance during parsing.
    pub variant: TokenVariant,
    /// Byte range in the original text marking the token's start and end indices.
    /// Enables attribute identification and position tracking.
    /// Inclusive start, exclusive of stop (start <= x < end).
    pub range: Range<usize>,
    ///
    pub outlined_clusters: Vec<OutlinedCluster>,
}

impl Token {
    pub fn new(variant: TokenVariant, range: Range<usize>) -> Self {
        Self {
            variant,
            range,
            outlined_clusters: Vec::new(),
        }
    }

    pub fn get_advance(&self) -> f32 {
        self.outlined_clusters
            .iter()
            .fold(0.0, |acc, oc| acc + oc.advance)
    }

    pub fn get_max_height(&self) -> f32 {
        self.outlined_clusters
            .iter()
            .fold(0.0, |acc, oc| acc.max(oc.height()))
    }

    // TODO: Does it make more sense to shape the glyphs from the attribute intervals
    // instead of from the tokens (thus always having to query the corresponding attributes)
    // now that we have guranteed there are not overlapping attributes (due to `divide_overlaps_with`).
    pub fn shape_glyphs(
        &mut self,
        text: &String,
        attribute_intervals: &Lapper<usize, Attribute>,
        fonts_cache: &mut FontsCache,
        fontdb: &fontdb::Database,
    ) {
        let mut glyphs: Vec<Option<Glyph>> = vec![None; self.range.end - self.range.start];
        let mut interval_start_byte_idx = ByteIndex::new(0);

        // Outline token and thus create glyphs based on attributes
        for Interval { start, stop, val } in
            attribute_intervals.find(self.range.start, self.range.end)
        {
            let resolved_font = match resolve_font_from_cache(&val.font, fonts_cache, fontdb) {
                Some(v) => v.clone(),
                None => continue,
            };

            let text_range = self.range.start.max(*start)..self.range.end.min(*stop);
            let interval_glyphs = shape_text(
                &text[text_range.clone()],
                resolved_font,
                val.small_caps,
                val.apply_kerning,
                fontdb,
            );

            // Add interval_glyphs to glyphs vector at start to stop index
            let interval_glyphs_len = interval_glyphs.len();
            for (index, mut glyph) in interval_glyphs.into_iter().enumerate() {
                let glyphs_index = text_range.start - self.range.start + index;

                // Update byte index (index of char in token text)
                glyph.byte_idx =
                    ByteIndex::new(glyph.byte_idx.value() + interval_start_byte_idx.value());
                if index == interval_glyphs_len - 1 {
                    interval_start_byte_idx = ByteIndex::new(glyph.byte_idx.value() + 1);
                }

                glyphs[glyphs_index] = Some(glyph);
            }
        }

        // Validate glyphs
        let maybe_glyphs_len = glyphs.len();
        let glyphs: Vec<Glyph> = glyphs.into_iter().filter_map(|glyph| glyph).collect();
        if glyphs.is_empty() || glyphs.len() != maybe_glyphs_len {
            return;
        }

        // Convert glyphs to outlined glyph clusters
        for (range, byte_idx) in GlyphClusters::new(&glyphs) {
            let interval_index = self.range.start + byte_idx.value();
            let maybe_interval = attribute_intervals
                .find(interval_index, interval_index + 1)
                .last();
            if let Some(interval) = maybe_interval {
                self.outlined_clusters.push(outline_cluster(
                    &glyphs[range],
                    &text[self.range.clone()],
                    interval.val.font_size.0,
                    fontdb,
                ));
            }
        }
    }

    pub fn outline(&self) -> Option<tiny_skia_path::Path> {
        let mut path_builder = tiny_skia_path::PathBuilder::new();

        for outlined_cluster in &self.outlined_clusters {
            if !outlined_cluster.visible {
                continue;
            }

            if let Some(path) = outlined_cluster
                .path
                .clone()
                .and_then(|path| path.transform(outlined_cluster.transform))
            {
                path_builder.push_path(&path);
            }
        }

        return path_builder.finish();
    }
}

/// Categorizes types of tokens encountered during text parsing.
///
/// This enum allows for distinguishing between various types of text elements, such as words, separators, or line breaks, facilitating their appropriate handling.
#[derive(Clone)]
pub enum TokenVariant {
    /// A separator that indicates boundaries between words (e.g., spaces, punctuation).
    WordSeparator,
    /// A line break in the text, aiding in text structure recognition.
    Linebreak,
    /// A continuous fragment of text, typically a word or number.
    TextFragment,
}
