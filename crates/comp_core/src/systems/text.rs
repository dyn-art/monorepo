use bevy_ecs::{
    entity::Entity,
    query::{Changed, Or, Without},
    system::{Commands, Query, ResMut},
};
use dyn_attributed_string::{
    outline::tiny_skia_path_builder::TinySkiaPathBuilder, AttributedString, AttributedStringConfig,
    TextSizingMode,
};
use dyn_comp_asset::resources::AssetsRes;
use dyn_comp_bundles::components::{
    mixins::{AttributedStringMixin, PathMixin, SizeMixin, WindingRule},
    nodes::TextCompNode,
};
use dyn_utils::units::abs::Abs;

pub fn compute_text_from_scratch(
    mut commands: Commands,
    mut assets_res: ResMut<AssetsRes>,
    mut query: Query<
        (Entity, &TextCompNode, &mut SizeMixin),
        (
            Or<(Changed<TextCompNode>, Changed<SizeMixin>)>,
            Without<AttributedStringMixin>,
        ),
    >,
) {
    for (entity, text, mut size_mixin) in query.iter_mut() {
        let mut attributed_string = AttributedString::new(
            text.text.clone(),
            text.attributes
                .iter()
                .map(|attrs| attrs.to_attrs_interval())
                .collect(),
            AttributedStringConfig {
                line_wrap: text.line_wrap,
                horizontal_text_alignment: text.horizontal_text_alignment,
                vertical_text_alignment: text.vertical_text_alignment,
            },
        );

        attributed_string.tokenize_text(assets_res.get_fonts_book_mut());
        attributed_string.layout(&size_mixin.0, &text.sizing_mode);
        let maybe_path =
            TinySkiaPathBuilder::outline(&attributed_string, assets_res.get_fonts_book_mut());

        if let Some(path) = maybe_path {
            // Update bounds
            if text.sizing_mode == TextSizingMode::WidthAndHeight {
                size_mixin.0.width = Abs::pt(path.bounds().width())
            }
            if text.sizing_mode == TextSizingMode::WidthAndHeight
                || text.sizing_mode == TextSizingMode::Height
            {
                size_mixin.0.height = Abs::pt(path.bounds().height())
            }

            // Insert or update the PathMixin component for the entity
            commands.entity(entity).insert(PathMixin {
                path,
                winding_rule: WindingRule::Nonzero,
            });
        }

        commands
            .entity(entity)
            .insert(AttributedStringMixin(attributed_string));
    }
}

pub fn compute_text_on_size_change(
    mut commands: Commands,
    mut assets_res: ResMut<AssetsRes>,
    mut query: Query<
        (
            Entity,
            &TextCompNode,
            &mut AttributedStringMixin,
            &mut SizeMixin,
        ),
        Changed<SizeMixin>,
    >,
) {
    for (entity, text, mut attributed_string_mixin, mut size_mixin) in query.iter_mut() {
        let attributed_string = &mut attributed_string_mixin.0;

        attributed_string.layout_lines(&size_mixin.0, &text.sizing_mode);
        let maybe_path =
            TinySkiaPathBuilder::outline(&attributed_string, assets_res.get_fonts_book_mut());

        if let Some(path) = maybe_path {
            // Update bounds
            if text.sizing_mode == TextSizingMode::WidthAndHeight {
                size_mixin.0.width = Abs::pt(path.bounds().width())
            }
            if text.sizing_mode == TextSizingMode::WidthAndHeight
                || text.sizing_mode == TextSizingMode::Height
            {
                size_mixin.0.height = Abs::pt(path.bounds().height())
            }
        }
    }
}

pub fn compute_text_on_node_change(
    mut commands: Commands,
    mut assets_res: ResMut<AssetsRes>,
    mut query: Query<
        (
            Entity,
            &TextCompNode,
            &mut AttributedStringMixin,
            &mut SizeMixin,
        ),
        Changed<TextCompNode>,
    >,
) {
    for (entity, text, mut attributed_string_mixin, mut size_mixin) in query.iter_mut() {
        let mut new_attributed_string = AttributedString::new(
            text.text.clone(),
            text.attributes
                .iter()
                .map(|attrs| attrs.to_attrs_interval())
                .collect(),
            AttributedStringConfig {
                line_wrap: text.line_wrap,
                horizontal_text_alignment: text.horizontal_text_alignment,
                vertical_text_alignment: text.vertical_text_alignment,
            },
        );
        new_attributed_string.tokenize_text(assets_res.get_fonts_book_mut());
        new_attributed_string.layout(&size_mixin.0, &text.sizing_mode);

        let maybe_path =
            TinySkiaPathBuilder::outline(&new_attributed_string, assets_res.get_fonts_book_mut());

        if let Some(path) = maybe_path {
            // Update bounds
            if text.sizing_mode == TextSizingMode::WidthAndHeight {
                size_mixin.0.width = Abs::pt(path.bounds().width());
            }
            if text.sizing_mode == TextSizingMode::WidthAndHeight
                || text.sizing_mode == TextSizingMode::Height
            {
                size_mixin.0.height = Abs::pt(path.bounds().height());
            }
        }

        attributed_string_mixin.0 = new_attributed_string;
    }
}
