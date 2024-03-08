use crate::{
    resources::svg_context::SvgContextRes,
    svg::svg_bundle::{
        node::{
            frame::FrameNodeSvgBundle, shape::ShapeNodeSvgBundle, NodeSvgBundle, NodeSvgBundleMixin,
        },
        style::{solid::SolidStyleSvgBundle, StyleSvgBundle, StyleSvgBundleMixin},
    },
};
use bevy_ecs::{
    entity::Entity,
    query::{Changed, Or, With, Without},
    system::{Commands, Query, ResMut},
};
use dyn_comp_common::{
    mixins::PaintMixin,
    nodes::{CompNode, CompNodeVariant},
    paints::{CompPaint, CompPaintVariant},
    styles::{CompStyle, CompStyleVariant},
};

pub fn insert_node_svg_bundle(
    mut commands: Commands,
    mut svg_context_res: ResMut<SvgContextRes>,
    query: Query<(Entity, &CompNode), (With<CompNode>, Without<NodeSvgBundleMixin>)>,
) {
    for (entity, CompNode { variant }) in query.iter() {
        let node_bundle = match variant {
            CompNodeVariant::Frame => {
                NodeSvgBundle::Frame(FrameNodeSvgBundle::new(entity, &mut svg_context_res))
            }
            CompNodeVariant::Rectangle
            | CompNodeVariant::Ellipse
            | CompNodeVariant::Polygon
            | CompNodeVariant::Star
            | CompNodeVariant::Text
            | CompNodeVariant::Vector => {
                NodeSvgBundle::Shape(ShapeNodeSvgBundle::new(entity, &mut svg_context_res))
            }
            // TODO
            _ => return,
        };
        commands
            .entity(entity)
            .insert(NodeSvgBundleMixin(node_bundle));
    }
}

pub fn insert_style_svg_bundle(
    mut commands: Commands,
    mut svg_context_res: ResMut<SvgContextRes>,
    query: Query<
        (Entity, &CompStyle, Option<&PaintMixin>),
        (
            With<CompStyle>,
            Or<(Without<StyleSvgBundleMixin>, Changed<PaintMixin>)>,
        ),
    >,
    paint_query: Query<&CompPaint>,
) {
    for (entity, CompStyle { variant }, maybe_paint_mixin) in query.iter() {
        let node_bundle = match variant {
            CompStyleVariant::Fill | CompStyleVariant::Stroke => {
                if let Some(PaintMixin(maybe_paint_entity)) = maybe_paint_mixin {
                    if let Some(paint_entity) = maybe_paint_entity {
                        if let Ok(CompPaint { variant }) = paint_query.get(*paint_entity) {
                            match variant {
                                CompPaintVariant::Solid => StyleSvgBundle::Solid(
                                    SolidStyleSvgBundle::new(entity, &mut svg_context_res),
                                ),
                                // TODO
                                _ => return,
                            }
                        } else {
                            return;
                        }
                    } else {
                        return;
                    }
                } else {
                    return;
                }
            }
        };
        commands
            .entity(entity)
            .insert(StyleSvgBundleMixin(node_bundle));
    }
}
