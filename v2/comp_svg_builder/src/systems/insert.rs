use crate::{
    resources::svg_context::SvgContextRes,
    svg::svg_bundle::{
        node::{frame::FrameNodeSvgBundle, shape::ShapeNodeSvgBundle},
        style::solid::SolidStyleSvgBundle,
        SvgBundleVariant,
    },
};
use bevy_ecs::{
    entity::Entity,
    query::{Changed, Or, With, Without},
    system::{Commands, Query, ResMut},
};
use dyn_comp_common::{
    mixins::PaintChildMixin,
    nodes::{CompNode, CompNodeVariant},
    paints::{CompPaint, CompPaintVariant},
    styles::{CompStyle, CompStyleVariant},
};

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
    paint_query: Query<&CompPaint>,
) {
    for (entity, style, maybe_paint_mixin) in query.iter() {
        if let Some(paint_entity) = maybe_paint_mixin.and_then(|paint_mixin| paint_mixin.0) {
            if let Ok(paint) = paint_query.get(paint_entity) {
                let bundle_variant = match (style.variant, paint.variant) {
                    (
                        CompStyleVariant::Fill | CompStyleVariant::Stroke,
                        CompPaintVariant::Solid,
                    ) => Some(SvgBundleVariant::Solid(SolidStyleSvgBundle::new(
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
    }
}
