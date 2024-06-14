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
use dyn_cnv_bundles::components::{
    mixins::{HierarchyLevel, StyleChildrenMixin, StyleParentMixin},
    nodes::CnvNode,
    styles::CnvStyle,
};

pub fn extract_node_bundles(
    mut changed_svg_bundles_res: ResMut<ChangedSvgBundlesRes>,
    mut query: Query<
        (
            Entity,
            &mut SvgBundleVariant,
            Option<&Parent>,
            Option<&HierarchyLevel>,
        ),
        (With<CnvNode>, Changed<SvgBundleVariant>),
    >,
    children_query: Query<&Children>,
) {
    let default_hierarchy_level = 0;
    let default_child_index = 0;

    for (entity, mut bundle_variant, maybe_parent, maybe_hierarchy_level) in query.iter_mut() {
        let (elements_changes, deferred_elements_changes) =
            bundle_variant.get_svg_bundle_mut().drain_changes();

        if !elements_changes.is_empty() {
            // Try to get parent entity and the current entity's position in the parent's children array
            let child_index = if let Some(parent_entity) = maybe_parent.map(|parent| parent.get()) {
                if let Ok(children) = children_query.get(parent_entity) {
                    children
                        .iter()
                        .position(|&child| child == entity)
                        .map(|index| index)
                        .unwrap_or(default_child_index)
                }
                // No children found, default index to 0
                else {
                    default_child_index
                }
            }
            // No parent, so no index
            else {
                default_child_index
            };

            changed_svg_bundles_res.push_change(ChangedSvgBundle {
                entity,
                elements_changes,
                child_index,
                hierarchy_level: maybe_hierarchy_level
                    .map(|level| level.0)
                    .unwrap_or(default_hierarchy_level),
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
        (Entity, &mut SvgBundleVariant, &StyleParentMixin),
        (With<CnvStyle>, Changed<SvgBundleVariant>),
    >,
    children_query: Query<(&StyleChildrenMixin, Option<&HierarchyLevel>)>,
) {
    let default_hierarchy_level = 1;
    let default_child_index = 0;

    for (entity, mut bundle_variant, StyleParentMixin(parent_entity)) in query.iter_mut() {
        let (elements_changes, deferred_elements_changes) =
            bundle_variant.get_svg_bundle_mut().drain_changes();

        if !elements_changes.is_empty() {
            // Try to get parent entity and the current entity's position in the parent's children array
            let (hierarchy_level, child_index) =
                if let Ok((StyleChildrenMixin(children), maybe_hierarchy_level)) =
                    children_query.get(*parent_entity)
                {
                    // Increment hierarchy level by 1 because the queried hierarchy level is from the parent
                    let hierarchy_level = maybe_hierarchy_level
                        .map(|level| level.0 + 1)
                        .unwrap_or(default_hierarchy_level);
                    children
                        .iter()
                        .position(|&child| child == entity)
                        .map(|index| (hierarchy_level, index))
                        .unwrap_or((hierarchy_level, default_child_index))
                }
                // No children found, default index to 0
                else {
                    (default_hierarchy_level, default_child_index)
                };

            changed_svg_bundles_res.push_change(ChangedSvgBundle {
                entity,
                elements_changes,
                child_index,
                hierarchy_level,
            });
        }

        if !deferred_elements_changes.is_empty() {
            for deferred_change in deferred_elements_changes {
                changed_svg_bundles_res.push_deferred_change(deferred_change);
            }
        }
    }
}
