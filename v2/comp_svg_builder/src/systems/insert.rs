use crate::{
    resources::svg_context::SvgContextRes,
    svg::svg_bundle::{
        frame_node::FrameNodeSvgBundle, shape_node::ShapeNodeSvgBundle, NodeSvgBundleVariant,
    },
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

pub fn insert_frame_svg_bundle(
    mut commands: Commands,
    mut svg_context_res: ResMut<SvgContextRes>,
    query: Query<
        Entity,
        (
            With<CompNode>,
            With<FrameCompNode>,
            Without<NodeSvgBundleVariant>,
        ),
    >,
) {
    query.iter().for_each(|entity| {
        commands
            .entity(entity)
            .insert(NodeSvgBundleVariant::Frame(FrameNodeSvgBundle::new(
                entity,
                &mut svg_context_res,
            )));
    });
}

pub fn insert_shape_svg_bundle(
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
            Without<NodeSvgBundleVariant>,
        ),
    >,
) {
    query.iter().for_each(|entity| {
        commands
            .entity(entity)
            .insert(NodeSvgBundleVariant::Shape(ShapeNodeSvgBundle::new(
                entity,
                &mut svg_context_res,
            )));
    });
}
