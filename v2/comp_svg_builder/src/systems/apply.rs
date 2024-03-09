use crate::svg::{
    svg_bundle::SvgBundleVariant,
    svg_element::{
        attributes::{SvgAttribute, SvgMeasurementUnit},
        element_changes::{SvgElementChange, SvgElementReorderedChange},
        styles::{SvgDisplayStyle, SvgFillStyle, SvgStyle},
        SvgElementId,
    },
};
use bevy_ecs::{
    entity::Entity,
    query::{Changed, With, Without},
    system::{ParamSet, Query},
};
use bevy_hierarchy::Children;
use bevy_transform::components::Transform;
use dyn_comp_common::{
    common::Visibility,
    error::NoneErr,
    mixins::{
        BlendModeMixin, OpacityMixin, PaintParentMixin, PathMixin, SizeMixin, StrokePathMixin,
        StyleChildrenMixin, VisibilityMixin,
    },
    nodes::CompNode,
    paints::{CompPaint, SolidCompPaint},
    styles::{CompStyle, FillCompStyle, StrokeCompStyle},
};
use smallvec::SmallVec;
use std::{collections::HashSet, error::Error};

#[derive(Debug, Clone)]
pub struct SvgBundleChildrenChange {
    pub parent_entity: Entity,
    pub new_entities_order: SmallVec<[Entity; 2]>,
    pub added_entities: SmallVec<[Entity; 2]>,
    pub removed_entities: SmallVec<[Entity; 2]>,
}

pub fn apply_node_children_changes(
    // https://bevyengine.org/learn/errors/
    mut queries: ParamSet<(
        Query<(Entity, &Children, &mut SvgBundleVariant), (With<CompNode>, Changed<Children>)>,
        Query<&mut SvgBundleVariant, With<CompNode>>,
    )>,
) {
    let mut changes: SmallVec<[SvgBundleChildrenChange; 2]> = SmallVec::new();

    // Query changes
    {
        let children_query = queries.p0();
        for (parent_entity, children, node_bundle_variant) in children_query.iter() {
            if let Some(child_node_entities) = node_bundle_variant.get_child_node_entities() {
                // Identify removed and newly added node entities
                let current_entities_set: HashSet<Entity> =
                    child_node_entities.iter().copied().collect();
                let new_entities_set: HashSet<Entity> = children.iter().copied().collect();
                let removed_entities: SmallVec<[Entity; 2]> = current_entities_set
                    .difference(&new_entities_set)
                    .copied()
                    .collect();
                let added_entities: SmallVec<[Entity; 2]> = new_entities_set
                    .difference(&current_entities_set)
                    .copied()
                    .collect();

                changes.push(SvgBundleChildrenChange {
                    parent_entity,
                    added_entities,
                    removed_entities,
                    new_entities_order: children.iter().copied().collect(),
                });
            }
        }
    }

    // Apply detected changes (Deferred to avoid query conflicts)
    {
        let mut node_bundle_variant_query = queries.p1();
        for change in changes {
            process_removed_node_children(
                change.parent_entity,
                &change.removed_entities,
                &mut node_bundle_variant_query,
            )
            .unwrap();
            process_added_node_children(
                change.parent_entity,
                &change.added_entities,
                &mut node_bundle_variant_query,
            )
            .unwrap();
            reorder_node_children(
                change.parent_entity,
                &change.new_entities_order,
                &mut node_bundle_variant_query,
            )
            .unwrap();
        }
    }
}

fn process_removed_node_children(
    parent_entity: Entity,
    removed_entities: &[Entity],
    node_bundle_variant_query: &mut Query<&mut SvgBundleVariant, With<CompNode>>,
) -> Result<(), Box<dyn Error>> {
    for entity in removed_entities {
        let [mut node_bundle_variant, child_node_bundle_variant] =
            node_bundle_variant_query.get_many_mut([parent_entity, *entity])?;

        if let Some(children_wrapper_element) =
            node_bundle_variant.get_children_wrapper_element_mut()
        {
            children_wrapper_element.remove_child(
                child_node_bundle_variant
                    .get_svg_bundle()
                    .get_root_element()
                    .get_id(),
            );
        }

        // Note: child_node_entities are synced in "reorder_node_children()"
    }

    return Ok(());
}

fn process_added_node_children(
    parent_entity: Entity,
    added_entities: &[Entity],
    node_bundle_variant_query: &mut Query<&mut SvgBundleVariant, With<CompNode>>,
) -> Result<(), Box<dyn Error>> {
    for entity in added_entities {
        let [mut node_bundle_variant, mut child_node_bundle_variant] =
            node_bundle_variant_query.get_many_mut([parent_entity, *entity])?;

        if let Some(children_wrapper_element) =
            node_bundle_variant.get_children_wrapper_element_mut()
        {
            children_wrapper_element.append_child_in_world_context(
                *entity,
                child_node_bundle_variant
                    .get_svg_bundle_mut()
                    .get_root_element_mut(),
            );
        }

        // Note: child_node_entities are synced in "reorder_node_children()"
    }

    return Ok(());
}

fn reorder_node_children(
    parent_entity: Entity,
    new_entities_order: &[Entity],
    node_bundle_variant_query: &mut Query<&mut SvgBundleVariant, With<CompNode>>,
) -> Result<(), Box<dyn Error>> {
    // Track the original positions of the node children
    #[cfg(feature = "output_svg_element_changes")]
    let original_positions: SmallVec<[(Entity, SvgElementId); 2]> = {
        let child_node_entities = node_bundle_variant_query
            .get(parent_entity)?
            .get_child_node_entities()
            .ok_or(NoneErr::new("Failed to retrieve node children!"))?;
        child_node_entities
            .iter()
            .filter_map(|entity| {
                Some((
                    *entity,
                    node_bundle_variant_query
                        .get(*entity)
                        .ok()?
                        .get_svg_bundle()
                        .get_root_element()
                        .get_id(),
                ))
            })
            .collect()
    };

    let mut node_bundle_variant = node_bundle_variant_query.get_mut(parent_entity)?;
    let child_node_entities = node_bundle_variant
        .get_child_node_entities_mut()
        .ok_or(NoneErr::new("Failed to retrieve node children!"))?;

    // Apply new order
    child_node_entities.clear();
    child_node_entities.extend(new_entities_order.iter().copied());

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

            // If the child has been moved
            if original_index != new_index {
                let children_wrapper_element = node_bundle_variant
                    .get_children_wrapper_element_mut()
                    .ok_or(NoneErr::new("Failed to retrieve children wrapper element!"))?;
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

    return Ok(());
}

pub fn apply_node_styles_changes(
    mut query: Query<
        (&StyleChildrenMixin, &mut SvgBundleVariant),
        (
            With<CompNode>,
            Without<CompStyle>,
            Changed<StyleChildrenMixin>,
        ),
    >,
    mut style_bundle_variant_query: Query<
        &mut SvgBundleVariant,
        (With<CompStyle>, Without<CompNode>),
    >,
) {
    for (StyleChildrenMixin(styles), mut node_bundle_variant) in query.iter_mut() {
        if let Some(style_entities) = node_bundle_variant.get_style_entities() {
            // Identify removed and newly added style entities
            let current_entities_set: HashSet<Entity> = style_entities.iter().copied().collect();
            let new_entities_set: HashSet<Entity> = styles.iter().copied().collect();
            let mut removed_entities: SmallVec<[Entity; 2]> = current_entities_set
                .difference(&new_entities_set)
                .copied()
                .collect();
            let mut added_entities: SmallVec<[Entity; 2]> = new_entities_set
                .difference(&current_entities_set)
                .copied()
                .collect();

            // Apply detected changes
            process_removed_node_styles(
                node_bundle_variant.as_mut(),
                &mut removed_entities,
                &mut style_bundle_variant_query,
            )
            .unwrap();
            process_added_node_styles(
                node_bundle_variant.as_mut(),
                &mut added_entities,
                &mut style_bundle_variant_query,
            )
            .unwrap();
            reorder_node_styles(
                node_bundle_variant.as_mut(),
                &styles,
                &mut style_bundle_variant_query,
            )
            .unwrap();
        }
    }
}

fn process_removed_node_styles(
    node_bundle_variant: &mut SvgBundleVariant,
    removed_entities: &[Entity],
    style_bundle_variant_query: &mut Query<
        &mut SvgBundleVariant,
        (With<CompStyle>, Without<CompNode>),
    >,
) -> Result<(), Box<dyn Error>> {
    for entity in removed_entities {
        let style_bundle_variant = style_bundle_variant_query.get_mut(*entity)?;

        if let Some(styles_wrapper_element) = node_bundle_variant.get_styles_wrapper_element_mut() {
            styles_wrapper_element.remove_child(
                style_bundle_variant
                    .get_svg_bundle()
                    .get_root_element()
                    .get_id(),
            );
        }

        // Note: style_entities are synced in "reorder_node_styles()"
    }

    return Ok(());
}

fn process_added_node_styles(
    node_bundle_variant: &mut SvgBundleVariant,
    added_entities: &[Entity],
    style_bundle_variant_query: &mut Query<
        &mut SvgBundleVariant,
        (With<CompStyle>, Without<CompNode>),
    >,
) -> Result<(), Box<dyn Error>> {
    for entity in added_entities {
        let mut style_bundle_variant = style_bundle_variant_query.get_mut(*entity)?;

        if let Some(styles_wrapper_element) = node_bundle_variant.get_styles_wrapper_element_mut() {
            styles_wrapper_element.append_child_in_world_context(
                *entity,
                style_bundle_variant
                    .get_svg_bundle_mut()
                    .get_root_element_mut(),
            );
        }

        // Note: style_entities are synced in "reorder_node_styles()"
    }

    return Ok(());
}

fn reorder_node_styles(
    node_bundle_variant: &mut SvgBundleVariant,
    new_entities_order: &[Entity],
    style_bundle_variant_query: &mut Query<
        &mut SvgBundleVariant,
        (With<CompStyle>, Without<CompNode>),
    >,
) -> Result<(), Box<dyn Error>> {
    // Track the original positions of the node styles
    #[cfg(feature = "output_svg_element_changes")]
    let original_positions: SmallVec<[(Entity, SvgElementId); 2]> = {
        let style_entities = node_bundle_variant
            .get_style_entities()
            .ok_or(NoneErr::new("Failed to retrieve node styles!"))?;
        style_entities
            .iter()
            .filter_map(|entity| {
                Some((
                    *entity,
                    style_bundle_variant_query
                        .get(*entity)
                        .ok()?
                        .get_svg_bundle()
                        .get_root_element()
                        .get_id(),
                ))
            })
            .collect()
    };

    let style_entities = node_bundle_variant
        .get_style_entities_mut()
        .ok_or(NoneErr::new("Failed to retrieve node styles!"))?;

    // Apply new order
    style_entities.clear();
    style_entities.extend(new_entities_order.iter().copied());

    #[cfg(feature = "output_svg_element_changes")]
    {
        // Determine the new positions after sorting
        let new_positions = original_positions
            .iter()
            .map(|(_, id)| *id)
            .collect::<Vec<_>>();

        // Emit SvgElementReorderedChange events for node styles that have been moved
        for (new_index, &element_id) in new_positions.iter().enumerate() {
            let original_index = original_positions
                .iter()
                .position(|(_, e)| *e == element_id)
                .unwrap_or(new_index);

            // If the style has been moved
            if original_index != new_index {
                let styles_wrapper_element =
                    node_bundle_variant
                        .get_styles_wrapper_element_mut()
                        .ok_or(NoneErr::new("Failed to retrieve styles wrapper element!"))?;
                let new_parent_id = styles_wrapper_element.get_id();

                // Determine insert_before_id based on the next sibling in the new order
                let insert_before_id = if new_index + 1 < new_positions.len() {
                    // There is a next sibling, get its ID
                    Some(new_positions[new_index + 1])
                } else {
                    // No next sibling, append at the end
                    None
                };

                styles_wrapper_element.register_change(SvgElementChange::ElementReordered(
                    SvgElementReorderedChange {
                        element_id,
                        new_parent_id,
                        insert_before_id,
                    },
                ));
            }
        }
    }

    return Ok(());
}

pub fn apply_visibility_mixin_changes(
    mut query: Query<(&VisibilityMixin, &mut SvgBundleVariant), Changed<VisibilityMixin>>,
) {
    for (VisibilityMixin(visibility), mut bundle_variant) in query.iter_mut() {
        let display = match visibility {
            Visibility::Visible => SvgDisplayStyle::Block,
            Visibility::Hidden => SvgDisplayStyle::None,
        };
        bundle_variant
            .get_root_element_mut()
            .set_style(SvgStyle::Display { display });
    }
}

pub fn apply_size_mixin_changes(
    mut query: Query<(&SizeMixin, &mut SvgBundleVariant), Changed<SizeMixin>>,
) {
    for (SizeMixin(size), mut bundle_variant) in query.iter_mut() {
        let [width, height] = size.0.to_array();

        bundle_variant.get_root_element_mut().set_attributes(vec![
            SvgAttribute::Width {
                width,
                unit: SvgMeasurementUnit::Pixel,
            },
            SvgAttribute::Height {
                height,
                unit: SvgMeasurementUnit::Pixel,
            },
        ]);
        if let Some(click_area_element) = bundle_variant.get_click_area_element_mut() {
            click_area_element.set_attributes(vec![
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

pub fn apply_transform_changes(
    mut query: Query<(&Transform, &mut SvgBundleVariant), Changed<Transform>>,
) {
    for (transform, mut bundle_variant) in query.iter_mut() {
        bundle_variant
            .get_root_element_mut()
            .set_attribute(SvgAttribute::Transform {
                transform: transform.into(),
            });
    }
}

pub fn apply_opacity_mixin_changes(
    mut query: Query<(&OpacityMixin, &mut SvgBundleVariant), Changed<OpacityMixin>>,
) {
    for (OpacityMixin(opacity), mut bundle_variant) in query.iter_mut() {
        bundle_variant
            .get_root_element_mut()
            .set_style(SvgStyle::Opacity {
                opacity: opacity.0.get(),
            });
    }
}

pub fn apply_blend_mode_mixin_changes(
    mut query: Query<(&BlendModeMixin, &mut SvgBundleVariant), Changed<BlendModeMixin>>,
) {
    for (BlendModeMixin(blend_mode), mut bundle_variant) in query.iter_mut() {
        bundle_variant
            .get_root_element_mut()
            .set_style(SvgStyle::BlendMode {
                blend_mode: blend_mode.into(),
            });
    }
}

pub fn apply_path_mixin_changes(
    mut query: Query<
        (&PathMixin, &mut SvgBundleVariant),
        (With<CompNode>, Without<CompStyle>, Changed<PathMixin>),
    >,
    mut style_bundle_query: Query<
        &mut SvgBundleVariant,
        (With<CompStyle>, Without<CompNode>, With<FillCompStyle>),
    >,
) {
    for (PathMixin(path), mut node_bundle_variant) in query.iter_mut() {
        // Apply path to node bundle
        match node_bundle_variant.as_mut() {
            SvgBundleVariant::Frame(bundle) => bundle
                .children_clipped_path
                .set_attribute(SvgAttribute::D { d: path.into() }),
            _ => {}
        }

        // Apply path to style bundles of node
        if let Some(style_entities) = node_bundle_variant.get_style_entities() {
            for style_entity in style_entities {
                if let Ok(mut style_bundle_variant) = style_bundle_query.get_mut(*style_entity) {
                    match style_bundle_variant.as_mut() {
                        SvgBundleVariant::Solid(bundle) => bundle
                            .shape_path
                            .set_attribute(SvgAttribute::D { d: path.into() }),
                        _ => {}
                    }
                }
            }
        }
    }
}

pub fn apply_stroke_path_mixin_changes(
    mut query: Query<
        (&StrokePathMixin, &mut SvgBundleVariant),
        (With<StrokeCompStyle>, Changed<StrokePathMixin>),
    >,
) {
    for (StrokePathMixin(stroke_path), mut bundle_variant) in query.iter_mut() {
        // Apply stroke path to styles
        match bundle_variant.as_mut() {
            SvgBundleVariant::Solid(bundle) => bundle.shape_path.set_attribute(SvgAttribute::D {
                d: stroke_path.into(),
            }),
            _ => {}
        }
    }
}

pub fn apply_solid_paint_changes(
    query: Query<(&SolidCompPaint, &PaintParentMixin), (With<CompPaint>, Changed<SolidCompPaint>)>,
    mut bundle_query: Query<&mut SvgBundleVariant>,
) {
    for (solid_paint, PaintParentMixin(parent_entities)) in query.iter() {
        for parent_entity in parent_entities {
            if let Ok(mut bundle_variant) = bundle_query.get_mut(*parent_entity) {
                match bundle_variant.as_mut() {
                    SvgBundleVariant::Solid(bundle) => {
                        bundle.shape_path.set_style(SvgStyle::Fill {
                            fill: SvgFillStyle::RGB {
                                red: solid_paint.color.red,
                                green: solid_paint.color.green,
                                blue: solid_paint.color.blue,
                            },
                        })
                    }
                    _ => {}
                }
            }
        }
    }
}
