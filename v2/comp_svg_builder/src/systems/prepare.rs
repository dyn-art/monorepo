use crate::{
    resources::svg_context::SvgContextRes,
    svg::svg_bundle::{
        node::{frame::FrameNodeSvgBundle, shape::ShapeNodeSvgBundle},
        style::{gradient::GradientStyleSvgBundle, solid::SolidStyleSvgBundle},
        SvgBundleVariant,
    },
};
use bevy_ecs::{
    entity::Entity,
    query::{Changed, Or, With, Without},
    system::{Commands, Query, ResMut},
};
use dyn_comp_common::{
    common::Size,
    mixins::{PaintChildMixin, SizeMixin, StyleChildrenMixin, StyleParentMixin},
    nodes::{CompNode, CompNodeVariant},
    paints::{CompPaint, CompPaintVariant, GradientCompPaint, ImageCompPaint},
    styles::{CompStyle, CompStyleVariant},
};
use glam::Vec2;

pub fn insert_node_svg_bundle(
    mut commands: Commands,
    mut svg_context_res: ResMut<SvgContextRes>,
    query: Query<(Entity, &CompNode), (With<CompNode>, Without<SvgBundleVariant>)>,
) {
    for (entity, CompNode { variant }) in query.iter() {
        let bundle_variant = match variant {
            CompNodeVariant::Frame => Some(SvgBundleVariant::Frame(FrameNodeSvgBundle::new(
                entity,
                &mut svg_context_res,
            ))),
            CompNodeVariant::Rectangle
            | CompNodeVariant::Ellipse
            | CompNodeVariant::Polygon
            | CompNodeVariant::Star
            | CompNodeVariant::Text
            | CompNodeVariant::Vector => Some(SvgBundleVariant::Shape(ShapeNodeSvgBundle::new(
                entity,
                &mut svg_context_res,
            ))),
            _ => None,
        };

        if let Some(bundle_variant) = bundle_variant {
            commands.entity(entity).insert(bundle_variant);
        }
    }
}

pub fn insert_style_svg_bundle(
    mut commands: Commands,
    mut svg_context_res: ResMut<SvgContextRes>,
    query: Query<
        (Entity, &CompStyle, Option<&PaintChildMixin>),
        (
            With<CompStyle>,
            Or<(Without<SvgBundleVariant>, Changed<PaintChildMixin>)>,
        ),
    >,
    paint_query: Query<(
        &CompPaint,
        Option<&GradientCompPaint>,
        Option<&ImageCompPaint>,
    )>,
) {
    for (entity, style, maybe_paint_mixin) in query.iter() {
        if let Some(paint_entity) = maybe_paint_mixin.and_then(|paint_mixin| paint_mixin.0) {
            if let Ok((paint, maybe_gradient_paint, maybe_image_paint)) =
                paint_query.get(paint_entity)
            {
                let bundle_variant = match (style.variant, paint.variant) {
                    (
                        CompStyleVariant::Fill | CompStyleVariant::Stroke,
                        CompPaintVariant::Solid,
                    ) => Some(SvgBundleVariant::Solid(SolidStyleSvgBundle::new(
                        entity,
                        &mut svg_context_res,
                    ))),
                    (
                        CompStyleVariant::Fill | CompStyleVariant::Stroke,
                        CompPaintVariant::Gradient,
                    ) => Some(SvgBundleVariant::Gradient(GradientStyleSvgBundle::new(
                        entity,
                        maybe_gradient_paint.unwrap().variant,
                        &mut svg_context_res,
                    ))),
                    _ => None,
                };

                if let Some(bundle_variant) = bundle_variant {
                    commands.entity(entity).insert(bundle_variant);
                }
            }
        }
    }
}

// Note: To avoid Bevy's ECS conflict between mutable and immutable references of the same component
// (`SizeMixin` in this case), we explicitly specify `Without` in the queries.
// This is necessary because Bevy ensures safe access to components, and having both mutable and
// immutable references to the same component type in different queries can lead to runtime errors.
// In our system, `With<CompNode>` and `With<CompStyle>` could potentially conflict, as they might coexist on the same entity.
// Adding `Without<CompStyle>` and `Without<CompNode>` to the respective queries resolves this conflict by ensuring
// that entities in one query cannot be present in the other, thereby upholding Rust's borrowing rules.
// https://discord.com/channels/691052431525675048/1199265475155202108
// https://github.com/bevyengine/bevy/blob/main/errors/B0002.md
pub fn sync_node_size_with_style(
    mut commands: Commands,
    node_query: Query<
        (Entity, &SizeMixin, &StyleChildrenMixin),
        (With<CompNode>, Without<CompStyle>, Changed<SizeMixin>),
    >,
    mut style_with_size_query: Query<
        (Entity, &StyleParentMixin, &mut SizeMixin),
        (With<CompStyle>, Without<CompNode>, With<SizeMixin>),
    >,
    style_without_size_query: Query<
        (Entity, &StyleParentMixin),
        (With<CompStyle>, Without<CompNode>, Without<SizeMixin>),
    >,
) {
    for (node_entity, SizeMixin(Size(size)), StyleChildrenMixin(children)) in node_query.iter() {
        // Update existing DimensionMixin for children with Paint and DimensionMixin
        for (paint_entity, StyleParentMixin(parent), mut size_mixin) in
            style_with_size_query.iter_mut()
        {
            if children.contains(&paint_entity) && *parent == node_entity {
                size_mixin.0 .0.x = size.x;
                size_mixin.0 .0.y = size.y;
            }
        }

        // Add DimensionMixin for children with Paint but without DimensionMixin
        for (paint_entity, StyleParentMixin(parent)) in style_without_size_query.iter() {
            if children.contains(&paint_entity) && *parent == node_entity {
                commands
                    .entity(paint_entity)
                    .insert(SizeMixin(Size(Vec2::new(size.x, size.y))));
            }
        }
    }
}
