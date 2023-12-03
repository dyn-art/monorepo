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
            let verticies = process_section(&mut font_cache, section, dimension);
            path.vertices.extend(verticies);
        }

        commands.entity(entity).insert(path);
    }
}

/// Processes a section of text.
fn process_section(
    font_cache: &mut FontCacheRes,
    section: &TextSection,
    dimension: &DimensionMixin,
) -> Vec<Anchor> {
    if let Some(cached_font) = font_cache.get_mut(&section.style.font_hash) {
        if let Some(font_face) = cached_font.get_or_create_face() {
            let font_size = section.style.font_size;
            let mut text_builder = TextBuilder::new(&font_face, font_size);
            let mut unicode_buffer = UnicodeBuffer::new();

            // Process each line of the section
            for line in section.value.split('\n') {
                unicode_buffer = text_builder.process_line(
                    unicode_buffer,
                    line,
                    dimension.width as f32,
                    font_size as f32,
                    &font_face,
                );
            }

            return text_builder.into_vertices();
        }
    }

    return Vec::new();
}
