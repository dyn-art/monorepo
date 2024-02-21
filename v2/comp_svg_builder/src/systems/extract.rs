use bevy_ecs::{
    entity::Entity,
    query::{Changed, With},
    system::{Query, ResMut},
};
use dyn_comp_types::{bevy_hierarchy::Parent, nodes::CompNode};

use crate::{
    resources::changed_svg_nodes::{ChangedSVGNode, ChangedSVGNodesRes},
    svg::svg_node::SVGNode,
};

pub fn extract_svg_nodes_generic<C: SVGNode>(
    mut changed_svg_nodes_res: ResMut<ChangedSVGNodesRes>,
    mut query: Query<(Entity, &mut C, Option<&Parent>), (With<CompNode>, Changed<C>)>,
) {
    query
        .iter_mut()
        .for_each(|(entity, mut svg_node, maybe_parent)| {
            changed_svg_nodes_res.changes.push(ChangedSVGNode {
                entity,
                parent_entity: maybe_parent.map(|parent| parent.get()),
                changes: svg_node.drain_changes(),
            })
        });
}
