use bevy_ecs::{
    entity::Entity,
    query::{Changed, Or},
    system::{Commands, NonSend, NonSendMut, Query},
};

use crate::modules::{
    composition::resources::font::FontRes,
    node::components::{mixins::DimensionMixin, types::TextNode},
};

use self::text_layout::TokenChunk;

mod text_layout;

// TODO: Improve based on:
//  https://github.dev/pop-os/cosmic-text/tree/main

pub fn construct_text_path(
    mut commands: Commands,
    mut font_res: NonSendMut<FontRes>,
    query: Query<
        (Entity, &TextNode, &DimensionMixin),
        Or<(Changed<TextNode>, Changed<DimensionMixin>)>,
    >,
) {
    for (entity, text, dimension) in query.iter() {
        // let mut path = PathMixin {
        //     vertices: Vec::new(),
        // };
        // let mut text_builder = TextBuilder::new(dimension.width as f32);

        let mut token_chunk = TokenChunk::from_text_node(text, &mut font_res.context);
        token_chunk.to_paths(&mut font_res.context);

        // // Process text
        // text_builder.process_text(text, &mut font_cache);
        // path.vertices.extend(text_builder.into_vertices());

        // commands.entity(entity).insert(path);
    }
}
