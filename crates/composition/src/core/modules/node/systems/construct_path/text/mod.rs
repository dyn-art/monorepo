use bevy_ecs::{
    entity::Entity,
    query::{Changed, Or},
    system::{Commands, Query, ResMut},
};

use crate::core::modules::{
    composition::resources::font_cache::FontCacheRes,
    node::components::{
        mixins::{DimensionMixin, PathMixin},
        types::Text,
    },
};

use self::text_builder::TextBuilder;

mod current_line;
mod line_break_strategy;
mod text_builder;
mod token;
mod token_stream;
mod token_with_shape;

pub fn construct_text_path(
    mut commands: Commands,
    mut font_cache: ResMut<FontCacheRes>,
    query: Query<(Entity, &Text, &DimensionMixin), Or<(Changed<Text>, Changed<DimensionMixin>)>>,
) {
    for (entity, text, dimension) in query.iter() {
        let mut path = PathMixin {
            vertices: Vec::new(),
        };
        let mut text_builder = TextBuilder::new(dimension.width as f32);

        // Process text
        text_builder.process_text(text, &mut font_cache);
        path.vertices.extend(text_builder.into_vertices());

        commands.entity(entity).insert(path);
    }
}
