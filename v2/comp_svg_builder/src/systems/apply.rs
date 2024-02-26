use crate::{
    resources::delayed_node_modifications::{
        DelayedNodeModificationsRes, SvgNodeChildrenModification,
    },
    svg::{
        svg_element::{
            attributes::{SvgAttribute, SvgMeasurementUnit},
            styles::{SvgDisplayStyle, SvgStyle},
        },
        svg_node::SvgNodeVariant,
    },
};
use bevy_ecs::{
    entity::Entity,
    query::{Changed, With},
    system::{Commands, Query, ResMut},
};
use bevy_hierarchy::Children;
use bevy_transform::components::Transform;
use dyn_comp_types::{
    mixins::{BlendModeMixin, FillMixin, OpacityMixin, SizeMixin, VisibilityMixin},
    nodes::CompNode,
    shared::Visibility,
};
use std::collections::HashSet;

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

pub fn apply_visibility_mixin_changes(
    mut query: Query<
        (&VisibilityMixin, &mut SvgNodeVariant),
        (With<CompNode>, Changed<VisibilityMixin>),
    >,
) {
    query
        .iter_mut()
        .for_each(|(VisibilityMixin(visibility), mut node_variant)| {
            let element = match node_variant.as_mut() {
                SvgNodeVariant::Frame(node) => &mut node.root,
                SvgNodeVariant::Shape(node) => &mut node.root,
            };

            let display = match visibility {
                Visibility::Visible => SvgDisplayStyle::Block,
                Visibility::Hidden => SvgDisplayStyle::None,
            };
            element.set_style(SvgStyle::Display { display });
        });
}

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

pub fn apply_transform_changes(
    mut query: Query<(&Transform, &mut SvgNodeVariant), (With<CompNode>, Changed<Transform>)>,
) {
    query.iter_mut().for_each(|(transform, mut node_variant)| {
        let element = match node_variant.as_mut() {
            SvgNodeVariant::Frame(node) => &mut node.root,
            SvgNodeVariant::Shape(node) => &mut node.root,
        };

        element.set_attribute(SvgAttribute::Transform {
            transform: transform.into(),
        });
    });
}

pub fn apply_opacity_mixin_changes(
    mut query: Query<(&OpacityMixin, &mut SvgNodeVariant), (With<CompNode>, Changed<OpacityMixin>)>,
) {
    query
        .iter_mut()
        .for_each(|(OpacityMixin(opacity), mut node_variant)| {
            let element = match node_variant.as_mut() {
                SvgNodeVariant::Frame(node) => &mut node.root,
                SvgNodeVariant::Shape(node) => &mut node.root,
            };

            element.set_attribute(SvgAttribute::Opacity {
                opacity: opacity.0.get(),
            });
        });
}

pub fn apply_blend_mode_mixin_changes(
    mut query: Query<
        (&BlendModeMixin, &mut SvgNodeVariant),
        (With<CompNode>, Changed<BlendModeMixin>),
    >,
) {
    query
        .iter_mut()
        .for_each(|(BlendModeMixin(blend_mode), mut node_variant)| {
            let element = match node_variant.as_mut() {
                SvgNodeVariant::Frame(node) => &mut node.root,
                SvgNodeVariant::Shape(node) => &mut node.root,
            };

            element.set_style(SvgStyle::BlendMode {
                blend_mode: blend_mode.into(),
            });
        });
}

pub fn apply_fill_mixin_changes(
    mut query: Query<(&FillMixin, &mut SvgNodeVariant), (With<CompNode>, Changed<FillMixin>)>,
) {
    query.iter_mut().for_each(|(fill_mixin, mut node_variant)| {
        let fill_wrapper_element = match node_variant.as_mut() {
            SvgNodeVariant::Frame(node) => &mut node.fill_wrapper_g,
            SvgNodeVariant::Shape(node) => &mut node.fill_wrapper_g,
        };

        // TODO
    });
}
