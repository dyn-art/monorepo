#![cfg(feature = "output_events")]

use bevy_ecs::{
    entity::Entity,
    query::{Changed, With},
    system::{Query, ResMut},
};
use bevy_hierarchy::{Children, Parent};
use dyn_comp_types::nodes::CompNode;

use crate::{
    resources::changed_svg_nodes::{ChangedSvgNode, ChangedSvgNodesRes},
    svg::svg_node::SvgNode,
};

pub fn extract_svg_nodes_generic<C: SvgNode>(
    mut changed_svg_nodes_res: ResMut<ChangedSvgNodesRes>,
    mut query: Query<(Entity, &mut C, Option<&Parent>), (With<CompNode>, Changed<C>)>,
    child_query: Query<&Children>,
) {
    query
        .iter_mut()
        .for_each(|(entity, mut svg_node, maybe_parent)| {
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

            changed_svg_nodes_res.push_change(ChangedSvgNode {
                entity,
                parent_entity,
                changes: svg_node.drain_changes(),
                index,
            });
        });
}
