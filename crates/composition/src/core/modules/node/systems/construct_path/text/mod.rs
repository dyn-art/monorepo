use bevy_ecs::{
    entity::Entity,
    query::{Changed, Or},
    system::{Commands, Query, ResMut},
};
use rustybuzz::{GlyphBuffer, UnicodeBuffer};

use crate::core::modules::{
    composition::resources::font_cache::FontCacheRes,
    node::components::{
        mixins::{DimensionMixin, PathMixin},
        types::Text,
    },
};

use self::text_builder::TextBuilder;

mod text_builder;

/// Constructs a path representation of text for rendering.
/// Processes each text section, line, and word, converting them into graphical path data.
pub fn construct_text_path(
    mut commands: Commands,
    mut font_cache: ResMut<FontCacheRes>,
    query: Query<(Entity, &Text, &DimensionMixin), Or<(Changed<Text>, Changed<DimensionMixin>)>>,
) {
    for (entity, text, dimension) in query.iter() {
        let mut path = PathMixin {
            vertices: Vec::new(),
        };

        for section in &text.sections {
            if let Some(cached_font) = font_cache.get_mut(&section.style.font_hash) {
                if let Some(font_face) = cached_font.get_or_create_face() {
                    let font_size = section.style.font_size;
                    let scale = (font_face.units_per_em() as f32).recip() * font_size as f32;
                    let mut unicode_buffer = UnicodeBuffer::new();
                    let mut text_builder = TextBuilder::new(
                        scale,
                        font_face.ascender(),
                        font_face.height(),
                        font_size,
                    );

                    // Process each line of the section
                    for line in section.value.split('\n') {
                        unicode_buffer = process_line(
                            &mut text_builder,
                            line,
                            dimension.width as f32,
                            font_size as f32,
                            &font_face,
                            unicode_buffer,
                        );
                    }

                    // Merge and append vertices
                    path.vertices.extend(text_builder.into_vertices());
                }
            }
        }

        commands.entity(entity).insert(path);
    }
}

/// Processes a line of text, breaking it into words and constructing their paths.
fn process_line(
    text_builder: &mut TextBuilder,
    line: &str,
    line_width: f32,
    line_height: f32,
    font_face: &rustybuzz::Face,
    mut unicode_buffer: UnicodeBuffer,
) -> UnicodeBuffer {
    let words = line.split(' ');
    let word_count = words.clone().count();

    for (index, word) in words.enumerate() {
        unicode_buffer.push_str(word);
        if index != word_count - 1 {
            unicode_buffer.push_str(" ");
        }

        let glyph_buffer = rustybuzz::shape(&font_face, &[], unicode_buffer);
        if wrap_word(
            line_width,
            &glyph_buffer,
            text_builder.scale,
            text_builder.pos.x,
        ) {
            text_builder.move_to_new_line(line_height);
        }

        text_builder.process_glyphs(&glyph_buffer, line_width, line_height, font_face);
        unicode_buffer = glyph_buffer.clear();
    }

    text_builder.move_to_new_line(line_height);

    return unicode_buffer;
}

/// Decides if a word should be wrapped to the next line based on the available width.
fn wrap_word(line_width: f32, glyph_buffer: &GlyphBuffer, scale: f32, x_pos: f32) -> bool {
    let word_length: i32 = glyph_buffer
        .glyph_positions()
        .iter()
        .map(|pos| pos.x_advance)
        .sum();
    let scaled_word_length = word_length as f32 * scale;

    scaled_word_length + x_pos > line_width
}
