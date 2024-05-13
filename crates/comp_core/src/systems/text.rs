use bevy_ecs::{
    entity::Entity,
    query::{Changed, Or, With, Without},
    system::{Commands, Query, ResMut},
};
use dyn_attributed_string::{
    layout::layout_config::{LayoutConfig, LayoutSize, TextSizingMode},
    AttributedString,
};
use dyn_comp_asset::resources::AssetsRes;
use dyn_comp_bundles::components::{
    mixins::{AttributedStringMixin, SizeMixin},
    nodes::TextCompNode,
};
use dyn_utils::units::{abs::Abs, auto_length::AutoLength};

// TODO:
// 1. Compute text bounding box without outlining (for text computation)
// 2. Figure out how to best apply sizing mode
//    - I guess best would be to create custom Size (with Auto and Absolute)
// 3. If Auto sizing mode figure out how to update elements position e.g. if center or right aligned

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
        );

        attributed_string.tokenize_text(assets_res.get_fonts_book_mut());
        let layout_handler = attributed_string.layout(LayoutConfig {
            size: match text.sizing_mode {
                TextSizingMode::Fixed => LayoutSize::new(
                    AutoLength::abs(size_mixin.0.width),
                    AutoLength::abs(size_mixin.0.height),
                ),
                TextSizingMode::WidthAndHeight => {
                    LayoutSize::new(AutoLength::Auto, AutoLength::Auto)
                }
                TextSizingMode::Height => {
                    LayoutSize::new(AutoLength::abs(size_mixin.0.width), AutoLength::Auto)
                }
            },
            line_wrap: text.line_wrap,
            horizontal_text_alignment: text.horizontal_text_alignment,
            vertical_text_alignment: text.vertical_text_alignment,
        });

        let text_size = layout_handler.compute_text_size(attributed_string.get_spans());

        // Update bounds
        if text.sizing_mode == TextSizingMode::WidthAndHeight {
            size_mixin.0.width = Abs::pt(text_size.width())
        }
        if text.sizing_mode == TextSizingMode::WidthAndHeight
            || text.sizing_mode == TextSizingMode::Height
        {
            size_mixin.0.height = Abs::pt(text_size.height())
        }

        commands
            .entity(entity)
            .insert(AttributedStringMixin(attributed_string));
    }
}

pub fn compute_text_on_size_change(
    mut query: Query<
        (&TextCompNode, &mut AttributedStringMixin, &SizeMixin),
        (With<AttributedStringMixin>, Changed<SizeMixin>),
    >,
) {
    for (text, mut attributed_string_mixin, size_mixin) in query.iter_mut() {
        attributed_string_mixin.0.layout(LayoutConfig {
            size: match text.sizing_mode {
                TextSizingMode::Fixed => LayoutSize::new(
                    AutoLength::abs(size_mixin.0.width),
                    AutoLength::abs(size_mixin.0.height),
                ),
                TextSizingMode::WidthAndHeight => {
                    LayoutSize::new(AutoLength::Auto, AutoLength::Auto)
                }
                TextSizingMode::Height => {
                    LayoutSize::new(AutoLength::abs(size_mixin.0.width), AutoLength::Auto)
                }
            },
            line_wrap: text.line_wrap,
            horizontal_text_alignment: text.horizontal_text_alignment,
            vertical_text_alignment: text.vertical_text_alignment,
        });
    }
}

pub fn compute_text_on_node_change(
    mut assets_res: ResMut<AssetsRes>,
    mut query: Query<
        (&TextCompNode, &mut AttributedStringMixin, &mut SizeMixin),
        (With<AttributedStringMixin>, Changed<TextCompNode>),
    >,
) {
    for (text, mut attributed_string_mixin, mut size_mixin) in query.iter_mut() {
        let mut new_attributed_string = AttributedString::new(
            text.text.clone(),
            text.attributes
                .iter()
                .map(|attrs| attrs.to_attrs_interval())
                .collect(),
        );
        new_attributed_string.tokenize_text(assets_res.get_fonts_book_mut());
        let layout_handler = attributed_string_mixin.0.layout(LayoutConfig {
            size: match text.sizing_mode {
                TextSizingMode::Fixed => LayoutSize::new(
                    AutoLength::abs(size_mixin.0.width),
                    AutoLength::abs(size_mixin.0.height),
                ),
                TextSizingMode::WidthAndHeight => {
                    LayoutSize::new(AutoLength::Auto, AutoLength::Auto)
                }
                TextSizingMode::Height => {
                    LayoutSize::new(AutoLength::abs(size_mixin.0.width), AutoLength::Auto)
                }
            },
            line_wrap: text.line_wrap,
            horizontal_text_alignment: text.horizontal_text_alignment,
            vertical_text_alignment: text.vertical_text_alignment,
        });

        let text_size = layout_handler.compute_text_size(attributed_string_mixin.0.get_spans());

        // Update bounds
        if text.sizing_mode == TextSizingMode::WidthAndHeight {
            size_mixin.0.width = Abs::pt(text_size.width());
        }
        if text.sizing_mode == TextSizingMode::WidthAndHeight
            || text.sizing_mode == TextSizingMode::Height
        {
            size_mixin.0.height = Abs::pt(text_size.height());
        }

        attributed_string_mixin.0 = new_attributed_string;
    }
}
