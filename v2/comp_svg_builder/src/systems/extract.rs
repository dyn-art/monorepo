#![cfg(feature = "output_svg_element_changes")]

use crate::{
    resources::changed_svg_bundles::{ChangedSvgBundle, ChangedSvgBundlesRes},
    svg::svg_bundle::node::NodeSvgBundleMixin,
};
use bevy_ecs::{
    entity::Entity,
    query::{Changed, With},
    system::{Query, ResMut},
};
use bevy_hierarchy::{Children, Parent};
use dyn_comp_types::nodes::CompNode;

pub fn extract_svg_bundles(
    mut changed_svg_bundles_res: ResMut<ChangedSvgBundlesRes>,
    mut query: Query<
        (Entity, &mut NodeSvgBundleMixin, Option<&Parent>),
        (With<CompNode>, Changed<NodeSvgBundleMixin>),
    >,
    child_query: Query<&Children>,
) {
    for (entity, mut bundle_mixin, maybe_parent) in query.iter_mut() {
        let NodeSvgBundleMixin(bundle) = bundle_mixin.as_mut();
        let changes = bundle.get_svg_bundle_mut().drain_changes();
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

        changed_svg_bundles_res.push_change(ChangedSvgBundle {
            entity,
            parent_entity,
            elements_changes: changes,
            index,
        });
    }
}
