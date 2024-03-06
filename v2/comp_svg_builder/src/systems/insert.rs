use crate::{
    resources::svg_context::SvgContextRes,
    svg::{
        svg_bundle::{
            fill::{solid::SolidFillSvgBundle, FillSvgBundle},
            node::{
                frame::FrameNodeSvgBundle, shape::ShapeNodeSvgBundle, NodeSvgBundle,
                NodeSvgBundleMixin,
            },
        },
        svg_element::{
            element_changes::{SvgElementChange, SvgElementReorderedChange},
            styles::SvgStyle,
            SvgElementId,
        },
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
    paints::{CompPaint, CompPaintVariant},
};
use smallvec::SmallVec;
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

// TODO: Right now the system doesn't work with nodes referencing the same paint multiple times
// but thats good enough for now
pub fn insert_fills(
    mut svg_context_res: ResMut<SvgContextRes>,
    mut query: Query<(&FillMixin, &mut NodeSvgBundleMixin), (With<CompNode>, Changed<FillMixin>)>,
    paint_query: Query<&CompPaint>,
) {
    for (FillMixin(fills), mut bundle_mixin) in query.iter_mut() {
        let NodeSvgBundleMixin(bundle) = bundle_mixin.as_mut();

        remove_absent_node_fills(bundle, fills);
        add_or_update_node_fills(bundle, fills, &paint_query, &mut svg_context_res);
        reorder_node_fills(bundle, fills);
    }
}

fn remove_absent_node_fills(node_bundle: &mut NodeSvgBundle, fills: &[Fill]) {
    let mut to_remove_element_ids: SmallVec<[SvgElementId; 2]> = SmallVec::new();

    // Identify to remove element ids
    let fill_bundles = match node_bundle.get_fill_bundles_mut() {
        Some(fill_bundles) => fill_bundles,
        None => return,
    };
    let fill_entities: HashSet<Entity> = fills.iter().map(|fill| fill.paint).collect();
    fill_bundles.retain(|fill_bundle| {
        let retain = fill_entities.contains(fill_bundle.get_paint_entity());
        if !retain {
            to_remove_element_ids.push(fill_bundle.get_svg_bundle().get_root_element().get_id());
        }
        return retain;
    });

    // Remove elements from node bundle based on element ids
    let fill_wrapper_element = match node_bundle.get_fills_wrapper_element_mut() {
        Some(fill_wrapper_element) => fill_wrapper_element,
        None => return,
    };
    fill_wrapper_element.remove_children(&to_remove_element_ids);
}

fn add_or_update_node_fills(
    node_bundle: &mut NodeSvgBundle,
    fills: &[Fill],
    paint_query: &Query<&CompPaint>,
    svg_context_res: &mut ResMut<SvgContextRes>,
) {
    let mut to_add_fill_bundles: SmallVec<[FillSvgBundle; 2]> = SmallVec::new();

    // Update existing fills and identify newly added fills
    let fill_bundles = match node_bundle.get_fill_bundles_mut() {
        Some(fill_bundles) => fill_bundles,
        None => return,
    };
    for fill in fills.iter() {
        match fill_bundles
            .iter_mut()
            .find(|fill_bundle| *fill_bundle.get_paint_entity() == fill.paint)
        {
            // If found, update the existing fill bundle as necessary
            Some(fill_bundle) => {
                let root_element = fill_bundle.get_svg_bundle_mut().get_root_element_mut();
                root_element.set_styles(vec![
                    SvgStyle::BlendMode {
                        blend_mode: (&fill.blend_mode).into(),
                    },
                    SvgStyle::Opacity {
                        opacity: fill.opacity.0.get(),
                    },
                ]);
            }
            // If not found, create a new fill bundle
            None => {
                if let Ok(paint) = paint_query.get(fill.paint) {
                    let mut fill_bundle = create_fill_bundle(paint, fill, svg_context_res);
                    let root_element = fill_bundle.get_svg_bundle_mut().get_root_element_mut();
                    root_element.set_styles(vec![
                        SvgStyle::BlendMode {
                            blend_mode: (&fill.blend_mode).into(),
                        },
                        SvgStyle::Opacity {
                            opacity: fill.opacity.0.get(),
                        },
                    ]);
                    to_add_fill_bundles.push(fill_bundle);
                }
            }
        }
    }

    // Append fills to node bundle
    let fill_wrapper_element = match node_bundle.get_fills_wrapper_element_mut() {
        Some(fill_wrapper_element) => fill_wrapper_element,
        None => return,
    };
    for fill_bundle in &mut to_add_fill_bundles {
        fill_wrapper_element.append_child_in_world_context(
            *fill_bundle.get_paint_entity(),
            fill_bundle.get_svg_bundle_mut().get_root_element_mut(),
        );
    }
    let fill_bundles = match node_bundle.get_fill_bundles_mut() {
        Some(fill_bundles) => fill_bundles,
        None => return,
    };
    fill_bundles.extend(to_add_fill_bundles);
}

fn reorder_node_fills(node_bundle: &mut NodeSvgBundle, fills: &[Fill]) {
    let fill_bundles = match node_bundle.get_fill_bundles_mut() {
        Some(fill_bundles) => fill_bundles,
        None => return,
    };

    // Track the original positions of the fills
    #[cfg(feature = "output_svg_element_changes")]
    let original_positions = fill_bundles
        .iter()
        .map(|fill| fill.get_svg_bundle().get_root_element().get_id())
        .collect::<Vec<_>>();

    // Sort bundle fills
    fill_bundles.sort_by_key(|fill_bundle| {
        fills
            .iter()
            .position(|fill| *fill_bundle.get_paint_entity() == fill.paint)
            .unwrap_or(usize::MAX)
    });

    #[cfg(feature = "output_svg_element_changes")]
    {
        // Determine the new positions after sorting
        let new_positions = fill_bundles
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
                let fill_wrapper_element = match node_bundle.get_fills_wrapper_element_mut() {
                    Some(fill_wrapper_element) => fill_wrapper_element,
                    None => return,
                };
                let new_parent_id = fill_wrapper_element.get_id();

                // Determine insert_before_id based on the next sibling in the new order
                let insert_before_id = if new_index + 1 < new_positions.len() {
                    // There is a next sibling, get its ID
                    Some(new_positions[new_index + 1])
                } else {
                    // No next sibling, append at the end
                    None
                };

                fill_wrapper_element.register_change(SvgElementChange::ElementReordered(
                    SvgElementReorderedChange {
                        element_id,
                        new_parent_id,
                        insert_before_id,
                    },
                ));
            }
        }
    }
}

fn create_fill_bundle(
    paint: &CompPaint,
    fill: &Fill,
    mut svg_context_res: &mut ResMut<SvgContextRes>,
) -> FillSvgBundle {
    match paint.variant {
        CompPaintVariant::Solid => {
            FillSvgBundle::Solid(SolidFillSvgBundle::new(fill.paint, &mut svg_context_res))
        }
    }
}
