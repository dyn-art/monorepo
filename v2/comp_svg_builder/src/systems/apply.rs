use crate::svg::{
    svg_bundle::{FillSvgBundle, NodeSvgBundle, NodeSvgBundleMixin},
    svg_element::{
        attributes::{SvgAttribute, SvgMeasurementUnit},
        element_changes::{SvgElementChange, SvgElementReorderedChange},
        styles::{SvgDisplayStyle, SvgStyle},
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
    mixins::{BlendModeMixin, OpacityMixin, PaintParentMixin, SizeMixin, VisibilityMixin},
    nodes::CompNode,
    paints::{CompPaint, SolidCompPaint},
};
use smallvec::SmallVec;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct SvgBundleChildrenModification {
    pub parent_entity: Entity,
    pub entities: SmallVec<[Entity; 2]>,
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
            let node_children = match bundle.get_node_children() {
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
                entities: node_children.iter().copied().collect(),
            });
        }
    }

    // Apply modifications
    {
        let mut node_query = queries.p1();
        for modification in modifications {
            process_removed_nodes(
                modification.parent_entity,
                &modification.removed_entities,
                &mut node_query,
            );
            process_added_nodes(
                modification.parent_entity,
                &modification.added_entities,
                &mut node_query,
            );
            reorder_nodes(
                modification.parent_entity,
                &modification.entities,
                &mut node_query,
            );
        }
    }
}

fn process_removed_nodes(
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

fn process_added_nodes(
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

fn reorder_nodes(
    parent_entity: Entity,
    entities: &[Entity],
    node_query: &mut Query<&mut NodeSvgBundleMixin>,
) -> Option<()> {
    let order_map = entities
        .iter()
        .enumerate()
        .map(|(index, entity)| (*entity, index))
        .collect::<HashMap<Entity, usize>>();

    #[cfg(feature = "output_svg_element_changes")]
    let bundle_children = {
        let NodeSvgBundleMixin(bundle) = match node_query.get(parent_entity) {
            Ok(bundle_mixin) => bundle_mixin,
            Err(_) => return Some(()),
        };
        let entities_with_element_id: SmallVec<[(Entity, SvgElementId); 2]> = bundle
            .get_node_children()?
            .iter()
            .filter_map(|entity| match node_query.get(*entity) {
                Ok(NodeSvgBundleMixin(bundle)) => {
                    Some((*entity, bundle.get_svg_bundle().get_root_element().get_id()))
                }
                Err(_) => None,
            })
            .collect();
        entities_with_element_id
    };

    let mut bundle_mixin = match node_query.get_mut(parent_entity) {
        Ok(bundle_mixin) => bundle_mixin,
        Err(_) => return Some(()),
    };
    let NodeSvgBundleMixin(bundle) = bundle_mixin.as_mut();
    let bundle_children_mut = bundle.get_node_children_mut()?;

    // Track the original positions of the node children
    #[cfg(feature = "output_svg_element_changes")]
    let original_positions = bundle_children
        .iter()
        .map(|(_, id)| *id)
        .collect::<Vec<_>>();

    // Sort `bundle_children` based on the order defined in `order_map`
    bundle_children_mut.sort_by_key(|entity| *order_map.get(entity).unwrap_or(&usize::MAX));

    #[cfg(feature = "output_svg_element_changes")]
    {
        // Determine the new positions after sorting
        let new_positions = bundle_children
            .iter()
            .map(|(_, id)| *id)
            .collect::<Vec<_>>();

        // Emit SvgElementReorderedChange events for node children that have been moved
        for (new_index, &element_id) in new_positions.iter().enumerate() {
            let original_index = original_positions
                .iter()
                .position(|&e| e == element_id)
                .unwrap_or(new_index);

            // If the fill has been moved
            if original_index != new_index {
                let new_parent_id = bundle.get_fill_wrapper_element_mut()?.get_id();

                // Determine insert_before_id based on the next sibling in the new order
                let insert_before_id = if new_index + 1 < new_positions.len() {
                    // There is a next sibling, get its ID
                    Some(new_positions[new_index + 1])
                } else {
                    // No next sibling, append at the end
                    None
                };

                bundle.get_fill_wrapper_element_mut()?.register_change(
                    SvgElementChange::ElementReordered(SvgElementReorderedChange {
                        element_id,
                        new_parent_id,
                        insert_before_id,
                    }),
                );
            }
        }
    }

    Some(())
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
            NodeSvgBundle::Frame(bundle) => &mut bundle.root,
            NodeSvgBundle::Shape(bundle) => &mut bundle.root,
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
            NodeSvgBundle::Shape(bundle) => {
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

        // Apply dimension change to fills
        let fills = match bundle.get_fills_mut() {
            Some(fills) => fills,
            None => return,
        };
        for fill in fills {
            match fill {
                FillSvgBundle::Solid(fill) => {
                    fill.paint_rect.set_attributes(vec![
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
}

pub fn apply_transform_changes(
    mut query: Query<(&Transform, &mut NodeSvgBundleMixin), (With<CompNode>, Changed<Transform>)>,
) {
    for (transform, mut bundle_mixin) in query.iter_mut() {
        let NodeSvgBundleMixin(bundle) = bundle_mixin.as_mut();
        let element = match bundle {
            NodeSvgBundle::Frame(bundle) => &mut bundle.root,
            NodeSvgBundle::Shape(bundle) => &mut bundle.root,
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
            NodeSvgBundle::Frame(bundle) => &mut bundle.root,
            NodeSvgBundle::Shape(bundle) => &mut bundle.root,
        };

        element.set_attribute(SvgAttribute::Opacity {
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
            NodeSvgBundle::Frame(bundle) => &mut bundle.root,
            NodeSvgBundle::Shape(bundle) => &mut bundle.root,
        };

        element.set_style(SvgStyle::BlendMode {
            blend_mode: blend_mode.into(),
        });
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
                let bundle_fills = match bundle {
                    NodeSvgBundle::Frame(bundle) => &mut bundle.fills,
                    NodeSvgBundle::Shape(bundle) => &mut bundle.fills,
                    _ => return,
                };

                if let Some(fill) = bundle_fills
                    .iter_mut()
                    .find(|fill| *fill.get_paint_entity() == paint_entity)
                {
                    match fill {
                        FillSvgBundle::Solid(fill) => {
                            fill.paint_rect.set_attribute(SvgAttribute::Fill {
                                fill: format!(
                                    "rgb({}, {}, {})",
                                    solid_paint.color.red,
                                    solid_paint.color.green,
                                    solid_paint.color.blue
                                ),
                            })
                        }
                    }
                }
            }
        }
    }
}
