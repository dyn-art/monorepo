// This code is closely derived from:
// https://github.com/RazrFalcon/resvg/blob/master/crates/usvg/src/text_to_paths.rs

pub mod byte_index;
pub mod database;
pub mod geom;
pub mod glyph;
pub mod outlined_cluster;
pub mod path_builder;
pub mod resolved_font;
pub mod text;

use self::{
    glyph::Glyph,
    outlined_cluster::OutlinedCluster,
    resolved_font::ResolvedFont,
    text::{Font, FontFamily, FontStretch, FontStyle},
};
use crate::usvg::{byte_index::ByteIndex, database::DatabaseExt};
use rustybuzz::ttf_parser::GlyphId;
use std::sync::Arc;
use tiny_skia_path::Transform;

pub fn resolve_font(font: &Font, fontdb: &fontdb::Database) -> Option<ResolvedFont> {
    let mut name_list = Vec::new();
    for family in &font.families {
        name_list.push(match family {
            FontFamily::Serif => fontdb::Family::Serif,
            FontFamily::SansSerif => fontdb::Family::SansSerif,
            FontFamily::Cursive => fontdb::Family::Cursive,
            FontFamily::Fantasy => fontdb::Family::Fantasy,
            FontFamily::Monospace => fontdb::Family::Monospace,
            FontFamily::Named(s) => fontdb::Family::Name(s),
        });
    }

    // Use the default font as fallback.
    name_list.push(fontdb::Family::Serif);

    let stretch = match font.stretch {
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

    let style = match font.style {
        FontStyle::Normal => fontdb::Style::Normal,
        FontStyle::Italic => fontdb::Style::Italic,
        FontStyle::Oblique => fontdb::Style::Oblique,
    };

    let query = fontdb::Query {
        families: &name_list,
        weight: fontdb::Weight(font.weight),
        stretch,
        style,
    };

    let id = fontdb.query(&query);
    if id.is_none() {
        log::warn!(
            "No match for '{}' font-family.",
            font.families
                .iter()
                .map(|f| f.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    fontdb.load_font(id?)
}

/// Text shaping with font fallback.
pub fn shape_text(
    text: &str,
    font: Arc<ResolvedFont>,
    small_caps: bool,
    apply_kerning: bool,
    fontdb: &fontdb::Database,
) -> Vec<Glyph> {
    let mut glyphs = shape_text_with_font(text, font.clone(), small_caps, apply_kerning, fontdb)
        .unwrap_or_default();

    // Remember all fonts used for shaping.
    let mut used_fonts = vec![font.id];

    // Loop until all glyphs become resolved or until no more fonts are left.
    'outer: loop {
        let mut missing = None;
        for glyph in &glyphs {
            if glyph.is_missing() {
                missing = Some(glyph.byte_idx.char_from(text));
                break;
            }
        }

        if let Some(c) = missing {
            let fallback_font = match find_font_for_char(c, &used_fonts, fontdb) {
                Some(v) => Arc::new(v),
                None => break 'outer,
            };

            // Shape again, using a new font.
            let fallback_glyphs = shape_text_with_font(
                text,
                fallback_font.clone(),
                small_caps,
                apply_kerning,
                fontdb,
            )
            .unwrap_or_default();

            let all_matched = fallback_glyphs.iter().all(|g| !g.is_missing());
            if all_matched {
                // Replace all glyphs when all of them were matched.
                glyphs = fallback_glyphs;
                break 'outer;
            }

            // We assume, that shaping with an any font will produce the same amount of glyphs.
            // This is incorrect, but good enough for now.
            if glyphs.len() != fallback_glyphs.len() {
                break 'outer;
            }

            // TODO: Replace clusters and not glyphs. This should be more accurate.

            // Copy new glyphs.
            for i in 0..glyphs.len() {
                if glyphs[i].is_missing() && !fallback_glyphs[i].is_missing() {
                    glyphs[i] = fallback_glyphs[i].clone();
                }
            }

            // Remember this font.
            used_fonts.push(fallback_font.id);
        } else {
            break 'outer;
        }
    }

    // Warn about missing glyphs.
    for glyph in &glyphs {
        if glyph.is_missing() {
            let c = glyph.byte_idx.char_from(text);
            // TODO: print a full grapheme
            log::warn!(
                "No fonts with a {}/U+{:X} character were found.",
                c,
                c as u32
            );
        }
    }

    glyphs
}

/// Converts a text into a list of glyph IDs.
///
/// This function will do the BIDI reordering and text shaping.
pub fn shape_text_with_font(
    text: &str,
    font: Arc<ResolvedFont>,
    small_caps: bool,
    apply_kerning: bool,
    fontdb: &fontdb::Database,
) -> Option<Vec<Glyph>> {
    fontdb.with_face_data(font.id, |font_data, face_index| -> Option<Vec<Glyph>> {
        let rb_font = rustybuzz::Face::from_slice(font_data, face_index)?;

        let bidi_info = unicode_bidi::BidiInfo::new(text, Some(unicode_bidi::Level::ltr()));
        let paragraph = &bidi_info.paragraphs[0];
        let line = paragraph.range.clone();

        let mut glyphs = Vec::new();

        let (levels, runs) = bidi_info.visual_runs(paragraph, line);
        for run in runs.iter() {
            let sub_text = &text[run.clone()];
            if sub_text.is_empty() {
                continue;
            }

            let hb_direction = if levels[run.start].is_rtl() {
                rustybuzz::Direction::RightToLeft
            } else {
                rustybuzz::Direction::LeftToRight
            };

            let mut buffer = rustybuzz::UnicodeBuffer::new();
            buffer.push_str(sub_text);
            buffer.set_direction(hb_direction);

            let mut features = Vec::new();
            if small_caps {
                features.push(rustybuzz::Feature::new(
                    rustybuzz::Tag::from_bytes(b"smcp"),
                    1,
                    ..,
                ));
            }

            if !apply_kerning {
                features.push(rustybuzz::Feature::new(
                    rustybuzz::Tag::from_bytes(b"kern"),
                    0,
                    ..,
                ));
            }

            let output = rustybuzz::shape(&rb_font, &features, buffer);

            let positions = output.glyph_positions();
            let infos = output.glyph_infos();

            for (pos, info) in positions.iter().zip(infos) {
                let idx = run.start + info.cluster as usize;
                debug_assert!(text.get(idx..).is_some());

                glyphs.push(Glyph {
                    byte_idx: ByteIndex::new(idx),
                    id: GlyphId(info.glyph_id as u16),
                    dx: pos.x_offset,
                    dy: pos.y_offset,
                    width: pos.x_advance,
                    font: font.clone(),
                });
            }
        }

        Some(glyphs)
    })?
}

/// Outlines a glyph cluster.
///
/// Uses one or more `Glyph`s to construct an `OutlinedCluster`.
pub fn outline_cluster(
    glyphs: &[Glyph],
    text: &str,
    font_size: f32,
    db: &fontdb::Database,
) -> OutlinedCluster {
    debug_assert!(!glyphs.is_empty());

    let mut builder = tiny_skia_path::PathBuilder::new();
    let mut width = 0.0;
    let mut x: f32 = 0.0;

    for glyph in glyphs {
        let sx = glyph.font.scale(font_size);

        if let Some(outline) = db.outline(glyph.font.id, glyph.id) {
            // By default, glyphs are upside-down, so we have to mirror them.
            let mut ts = Transform::from_scale(1.0, -1.0);

            // Scale to font-size.
            ts = ts.pre_scale(sx, sx);

            // Apply offset.
            //
            // The first glyph in the cluster will have an offset from 0x0,
            // but the later one will have an offset from the "current position".
            // So we have to keep an advance.
            // TODO: should be done only inside a single text span
            ts = ts.pre_translate(x + glyph.dx as f32, glyph.dy as f32);

            if let Some(outline) = outline.transform(ts) {
                builder.push_path(&outline);
            }
        }

        x += glyph.width as f32;

        let glyph_width = glyph.width as f32 * sx;
        if glyph_width > width {
            width = glyph_width;
        }
    }

    let byte_idx = glyphs[0].byte_idx;
    let font = glyphs[0].font.clone();
    OutlinedCluster {
        byte_idx,
        codepoint: byte_idx.char_from(text),
        width,
        advance: width,
        ascent: font.ascent(font_size),
        descent: font.descent(font_size),
        x_height: font.x_height(font_size),
        has_relative_shift: false,
        path: builder.finish(),
        transform: Transform::default(),
        visible: true,
    }
}

/// Finds a font with a specified char.
///
/// This is a rudimentary font fallback algorithm.
pub fn find_font_for_char(
    c: char,
    exclude_fonts: &[fontdb::ID],
    fontdb: &fontdb::Database,
) -> Option<ResolvedFont> {
    let base_font_id = exclude_fonts[0];

    // Iterate over fonts and check if any of them support the specified char.
    for face in fontdb.faces() {
        // Ignore fonts, that were used for shaping already.
        if exclude_fonts.contains(&face.id) {
            continue;
        }

        // Check that the new face has the same style.
        let base_face = fontdb.face(base_font_id)?;
        if base_face.style != face.style
            && base_face.weight != face.weight
            && base_face.stretch != face.stretch
        {
            continue;
        }

        if !fontdb.has_char(face.id, c) {
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
        return fontdb.load_font(face.id);
    }

    None
}
