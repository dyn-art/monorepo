use crate::{
    resources::svg_context::SvgContextRes,
    svg::svg_node::{frame::FrameSvgNode, shape::ShapeSvgNode, SvgNodeVariant},
};
use bevy_ecs::{
    entity::Entity,
    query::{Or, With, Without},
    system::{Commands, Query, ResMut},
};
use dyn_comp_types::nodes::{
    CompNode, EllipseCompNode, FrameCompNode, PolygonCompNode, RectangleCompNode, StarCompNode,
    TextCompNode,
};

pub fn insert_frame_svg_node(
    mut commands: Commands,
    mut svg_context_res: ResMut<SvgContextRes>,
    query: Query<Entity, (With<CompNode>, With<FrameCompNode>, Without<SvgNodeVariant>)>,
) {
    query.iter().for_each(|entity| {
        commands
            .entity(entity)
            .insert(SvgNodeVariant::Frame(FrameSvgNode::new(
                entity,
                &mut svg_context_res,
            )));
    });
}

pub fn insert_shape_svg_node(
    mut commands: Commands,
    mut svg_context_res: ResMut<SvgContextRes>,
    query: Query<
        Entity,
        (
            With<CompNode>,
            Or<(
                With<RectangleCompNode>,
                With<TextCompNode>,
                With<PolygonCompNode>,
                With<EllipseCompNode>,
                With<StarCompNode>,
            )>,
            Without<SvgNodeVariant>,
        ),
    >,
) {
    query.iter().for_each(|entity| {
        commands
            .entity(entity)
            .insert(SvgNodeVariant::Shape(ShapeSvgNode::new(
                entity,
                &mut svg_context_res,
            )));
    });
}
