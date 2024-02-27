use crate::{
    resources::delayed_svg_bundle_modifications::{
        DelayedSvgBundleModificationsRes, SvgBundleChildrenModification,
    },
    svg::{
        svg_bundle::NodeSvgBundleVariant,
        svg_element::{
            attributes::{SvgAttribute, SvgMeasurementUnit},
            styles::{SvgDisplayStyle, SvgStyle},
        },
    },
};
use bevy_ecs::{
    entity::Entity,
    query::{Changed, With},
    system::{Commands, Query, ResMut},
};
use bevy_hierarchy::{BuildChildren, Children};
use bevy_transform::components::Transform;
use dyn_comp_types::{
    mixins::{BlendModeMixin, FillMixin, OpacityMixin, SizeMixin, VisibilityMixin},
    nodes::CompNode,
    shared::Visibility,
};
use std::collections::HashSet;

pub fn collect_children_changes(
    mut delayed_node_modification_res: ResMut<DelayedSvgBundleModificationsRes>,
    query: Query<
        (Entity, &Children, &mut NodeSvgBundleVariant),
        (With<CompNode>, Changed<Children>),
    >,
) {
    for (entity, children, bundle_variant) in query.iter() {
        let maybe_node_children = match &bundle_variant {
            NodeSvgBundleVariant::Frame(bundle) => Some(&bundle.node_children),
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
                SvgBundleChildrenModification {
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
    mut delayed_svg_bundle_modifications_res: ResMut<DelayedSvgBundleModificationsRes>,
    mut bundle_query: Query<&mut NodeSvgBundleVariant>,
) {
    let modifications = delayed_svg_bundle_modifications_res
        .children_modifications
        .drain(..)
        .collect::<Vec<_>>();

    for modification in modifications {
        // Process removed entities
        for entity in modification.removed_entities {
            let [mut bundle, child_bundle] =
                bundle_query.many_mut([modification.parent_entity, entity]);
            match bundle.as_mut() {
                NodeSvgBundleVariant::Frame(frame_node) => {
                    frame_node
                        .children_wrapper_g
                        .remove_child(child_bundle.get_svg_bundle().get_root_element().get_id());
                    // https://bevy-cheatbook.github.io/fundamentals/hierarchy.html#despawning-child-entities
                    commands.entity(entity).despawn();
                    commands
                        .entity(modification.parent_entity)
                        .remove_children(&[entity]);
                }
                _ => {}
            }
        }

        // Process added entities
        for entity in modification.added_entities {
            let [mut bundle, mut child_bundle] =
                bundle_query.many_mut([modification.parent_entity, entity]);
            match bundle.as_mut() {
                NodeSvgBundleVariant::Frame(frame_node) => {
                    frame_node.children_wrapper_g.append_child_in_world_context(
                        entity,
                        child_bundle.get_svg_bundle_mut().get_root_element_mut(),
                    );
                }
                _ => {}
            }
        }
    }
}

pub fn apply_visibility_mixin_changes(
    mut query: Query<
        (&VisibilityMixin, &mut NodeSvgBundleVariant),
        (With<CompNode>, Changed<VisibilityMixin>),
    >,
) {
    query
        .iter_mut()
        .for_each(|(VisibilityMixin(visibility), mut bundle_variant)| {
            let element = match bundle_variant.as_mut() {
                NodeSvgBundleVariant::Frame(bundle) => &mut bundle.root,
                NodeSvgBundleVariant::Shape(bundle) => &mut bundle.root,
            };

            let display = match visibility {
                Visibility::Visible => SvgDisplayStyle::Block,
                Visibility::Hidden => SvgDisplayStyle::None,
            };
            element.set_style(SvgStyle::Display { display });
        });
}

pub fn apply_size_mixin_changes(
    mut query: Query<(&SizeMixin, &mut NodeSvgBundleVariant), (With<CompNode>, Changed<SizeMixin>)>,
) {
    query
        .iter_mut()
        .for_each(|(SizeMixin(size), mut bundle_variant)| {
            let [width, height] = size.0.to_array();

            match bundle_variant.as_mut() {
                NodeSvgBundleVariant::Frame(bundle) => {
                    bundle.root.set_attributes(vec![
                        SvgAttribute::Width {
                            width,
                            unit: SvgMeasurementUnit::Pixel,
                        },
                        SvgAttribute::Height {
                            height,
                            unit: SvgMeasurementUnit::Pixel,
                        },
                    ]);
                    bundle.fill_clipped_path.set_attributes(vec![
                        SvgAttribute::Width {
                            width,
                            unit: SvgMeasurementUnit::Pixel,
                        },
                        SvgAttribute::Height {
                            height,
                            unit: SvgMeasurementUnit::Pixel,
                        },
                    ]);
                    bundle.content_clipped_rect.set_attributes(vec![
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
                NodeSvgBundleVariant::Shape(bundle) => {
                    bundle.root.set_attributes(vec![
                        SvgAttribute::Width {
                            width,
                            unit: SvgMeasurementUnit::Pixel,
                        },
                        SvgAttribute::Height {
                            height,
                            unit: SvgMeasurementUnit::Pixel,
                        },
                    ]);
                    bundle.click_area_rect.set_attributes(vec![
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
    mut query: Query<(&Transform, &mut NodeSvgBundleVariant), (With<CompNode>, Changed<Transform>)>,
) {
    query
        .iter_mut()
        .for_each(|(transform, mut bundle_variant)| {
            let element = match bundle_variant.as_mut() {
                NodeSvgBundleVariant::Frame(bundle) => &mut bundle.root,
                NodeSvgBundleVariant::Shape(bundle) => &mut bundle.root,
            };

            element.set_attribute(SvgAttribute::Transform {
                transform: transform.into(),
            });
        });
}

pub fn apply_opacity_mixin_changes(
    mut query: Query<
        (&OpacityMixin, &mut NodeSvgBundleVariant),
        (With<CompNode>, Changed<OpacityMixin>),
    >,
) {
    query
        .iter_mut()
        .for_each(|(OpacityMixin(opacity), mut bundle_variant)| {
            let element = match bundle_variant.as_mut() {
                NodeSvgBundleVariant::Frame(bundle) => &mut bundle.root,
                NodeSvgBundleVariant::Shape(bundle) => &mut bundle.root,
            };

            element.set_attribute(SvgAttribute::Opacity {
                opacity: opacity.0.get(),
            });
        });
}

pub fn apply_blend_mode_mixin_changes(
    mut query: Query<
        (&BlendModeMixin, &mut NodeSvgBundleVariant),
        (With<CompNode>, Changed<BlendModeMixin>),
    >,
) {
    query
        .iter_mut()
        .for_each(|(BlendModeMixin(blend_mode), mut bundle_variant)| {
            let element = match bundle_variant.as_mut() {
                NodeSvgBundleVariant::Frame(bundle) => &mut bundle.root,
                NodeSvgBundleVariant::Shape(bundle) => &mut bundle.root,
            };

            element.set_style(SvgStyle::BlendMode {
                blend_mode: blend_mode.into(),
            });
        });
}

pub fn apply_fill_mixin_changes(
    mut query: Query<(&FillMixin, &mut NodeSvgBundleVariant), (With<CompNode>, Changed<FillMixin>)>,
) {
    query
        .iter_mut()
        .for_each(|(FillMixin(fills), mut bundle_variant)| {
            let fill_wrapper_element = match bundle_variant.as_mut() {
                NodeSvgBundleVariant::Frame(bundle) => &mut bundle.fill_wrapper_g,
                NodeSvgBundleVariant::Shape(bundle) => &mut bundle.fill_wrapper_g,
            };

            // TODO
        });
}
