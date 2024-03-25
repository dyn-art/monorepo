use bevy_ecs::{
    entity::Entity,
    query::{Changed, Or},
    system::{Commands, Query, ResMut},
};
use dyn_attributed_string::{AttributedString, AttributedStringConfig, LineWrap};
use dyn_comp_asset::resources::AssetsRes;
use dyn_comp_bundles::components::{
    mixins::{PathMixin, SizeMixin},
    nodes::TextCompNode,
};

pub fn outline_text(
    mut commands: Commands,
    mut assets_res: ResMut<AssetsRes>,
    query: Query<
        (Entity, &TextCompNode, &SizeMixin),
        Or<(Changed<TextCompNode>, Changed<SizeMixin>)>,
    >,
) {
    for (entity, text, SizeMixin(size)) in query.iter() {
        // TODO: Reuse attributed string or something
        let mut attributed_string = AttributedString::new(
            text.text.clone(),
            text.attributes
                .iter()
                .map(|attrs| attrs.to_attrs_interval())
                .collect(),
            AttributedStringConfig {
                size: *size,
                // line_wrap: text.line_wrap,
                line_wrap: LineWrap::None, // TODO
            },
        );

        attributed_string.tokenize_text(assets_res.get_fonts_book_mut());
        attributed_string.layout();
        let maybe_path = attributed_string.to_path(assets_res.get_fonts_book_mut());

        // Insert or update the PathMixin component for the entity
        if let Some(path) = maybe_path {
            commands.entity(entity).insert(PathMixin(path));
        }
    }
}
