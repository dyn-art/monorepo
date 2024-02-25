#![cfg(feature = "output_events")]

use crate::{
    resources::changed_svg_nodes::{ChangedSvgNode, ChangedSvgNodesRes},
    svg::svg_node::SvgNode,
};
use bevy_ecs::{
    entity::Entity,
    query::{Changed, With},
    system::{Query, ResMut},
};
use bevy_hierarchy::{Children, Parent};
use dyn_comp_types::nodes::CompNode;

pub fn extract_svg_nodes_generic<C: SvgNode>(
    mut changed_svg_nodes_res: ResMut<ChangedSvgNodesRes>,
    mut query: Query<(Entity, &mut C, Option<&Parent>), (With<CompNode>, Changed<C>)>,
    child_query: Query<&Children>,
) {
    query
        .iter_mut()
        .for_each(|(entity, mut svg_node, maybe_parent)| {
            let changes = svg_node.drain_changes();
            if changes.is_empty() {
                return;
            }

            // Try to get parent entity and the current entity's position in the parent's children array
            let (parent_entity, index) =
                if let Some(parent_entity) = maybe_parent.map(|parent| parent.get()) {
                    if let Ok(children) = child_query.get(parent_entity) {
                        children
                            .iter()
                            .position(|&child| child == entity)
                            .map(|index| (Some(parent_entity), index))
                            .unwrap_or((Some(parent_entity), 0))
                    }
                    // No children found, default index to 0
                    else {
                        (Some(parent_entity), 0)
                    }
                }
                // No parent, so no index
                else {
                    (None, 0)
                };

            log::info!(
                "[extract_svg_nodes_generic] {:?} ({:?}, {:?})",
                entity,
                parent_entity,
                index
            );

            changed_svg_nodes_res.push_change(ChangedSvgNode {
                entity,
                parent_entity,
                changes,
                index,
            });
        });
}
