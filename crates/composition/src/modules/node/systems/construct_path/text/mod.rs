use bevy_ecs::{
    entity::Entity,
    query::{Changed, Or},
    system::{Commands, NonSendMut, Query},
};

use crate::modules::{
    composition::resources::font::FontRes,
    node::components::{
        mixins::{DimensionMixin, SkiaPathsMixin},
        types::TextNode,
    },
};

use self::text_layout::TokenStream;

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
        let mut token_chunk = TokenStream::from_text_node(text, &mut font_res.context);
        let paths = token_chunk.to_paths(&mut font_res.context);

        commands.entity(entity).insert(SkiaPathsMixin { paths });
    }
}
