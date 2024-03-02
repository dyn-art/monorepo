use crate::{
    resources::svg_context::SvgContextRes,
    svg::{
        svg_bundle::{
            frame_node::FrameNodeSvgBundle, shape_node::ShapeNodeSvgBundle,
            solid_fill::SolidFillSvgBundle, FillSvgBundle, NodeSvgBundle, NodeSvgBundleMixin,
            SvgBundle,
        },
        svg_element::element_changes::{SvgElementChange, SvgElementReorderedChange},
    },
};
use bevy_ecs::{
    entity::Entity,
    query::{Changed, Or, With, Without},
    system::{Commands, Query, ResMut},
};
use dyn_comp_types::{
    common::Fill,
    mixins::FillMixin,
    nodes::{
        CompNode, EllipseCompNode, FrameCompNode, PolygonCompNode, RectangleCompNode, StarCompNode,
        TextCompNode,
    },
};
use std::collections::{HashMap, HashSet};

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
        let bundle_fills = match bundle.get_fills_mut() {
            Some(bundle_fills) => bundle_fills,
            None => return,
        };

        // Identify removed and newly added fills (paint entities)
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

        process_removed_fills(removed_fill_entities, bundle);
        process_added_fills(added_fill_entities, bundle, &mut svg_context_res);
        reorder_fills(fills, bundle);
    }
}

fn process_removed_fills(removed_entities: Vec<Entity>, bundle: &mut NodeSvgBundle) -> Option<()> {
    for entity in removed_entities {
        let to_remove_ids = bundle
            .get_fills_mut()?
            .iter()
            .filter(|fill| *fill.get_paint_entity() == entity)
            .map(|fill| fill.get_svg_bundle().get_root_element().get_id())
            .collect::<Vec<_>>();

        bundle.get_fills_mut()?.retain(|fill| {
            !to_remove_ids.contains(&fill.get_svg_bundle().get_root_element().get_id())
        });

        bundle
            .get_fill_wrapper_element_mut()?
            .remove_children(&to_remove_ids);
    }

    Some(())
}

fn process_added_fills(
    added_entities: Vec<Entity>,
    bundle: &mut NodeSvgBundle,
    svg_context_res: &mut ResMut<SvgContextRes>,
) -> Option<()> {
    for entity in added_entities {
        let mut fill_bundle = SolidFillSvgBundle::new(entity, svg_context_res);
        bundle
            .get_fill_wrapper_element_mut()?
            .append_child_in_world_context(entity, fill_bundle.get_root_element_mut());
        bundle
            .get_fills_mut()?
            .push(FillSvgBundle::Solid(fill_bundle));
    }

    Some(())
}

fn reorder_fills(fills: &[Fill], bundle: &mut NodeSvgBundle) -> Option<()> {
    let order_map = fills
        .iter()
        .enumerate()
        .map(|(index, fill)| (fill.paint, index))
        .collect::<HashMap<Entity, usize>>();
    let bundle_fills = bundle.get_fills_mut()?;

    // Track the original positions of the fills
    #[cfg(feature = "output_svg_element_changes")]
    let original_positions = bundle_fills
        .iter()
        .map(|fill| fill.get_svg_bundle().get_root_element().get_id())
        .collect::<Vec<_>>();

    // Sort `bundle_fills` based on the order defined in `order_map`
    bundle_fills.sort_by_key(|fill| {
        *order_map
            .get(&fill.get_paint_entity())
            .unwrap_or(&usize::MAX)
    });

    #[cfg(feature = "output_svg_element_changes")]
    {
        // Determine the new positions after sorting
        let new_positions = bundle_fills
            .iter()
            .map(|fill| fill.get_svg_bundle().get_root_element().get_id())
            .collect::<Vec<_>>();

        // Emit SvgElementReorderedChange events for fills that have been moved
        for (new_index, &element_id) in new_positions.iter().enumerate() {
            let original_index = original_positions
                .iter()
                .position(|&e| e == element_id)
                .unwrap_or(new_index);

            // If the fill has been moved
            if original_index != new_index {
                let new_parent_id = bundle.get_fill_wrapper_element_mut()?.get_id();

                // Determine insert_before_id based on the next sibling in the new order
                let insert_before_id = if new_index + 1 < new_positions.len() {
                    // There is a next sibling, get its ID
                    Some(new_positions[new_index + 1])
                } else {
                    // No next sibling, append at the end
                    None
                };

                bundle.get_fill_wrapper_element_mut()?.register_change(
                    SvgElementChange::ElementReordered(SvgElementReorderedChange {
                        element_id,
                        new_parent_id,
                        insert_before_id,
                    }),
                );
            }
        }
    }

    Some(())
}
