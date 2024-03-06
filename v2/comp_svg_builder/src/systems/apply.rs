use crate::svg::{
    svg_bundle::{FillSvgBundle, NodeSvgBundle, NodeSvgBundleMixin},
    svg_element::{
        attributes::{SvgAttribute, SvgMeasurementUnit},
        element_changes::{SvgElementChange, SvgElementReorderedChange},
        styles::{SvgDisplayStyle, SvgFillStyle, SvgStyle},
        SvgElementId,
    },
};
use bevy_ecs::{
    entity::Entity,
    query::{Changed, With},
    system::{ParamSet, Query},
};
use bevy_hierarchy::Children;
use bevy_transform::components::Transform;
use dyn_comp_types::{
    common::Visibility,
    mixins::{
        BlendModeMixin, OpacityMixin, PaintParentMixin, PathMixin, SizeMixin, VisibilityMixin,
    },
    nodes::CompNode,
    paints::{CompPaint, SolidCompPaint},
};
use smallvec::SmallVec;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct SvgBundleChildrenModification {
    pub parent_entity: Entity,
    pub new_entities_order: SmallVec<[Entity; 2]>,
    pub added_entities: SmallVec<[Entity; 2]>,
    pub removed_entities: SmallVec<[Entity; 2]>,
}

pub fn apply_node_children_changes(
    // https://bevyengine.org/learn/errors/
    mut queries: ParamSet<(
        Query<(Entity, &Children, &mut NodeSvgBundleMixin), (With<CompNode>, Changed<Children>)>,
        Query<&mut NodeSvgBundleMixin>,
    )>,
) {
    let mut modifications: Vec<SvgBundleChildrenModification> = Vec::new();

    // Query modifications
    {
        let children_query = queries.p0();
        for (entity, children, NodeSvgBundleMixin(bundle)) in children_query.iter() {
            let node_children = match bundle.get_child_nodes() {
                Some(node_children) => node_children,
                None => return,
            };

            // Identify removed and newly added node entities
            let current_node_children_set = node_children.iter().copied().collect::<HashSet<_>>();
            let new_node_children_set = children.iter().copied().collect::<HashSet<_>>();
            let removed_node_entities = current_node_children_set
                .difference(&new_node_children_set)
                .cloned()
                .collect::<SmallVec<_>>();
            let added_node_entities = new_node_children_set
                .difference(&current_node_children_set)
                .cloned()
                .collect::<SmallVec<_>>();

            modifications.push(SvgBundleChildrenModification {
                parent_entity: entity,
                added_entities: added_node_entities,
                removed_entities: removed_node_entities,
                new_entities_order: node_children.iter().copied().collect(),
            });
        }
    }

    // Apply modifications (Deferred apply of node modifications to avoid query conflicts)
    {
        let mut node_query = queries.p1();
        for modification in modifications {
            process_removed_node_children(
                modification.parent_entity,
                &modification.removed_entities,
                &mut node_query,
            );
            process_added_node_children(
                modification.parent_entity,
                &modification.added_entities,
                &mut node_query,
            );
            reorder_node_children(
                modification.parent_entity,
                &modification.new_entities_order,
                &mut node_query,
            );
        }
    }
}

fn process_removed_node_children(
    parent_entity: Entity,
    removed_entities: &[Entity],
    node_query: &mut Query<&mut NodeSvgBundleMixin>,
) {
    for entity in removed_entities {
        let [mut bundle_mixin, child_bundle_mixin] = node_query.many_mut([parent_entity, *entity]);
        let NodeSvgBundleMixin(bundle) = bundle_mixin.as_mut();
        let NodeSvgBundleMixin(child_bundle) = child_bundle_mixin.as_ref();
        match bundle {
            NodeSvgBundle::Frame(frame_node) => {
                frame_node
                    .children_wrapper_g
                    .remove_child(child_bundle.get_svg_bundle().get_root_element().get_id());
            }
            _ => {}
        }
    }
}

fn process_added_node_children(
    parent_entity: Entity,
    added_entities: &[Entity],
    node_query: &mut Query<&mut NodeSvgBundleMixin>,
) {
    for entity in added_entities {
        let [mut bundle_mixin, mut child_bundle_mixin] =
            node_query.many_mut([parent_entity, *entity]);
        let NodeSvgBundleMixin(bundle) = bundle_mixin.as_mut();
        let NodeSvgBundleMixin(child_bundle) = child_bundle_mixin.as_mut();
        match bundle {
            NodeSvgBundle::Frame(bundle) => {
                bundle.children_wrapper_g.append_child_in_world_context(
                    *entity,
                    child_bundle.get_svg_bundle_mut().get_root_element_mut(),
                );
            }
            _ => {}
        }
    }
}

fn reorder_node_children(
    parent_entity: Entity,
    new_entities_order: &[Entity],
    node_query: &mut Query<&mut NodeSvgBundleMixin>,
) {
    // Track the original positions of the node children
    #[cfg(feature = "output_svg_element_changes")]
    let original_positions = {
        let NodeSvgBundleMixin(node_bundle) = match node_query.get(parent_entity) {
            Ok(bundle_mixin) => bundle_mixin,
            Err(_) => return,
        };
        let bundle_children = match node_bundle.get_child_nodes() {
            Some(bundle_children) => bundle_children,
            None => return,
        };
        bundle_children
            .iter()
            .filter_map(|entity| match node_query.get(*entity) {
                Ok(NodeSvgBundleMixin(bundle)) => {
                    Some((*entity, bundle.get_svg_bundle().get_root_element().get_id()))
                }
                Err(_) => None,
            })
            .collect::<SmallVec<[(Entity, SvgElementId); 2]>>()
    };

    let mut bundle_mixin = match node_query.get_mut(parent_entity) {
        Ok(bundle_mixin) => bundle_mixin,
        Err(_) => return,
    };
    let NodeSvgBundleMixin(node_bundle) = bundle_mixin.as_mut();
    let bundle_children = match node_bundle.get_child_nodes_mut() {
        Some(bundle_children) => bundle_children,
        None => return,
    };

    // Sort bundle children
    bundle_children.sort_by_key(|bundle_child| {
        new_entities_order
            .iter()
            .position(|entity| *bundle_child == *entity)
            .unwrap_or(usize::MAX)
    });

    #[cfg(feature = "output_svg_element_changes")]
    {
        // Determine the new positions after sorting
        let new_positions = original_positions
            .iter()
            .map(|(_, id)| *id)
            .collect::<Vec<_>>();

        // Emit SvgElementReorderedChange events for node children that have been moved
        for (new_index, &element_id) in new_positions.iter().enumerate() {
            let original_index = original_positions
                .iter()
                .position(|(_, e)| *e == element_id)
                .unwrap_or(new_index);

            // If the node has been moved
            if original_index != new_index {
                let children_wrapper_element = match node_bundle.get_children_wrapper_element_mut()
                {
                    Some(children_wrapper_element) => children_wrapper_element,
                    None => return,
                };
                let new_parent_id = children_wrapper_element.get_id();

                // Determine insert_before_id based on the next sibling in the new order
                let insert_before_id = if new_index + 1 < new_positions.len() {
                    // There is a next sibling, get its ID
                    Some(new_positions[new_index + 1])
                } else {
                    // No next sibling, append at the end
                    None
                };

                children_wrapper_element.register_change(SvgElementChange::ElementReordered(
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

pub fn apply_visibility_mixin_changes(
    mut query: Query<
        (&VisibilityMixin, &mut NodeSvgBundleMixin),
        (With<CompNode>, Changed<VisibilityMixin>),
    >,
) {
    for (VisibilityMixin(visibility), mut bundle_mixin) in query.iter_mut() {
        let NodeSvgBundleMixin(bundle) = bundle_mixin.as_mut();
        let element = match bundle {
            NodeSvgBundle::Frame(bundle) => &mut bundle.root_g,
            NodeSvgBundle::Shape(bundle) => &mut bundle.root_g,
        };

        let display = match visibility {
            Visibility::Visible => SvgDisplayStyle::Block,
            Visibility::Hidden => SvgDisplayStyle::None,
        };
        element.set_style(SvgStyle::Display { display });
    }
}

pub fn apply_size_mixin_changes(
    mut query: Query<(&SizeMixin, &mut NodeSvgBundleMixin), (With<CompNode>, Changed<SizeMixin>)>,
) {
    for (SizeMixin(size), mut bundle_mixin) in query.iter_mut() {
        let NodeSvgBundleMixin(bundle) = bundle_mixin.as_mut();
        let [width, height] = size.0.to_array();

        // Apply dimension change to node
        match bundle {
            NodeSvgBundle::Frame(bundle) => {
                bundle.root_g.set_attributes(vec![
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
            NodeSvgBundle::Shape(bundle) => {
                bundle.root_g.set_attributes(vec![
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
    }
}

pub fn apply_transform_changes(
    mut query: Query<(&Transform, &mut NodeSvgBundleMixin), (With<CompNode>, Changed<Transform>)>,
) {
    for (transform, mut bundle_mixin) in query.iter_mut() {
        let NodeSvgBundleMixin(bundle) = bundle_mixin.as_mut();
        let element = match bundle {
            NodeSvgBundle::Frame(bundle) => &mut bundle.root_g,
            NodeSvgBundle::Shape(bundle) => &mut bundle.root_g,
        };

        element.set_attribute(SvgAttribute::Transform {
            transform: transform.into(),
        });
    }
}

pub fn apply_opacity_mixin_changes(
    mut query: Query<
        (&OpacityMixin, &mut NodeSvgBundleMixin),
        (With<CompNode>, Changed<OpacityMixin>),
    >,
) {
    for (OpacityMixin(opacity), mut bundle_mixin) in query.iter_mut() {
        let NodeSvgBundleMixin(bundle) = bundle_mixin.as_mut();
        let element = match bundle {
            NodeSvgBundle::Frame(bundle) => &mut bundle.root_g,
            NodeSvgBundle::Shape(bundle) => &mut bundle.root_g,
        };

        element.set_style(SvgStyle::Opacity {
            opacity: opacity.0.get(),
        });
    }
}

pub fn apply_blend_mode_mixin_changes(
    mut query: Query<
        (&BlendModeMixin, &mut NodeSvgBundleMixin),
        (With<CompNode>, Changed<BlendModeMixin>),
    >,
) {
    for (BlendModeMixin(blend_mode), mut bundle_mixin) in query.iter_mut() {
        let NodeSvgBundleMixin(bundle) = bundle_mixin.as_mut();
        let element = match bundle {
            NodeSvgBundle::Frame(bundle) => &mut bundle.root_g,
            NodeSvgBundle::Shape(bundle) => &mut bundle.root_g,
        };

        element.set_style(SvgStyle::BlendMode {
            blend_mode: blend_mode.into(),
        });
    }
}

pub fn apply_path_mixin_changes(
    mut query: Query<(&PathMixin, &mut NodeSvgBundleMixin), (With<CompNode>, Changed<PathMixin>)>,
) {
    for (PathMixin(path), mut bundle_mixin) in query.iter_mut() {
        let NodeSvgBundleMixin(bundle) = bundle_mixin.as_mut();

        match bundle {
            NodeSvgBundle::Frame(frame_bundle) => {
                for fill_bundle in &mut frame_bundle.fill_bundles {
                    match fill_bundle {
                        FillSvgBundle::Solid(solid_bundle) => solid_bundle
                            .shape_path
                            .set_attribute(SvgAttribute::D { d: path.into() }),
                    }
                }
                frame_bundle
                    .children_clipped_path
                    .set_attribute(SvgAttribute::D { d: path.into() })
            }
            NodeSvgBundle::Shape(shape_bundle) => {
                for fill_bundle in &mut shape_bundle.fill_bundles {
                    match fill_bundle {
                        FillSvgBundle::Solid(solid_bundle) => solid_bundle
                            .shape_path
                            .set_attribute(SvgAttribute::D { d: path.into() }),
                    }
                }
            }
        }
    }
}

pub fn apply_solid_paint_changes(
    query: Query<
        (Entity, &SolidCompPaint, &PaintParentMixin),
        (With<CompPaint>, Changed<SolidCompPaint>),
    >,
    mut node_bundle_query: Query<&mut NodeSvgBundleMixin>,
) {
    for (paint_entity, solid_paint, PaintParentMixin(node_entities)) in query.iter() {
        for node_entity in node_entities {
            if let Ok(mut bundle_mixin) = node_bundle_query.get_mut(*node_entity) {
                let NodeSvgBundleMixin(bundle) = bundle_mixin.as_mut();
                let fill_bundles = match bundle {
                    NodeSvgBundle::Frame(bundle) => &mut bundle.fill_bundles,
                    NodeSvgBundle::Shape(bundle) => &mut bundle.fill_bundles,
                    _ => return,
                };

                if let Some(fill_bundle) = fill_bundles
                    .iter_mut()
                    .find(|fill| *fill.get_paint_entity() == paint_entity)
                {
                    match fill_bundle {
                        FillSvgBundle::Solid(solid_bundle) => {
                            solid_bundle.shape_path.set_style(SvgStyle::Fill {
                                fill: SvgFillStyle::RGB {
                                    red: solid_paint.color.red,
                                    green: solid_paint.color.green,
                                    blue: solid_paint.color.blue,
                                },
                            })
                        }
                    }
                }
            }
        }
    }
}
