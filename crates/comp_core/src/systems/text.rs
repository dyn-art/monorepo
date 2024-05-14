use bevy_ecs::{
    entity::Entity,
    query::{Changed, With},
    system::{Commands, Query, ResMut},
};
use dyn_attributed_string::{
    layout::{
        layouter::{Layouter, LayouterConfig},
        LayoutSize, TextSizingMode,
    },
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
    mut query: Query<(Entity, &TextCompNode, &mut SizeMixin), Changed<TextCompNode>>,
) {
    for (entity, text, mut size_mixin) in query.iter_mut() {
        let mut intervals = text
            .attributes
            .iter()
            .map(|attrs| attrs.to_attrs_interval())
            .collect();
        AttributedString::adjust_intervals(&mut intervals, &text.text);

        let mut attributed_string = AttributedString::new(text.text.clone(), intervals);

        attributed_string.tokenize_text(assets_res.get_fonts_book_mut());
        let mut layouter = Layouter::new(LayouterConfig {
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
        layouter.layout(&mut attributed_string);
        let container_size = layouter.get_container_size().unwrap();

        // Update bounds
        if text.sizing_mode == TextSizingMode::WidthAndHeight {
            size_mixin.0.width = Abs::pt(container_size.width())
        }
        if text.sizing_mode == TextSizingMode::WidthAndHeight
            || text.sizing_mode == TextSizingMode::Height
        {
            size_mixin.0.height = Abs::pt(container_size.height())
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
        let mut layouter = Layouter::new(LayouterConfig {
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
        layouter.layout(&mut attributed_string_mixin.0);
    }
}
