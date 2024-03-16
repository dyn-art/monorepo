#![cfg(feature = "output_svg_element_changes")]

use crate::{
    resources::changed_svg_bundles::{ChangedSvgBundle, ChangedSvgBundlesRes},
    svg::svg_bundle::SvgBundleVariant,
};
use bevy_ecs::{
    entity::Entity,
    query::{Changed, With},
    system::{Query, ResMut},
};
use bevy_hierarchy::{Children, Parent};
use dyn_comp_common::{
    mixins::{StyleChildrenMixin, StyleParentMixin},
    nodes::CompNode,
    styles::CompStyle,
};

pub fn extract_node_bundles(
    mut changed_svg_bundles_res: ResMut<ChangedSvgBundlesRes>,
    mut query: Query<
        (Entity, &mut SvgBundleVariant, Option<&Parent>),
        (With<CompNode>, Changed<SvgBundleVariant>),
    >,
    child_query: Query<&Children>,
) {
    for (entity, mut bundle_variant, maybe_parent) in query.iter_mut() {
        let (elements_changes, deferred_elements_changes) =
            bundle_variant.get_svg_bundle_mut().drain_changes();

        if !elements_changes.is_empty() {
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
                elements_changes,
                index,
            });
        }

        if !deferred_elements_changes.is_empty() {
            for deferred_change in deferred_elements_changes {
                changed_svg_bundles_res.push_deferred_change(deferred_change);
            }
        }
    }
}

pub fn extract_style_bundles(
    mut changed_svg_bundles_res: ResMut<ChangedSvgBundlesRes>,
    mut query: Query<
        (Entity, &mut SvgBundleVariant, Option<&StyleParentMixin>),
        (With<CompStyle>, Changed<SvgBundleVariant>),
    >,
    child_query: Query<&StyleChildrenMixin>,
) {
    for (entity, mut bundle_variant, maybe_parent) in query.iter_mut() {
        let (elements_changes, deferred_elements_changes) =
            bundle_variant.get_svg_bundle_mut().drain_changes();

        if !elements_changes.is_empty() {
            // Try to get parent entity and the current entity's position in the parent's children array
            let (parent_entity, index) =
                if let Some(parent_entity) = maybe_parent.map(|parent| parent.0) {
                    if let Ok(children) = child_query.get(parent_entity) {
                        children
                            .0
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
                elements_changes,
                index,
            });
        }

        if !deferred_elements_changes.is_empty() {
            for deferred_change in deferred_elements_changes {
                changed_svg_bundles_res.push_deferred_change(deferred_change);
            }
        }
    }
}
