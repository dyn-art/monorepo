use crate::{
    resources::delayed_node_modifications::{
        DelayedNodeModificationsRes, SvgNodeChildrenModification,
    },
    svg::{
        svg_element::attributes::{SvgAttribute, SvgMeasurementUnit},
        svg_node::SvgNodeVariant,
    },
};
use bevy_ecs::{
    entity::Entity,
    query::{Changed, With},
    system::{Commands, Query, ResMut},
};
use bevy_hierarchy::Children;
use dyn_comp_types::{mixins::SizeMixin, nodes::CompNode};
use std::collections::HashSet;

pub fn apply_size_mixin_changes(
    mut query: Query<(&SizeMixin, &mut SvgNodeVariant), (With<CompNode>, Changed<SizeMixin>)>,
) {
    query
        .iter_mut()
        .for_each(|(SizeMixin(size), mut node_variant)| {
            let [width, height] = size.0.to_array();

            match node_variant.as_mut() {
                SvgNodeVariant::Frame(node) => {
                    node.root.set_attributes(vec![
                        SvgAttribute::Width {
                            width,
                            unit: SvgMeasurementUnit::Pixel,
                        },
                        SvgAttribute::Height {
                            height,
                            unit: SvgMeasurementUnit::Pixel,
                        },
                    ]);
                    node.fill_clipped_path.set_attributes(vec![
                        SvgAttribute::Width {
                            width,
                            unit: SvgMeasurementUnit::Pixel,
                        },
                        SvgAttribute::Height {
                            height,
                            unit: SvgMeasurementUnit::Pixel,
                        },
                    ]);
                    node.content_clipped_rect.set_attributes(vec![
                        SvgAttribute::Width {
                            width,
                            unit: SvgMeasurementUnit::Pixel,
                        },
                        SvgAttribute::Height {
                            height,
                            unit: SvgMeasurementUnit::Pixel,
                        },
                    ]);
                }
                SvgNodeVariant::Shape(node) => {
                    node.root.set_attributes(vec![
                        SvgAttribute::Width {
                            width,
                            unit: SvgMeasurementUnit::Pixel,
                        },
                        SvgAttribute::Height {
                            height,
                            unit: SvgMeasurementUnit::Pixel,
                        },
                    ]);
                    node.click_area_rect.set_attributes(vec![
                        SvgAttribute::Width {
                            width,
                            unit: SvgMeasurementUnit::Pixel,
                        },
                        SvgAttribute::Height {
                            height,
                            unit: SvgMeasurementUnit::Pixel,
                        },
                    ]);
                }
            }
        });
}

pub fn collect_children_changes(
    mut delayed_node_modification_res: ResMut<DelayedNodeModificationsRes>,
    query: Query<(Entity, &Children, &mut SvgNodeVariant), (With<CompNode>, Changed<Children>)>,
) {
    for (entity, children, node_variant) in query.iter() {
        let maybe_node_children = match &node_variant {
            SvgNodeVariant::Frame(node) => Some(&node.node_children),
            _ => None,
        };
        if let Some(node_children) = maybe_node_children {
            let current_node_children_set: HashSet<_> = node_children.iter().cloned().collect();
            let new_node_children_set: HashSet<_> = children.iter().cloned().collect();

            let removed_node_entities: Vec<_> = current_node_children_set
                .difference(&new_node_children_set)
                .cloned()
                .collect();

            let added_node_entities: Vec<_> = new_node_children_set
                .difference(&current_node_children_set)
                .cloned()
                .collect();

            delayed_node_modification_res.children_modifications.push(
                SvgNodeChildrenModification {
                    parent_entity: entity,
                    added_entities: added_node_entities,
                    removed_entities: removed_node_entities,
                },
            );
        }
    }
}

pub fn apply_children_changes(
    mut commands: Commands,
    mut delayed_node_modification_res: ResMut<DelayedNodeModificationsRes>,
    mut node_query: Query<&mut SvgNodeVariant>,
) {
    let modifications = delayed_node_modification_res
        .children_modifications
        .drain(..)
        .collect::<Vec<_>>();

    for modification in modifications {
        // Process removed entities
        for entity in modification.removed_entities {
            let [mut node, child_node] = node_query.many_mut([modification.parent_entity, entity]);
            match node.as_mut() {
                SvgNodeVariant::Frame(frame_node) => {
                    frame_node
                        .children_wrapper_g
                        .remove_child(child_node.get_svg_node().get_root_element().get_id());
                    commands.entity(entity).despawn();
                }
                _ => {}
            }
        }

        // Process added entities
        for entity in modification.added_entities {
            let [mut node, mut child_node] =
                node_query.many_mut([modification.parent_entity, entity]);
            match node.as_mut() {
                SvgNodeVariant::Frame(frame_node) => {
                    frame_node.children_wrapper_g.append_child_in_world_context(
                        entity,
                        child_node.get_svg_node_mut().get_root_element_mut(),
                    );
                }
                _ => {}
            }
        }
    }
}
