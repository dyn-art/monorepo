use crate::{
    resources::svg_context::SvgContextRes,
    svg::{
        svg_bundle::{
            frame_node::FrameNodeSvgBundle, shape_node::ShapeNodeSvgBundle,
            solid_fill::SolidFillSvgBundle, FillSvgBundle, NodeSvgBundle, NodeSvgBundleMixin,
            SvgBundle,
        },
        svg_element::SvgElementId,
    },
};
use bevy_ecs::{
    entity::Entity,
    query::{Changed, Or, With, Without},
    system::{Commands, Query, ResMut},
};
use dyn_comp_types::{
    mixins::FillMixin,
    nodes::{
        CompNode, EllipseCompNode, FrameCompNode, PolygonCompNode, RectangleCompNode, StarCompNode,
        TextCompNode,
    },
};
use std::collections::HashSet;

pub fn insert_frame_node_svg_bundle(
    mut commands: Commands,
    mut svg_context_res: ResMut<SvgContextRes>,
    query: Query<
        Entity,
        (
            With<CompNode>,
            With<FrameCompNode>,
            Without<NodeSvgBundleMixin>,
        ),
    >,
) {
    query.iter().for_each(|entity| {
        commands
            .entity(entity)
            .insert(NodeSvgBundleMixin(NodeSvgBundle::Frame(
                FrameNodeSvgBundle::new(entity, &mut svg_context_res),
            )));
    });
}

pub fn insert_shape_node_svg_bundle(
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
            Without<NodeSvgBundleMixin>,
        ),
    >,
) {
    query.iter().for_each(|entity| {
        commands
            .entity(entity)
            .insert(NodeSvgBundleMixin(NodeSvgBundle::Shape(
                ShapeNodeSvgBundle::new(entity, &mut svg_context_res),
            )));
    });
}

pub fn insert_fills(
    mut svg_context_res: ResMut<SvgContextRes>,
    mut query: Query<(&FillMixin, &mut NodeSvgBundleMixin), (With<CompNode>, Changed<FillMixin>)>,
) {
    for (FillMixin(fills), mut bundle_mixin) in query.iter_mut() {
        let NodeSvgBundleMixin(bundle) = bundle_mixin.as_mut();
        let bundle_fills = match bundle {
            NodeSvgBundle::Frame(bundle) => &bundle.fills,
            NodeSvgBundle::Shape(bundle) => &bundle.fills,
            _ => return,
        };

        // Identify removed and newly added fills
        let current_fill_entities_set = bundle_fills
            .iter()
            .map(|fill| *fill.get_paint_entity())
            .collect::<HashSet<_>>();
        let new_fill_entities_set = fills.iter().map(|fill| fill.paint).collect::<HashSet<_>>();
        let removed_fill_entities = current_fill_entities_set
            .difference(&new_fill_entities_set)
            .cloned()
            .collect::<Vec<_>>();
        let added_fill_entities = new_fill_entities_set
            .difference(&current_fill_entities_set)
            .cloned()
            .collect::<Vec<_>>();

        // Process removed entities
        for entity in removed_fill_entities {
            let mut to_remove_ids: Vec<SvgElementId> = Vec::new();
            let bundle_fills = match bundle {
                NodeSvgBundle::Frame(bundle) => &mut bundle.fills,
                NodeSvgBundle::Shape(bundle) => &mut bundle.fills,
                _ => return,
            };
            bundle_fills.retain(|fill| {
                if *fill.get_paint_entity() == entity {
                    to_remove_ids.push(fill.get_svg_bundle().get_root_element().get_id());
                    false
                } else {
                    true
                }
            });

            let fill_wrapper_element = match bundle {
                NodeSvgBundle::Frame(bundle) => &mut bundle.fill_wrapper_g,
                NodeSvgBundle::Shape(bundle) => &mut bundle.fill_wrapper_g,
            };
            fill_wrapper_element.remove_children(&to_remove_ids);
        }

        // Process added entities
        for entity in added_fill_entities {
            let mut fill_bundle = SolidFillSvgBundle::new(entity, &mut svg_context_res);
            match bundle {
                NodeSvgBundle::Frame(bundle) => {
                    bundle
                        .fill_wrapper_g
                        .append_child_in_world_context(entity, fill_bundle.get_root_element_mut());
                    bundle.fills.push(FillSvgBundle::Solid(fill_bundle));
                }
                NodeSvgBundle::Shape(bundle) => {
                    bundle
                        .fill_wrapper_g
                        .append_child_in_world_context(entity, fill_bundle.get_root_element_mut());
                    bundle.fills.push(FillSvgBundle::Solid(fill_bundle));
                }
            };
        }

        // TODO: Reorder
    }
}
