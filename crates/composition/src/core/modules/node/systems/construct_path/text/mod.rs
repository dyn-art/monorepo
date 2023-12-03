use bevy_ecs::{
    entity::Entity,
    query::{Changed, Or},
    system::{Commands, Query, ResMut},
};
use rustybuzz::UnicodeBuffer;

use crate::core::modules::{
    composition::resources::font_cache::FontCacheRes,
    node::components::{
        mixins::{Anchor, DimensionMixin, PathMixin},
        types::{Text, TextSection},
    },
};

use self::text_builder::TextBuilder;

mod text_builder;

pub fn construct_text_path(
    mut commands: Commands,
    mut font_cache: ResMut<FontCacheRes>,
    query: Query<(Entity, &Text, &DimensionMixin), Or<(Changed<Text>, Changed<DimensionMixin>)>>,
) {
    for (entity, text, dimension) in query.iter() {
        let mut path = PathMixin {
            vertices: Vec::new(),
        };
        let mut text_builder = TextBuilder::initial();

        // Process text sections
        for section in &text.sections {
            let vertices = process_section(&mut font_cache, &mut text_builder, section, dimension);
            path.vertices.extend(vertices);
        }

        commands.entity(entity).insert(path);
    }
}

fn process_section(
    font_cache: &mut FontCacheRes,
    text_builder: &mut TextBuilder,
    section: &TextSection,
    dimension: &DimensionMixin,
) -> Vec<Anchor> {
    if let Some(cached_font) = font_cache.get_mut(&section.style.font_hash) {
        if let Some(font_face) = cached_font.get_or_create_face() {
            let font_size = section.style.font_size;
            let mut unicode_buffer = UnicodeBuffer::new();
            text_builder.update_for_new_section(&font_face, font_size);

            // Process each line of the section, maintaining continuity
            let lines: Vec<&str> = section.value.split('\n').collect();
            let total_lines = lines.len();
            for (index, line) in lines.iter().enumerate() {
                unicode_buffer = text_builder.process_line(
                    unicode_buffer,
                    line,
                    dimension.width as f32,
                    font_size as f32,
                    &font_face,
                );

                // Move to a new line only if it's not the last line in the section
                if index < total_lines - 1 {
                    text_builder.move_to_new_line(font_size as f32);
                }
            }

            return text_builder.into_vertices();
        }
    }

    return Vec::new();
}
