use bevy_ecs::{
    entity::Entity,
    query::{Changed, Or, Without},
    system::{Commands, Query, ResMut},
};
use dyn_attributed_string::{
    outline::tiny_skia_path_builder::TinySkiaPathBuilder, AttributedString, AttributedStringConfig,
};
use dyn_comp_asset::resources::AssetsRes;
use dyn_comp_bundles::components::{
    mixins::{AttributedStringMixin, PathMixin, SizeMixin, WindingRule},
    nodes::TextCompNode,
};

pub fn outline_text_from_scratch(
    mut commands: Commands,
    mut assets_res: ResMut<AssetsRes>,
    query: Query<
        (Entity, &TextCompNode, &SizeMixin),
        (
            Or<(Changed<TextCompNode>, Changed<SizeMixin>)>,
            Without<AttributedStringMixin>,
        ),
    >,
) {
    for (entity, text, SizeMixin(size)) in query.iter() {
        let mut attributed_string = AttributedString::new(
            text.text.clone(),
            text.attributes
                .iter()
                .map(|attrs| attrs.to_attrs_interval())
                .collect(),
            AttributedStringConfig {
                size: *size,
                line_wrap: text.line_wrap,
                horizontal_text_alignment: text.horizontal_text_alignment,
                vertical_text_alignment: text.vertical_text_alignment,
            },
        );

        attributed_string.tokenize_text(assets_res.get_fonts_book_mut());
        attributed_string.layout();
        let maybe_path =
            TinySkiaPathBuilder::outline(&attributed_string, assets_res.get_fonts_book_mut());

        // Insert or update the PathMixin and AttributedStringMixin component for the entity
        let mut entity_commands = commands.entity(entity);
        if let Some(path) = maybe_path {
            entity_commands.insert(PathMixin {
                path,
                winding_rule: WindingRule::Nonzero,
            });
        }
        entity_commands.insert(AttributedStringMixin(attributed_string));
    }
}

pub fn outline_text_on_size_change(
    mut commands: Commands,
    mut assets_res: ResMut<AssetsRes>,
    mut query: Query<(Entity, &mut AttributedStringMixin, &SizeMixin), Changed<SizeMixin>>,
) {
    for (entity, mut attributed_string_mixin, SizeMixin(size)) in query.iter_mut() {
        let attributed_string = &mut attributed_string_mixin.0;

        attributed_string.apply_size(*size);
        let maybe_path =
            TinySkiaPathBuilder::outline(&attributed_string, assets_res.get_fonts_book_mut());

        // Insert or update the PathMixin component for the entity
        if let Some(path) = maybe_path {
            commands.entity(entity).insert(PathMixin {
                path,
                winding_rule: WindingRule::Nonzero,
            });
        }
    }
}

pub fn outline_text_on_node_change(
    mut commands: Commands,
    mut assets_res: ResMut<AssetsRes>,
    mut query: Query<(Entity, &mut AttributedStringMixin, &TextCompNode), Changed<TextCompNode>>,
) {
    for (entity, mut attributed_string_mixin, text) in query.iter_mut() {
        let attributed_string = &mut attributed_string_mixin.0;

        let size = attributed_string.get_size();
        let mut new_attributed_string = AttributedString::new(
            text.text.clone(),
            text.attributes
                .iter()
                .map(|attrs| attrs.to_attrs_interval())
                .collect(),
            AttributedStringConfig {
                size: *size,
                line_wrap: text.line_wrap,
                horizontal_text_alignment: text.horizontal_text_alignment,
                vertical_text_alignment: text.vertical_text_alignment,
            },
        );
        new_attributed_string.tokenize_text(assets_res.get_fonts_book_mut());
        new_attributed_string.layout();

        let maybe_path =
            TinySkiaPathBuilder::outline(&new_attributed_string, assets_res.get_fonts_book_mut());

        // Insert or update the PathMixin component for the entity
        if let Some(path) = maybe_path {
            commands.entity(entity).insert(PathMixin {
                path,
                winding_rule: WindingRule::Nonzero,
            });
        }
        attributed_string_mixin.0 = new_attributed_string;
    }
}
