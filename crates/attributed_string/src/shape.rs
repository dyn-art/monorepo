use crate::glyph::Glyph;
use dyn_fonts_book::{font::Font, FontsBook};
use rustybuzz::ttf_parser;
use std::{collections::HashSet, ops::Range};

pub fn shape_text_with_fallback(
    text: &str,
    range: Range<usize>,
    buffer: rustybuzz::UnicodeBuffer,
    font: &Font,
    fonts_book: &mut FontsBook,
) -> (Vec<Glyph>, rustybuzz::UnicodeBuffer) {
    let mut current_buffer = buffer;

    // Shape text
    let (mut glyphs, missing_glyphs, buffer) =
        shape_text(text, range.clone(), current_buffer, font, fonts_book);
    current_buffer = buffer;

    // Remember all fonts used for shaping
    let mut used_fonts = vec![font.get_id()];

    let mut resolved_glyphs_set: HashSet<usize> = HashSet::new();
    let text_bytes = text.as_bytes();
    for &index in &missing_glyphs {
        let fallback_font =
            match fonts_book.get_font_for_char(text_bytes[index] as char, &used_fonts) {
                Some(font) => font,
                None => continue,
            };

        // Shape text again, using a new font
        let (mut fallback_glyphs, fallback_missing_glyphs, buffer) = shape_text(
            text,
            range.clone(),
            current_buffer,
            &fallback_font,
            fonts_book,
        );
        current_buffer = buffer;

        // Identify resolved glyphs
        let resolved: Vec<_> = missing_glyphs
            .iter()
            .filter(|item| !fallback_missing_glyphs.contains(item))
            .collect();

        // Apply resolved glyphs
        for &i in resolved {
            if resolved_glyphs_set.contains(&i) {
                glyphs[i] = fallback_glyphs.swap_remove(i);
                resolved_glyphs_set.insert(i);
            }
        }

        // Chech whether all glyphs have been resolved already
        if missing_glyphs.len() == resolved_glyphs_set.len() {
            break;
        }

        // Remember this font
        used_fonts.push(fallback_font.get_id());
    }

    return (glyphs, current_buffer);
}

pub fn shape_text(
    text: &str,
    range: Range<usize>,
    mut buffer: rustybuzz::UnicodeBuffer,
    font: &Font,
    fonts_book: &mut FontsBook,
) -> (Vec<Glyph>, Vec<usize>, rustybuzz::UnicodeBuffer) {
    let mut glyphs = Vec::new();
    let mut missing_glyphs = Vec::new();
    let run_text = &text[range.clone()];

    // Prepare buffer for this run
    buffer.push_str(run_text);
    buffer.guess_segment_properties();

    let is_rtl = matches!(buffer.direction(), rustybuzz::Direction::RightToLeft);

    let shape_plan = fonts_book.get_shape_plan(font, &buffer);
    let glyph_buffer = rustybuzz::shape_with_plan(font.get_rustybuzz(), shape_plan, buffer);
    let glyph_infos = glyph_buffer.glyph_infos();
    let glyph_positions = glyph_buffer.glyph_positions();

    glyphs.reserve(glyph_infos.len());
    for (info, pos) in glyph_infos.iter().zip(glyph_positions.iter()) {
        let x_advance = font.to_em(pos.x_advance as f32);
        let start_glyph = range.start + info.cluster as usize; // Byte Index

        if info.glyph_id == 0 {
            missing_glyphs.push(start_glyph);
        }

        glyphs.push(Glyph {
            font_id: font.get_id(),
            glyph_id: ttf_parser::GlyphId(
                info.glyph_id.try_into().expect("Failed to cast glyph id!"),
            ),
            codepoint: text[start_glyph..].chars().next().unwrap(),
            range: Range {
                start: start_glyph,
                end: range.end, // Set later to adjust for glyph clusters (graphemes)
            },
            width: x_advance,
            x_advance,
            y_advance: font.to_em(pos.y_advance as f32),
            x_offset: font.to_em(pos.x_offset as f32),
            y_offset: font.to_em(pos.y_offset as f32),
            ascent: font.get_metrics().ascender,
            descent: font.get_metrics().descender,
        });
    }

    adjust_glyph_ends(&mut glyphs, is_rtl);

    return (glyphs, missing_glyphs, glyph_buffer.clear());
}

/// Adjusts end of glyphs to ensure correct glyph cluster boundaries.
fn adjust_glyph_ends(glyphs: &mut [Glyph], rtl: bool) {
    // For RTL, iterate normally as the logic and visual order align more closely
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
    }
    // For LTR, iterate in reverse to correctly adjust end positions based on the visual ordering
    else {
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
}
