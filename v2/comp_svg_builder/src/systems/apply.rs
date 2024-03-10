use crate::svg::{
    svg_bundle::SvgBundleVariant,
    svg_element::{
        attributes::{SvgAttribute, SvgMeasurementUnit},
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
use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

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
                let new_child_node_entities: SmallVec<[Entity; 2]> =
                    children.iter().copied().rev().collect();

                // Identify removed and newly added node entities
                let current_entities_set: HashSet<Entity> =
                    child_node_entities.iter().copied().collect();
                let new_entities_set: HashSet<Entity> =
                    new_child_node_entities.iter().copied().collect();
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
                    new_entities_order: new_child_node_entities,
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

        if let Some(child_node_entities) = node_bundle_variant.get_child_node_entities_mut() {
            child_node_entities.retain(|e| *e != *entity);
        }
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

        if let Some(child_node_entities) = node_bundle_variant.get_child_node_entities_mut() {
            child_node_entities.push(*entity);
        }
    }

    return Ok(());
}

fn reorder_node_children(
    parent_entity: Entity,
    new_entities_order: &[Entity],
    node_bundle_variant_query: &mut Query<&mut SvgBundleVariant, With<CompNode>>,
) -> Result<(), Box<dyn Error>> {
    // Create a new order mapping
    let new_order_mapping: HashMap<SvgElementId, usize> = new_entities_order
        .iter()
        .enumerate()
        .filter_map(|(index, entity)| {
            node_bundle_variant_query
                .get_mut(*entity)
                .ok()
                .map(|svg_bundle_variant| {
                    let id = svg_bundle_variant
                        .get_svg_bundle()
                        .get_root_element()
                        .get_id();
                    (id, index)
                })
        })
        .collect();

    // Sort children based on their new order, placing any unknown elements at the end
    let mut node_bundle_variant = node_bundle_variant_query.get_mut(parent_entity)?;
    let children_wrapper_element = node_bundle_variant
        .get_children_wrapper_element_mut()
        .ok_or(NoneErr::new("Failed to retrieve children wrapper element!"))?;
    children_wrapper_element.reorder_children_mut(|children| {
        children.sort_by(|a, b| {
            let index_a = new_order_mapping.get(&a.id).unwrap_or(&usize::MAX);
            let index_b = new_order_mapping.get(&b.id).unwrap_or(&usize::MAX);
            index_a.cmp(index_b)
        });
    });

    Ok(())
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
            let new_style_entities: SmallVec<[Entity; 2]> = styles.iter().copied().rev().collect();

            // Identify removed and newly added style entities
            let current_entities_set: HashSet<Entity> = style_entities.iter().copied().collect();
            let new_entities_set: HashSet<Entity> = new_style_entities.iter().copied().collect();
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
                &new_style_entities,
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

        if let Some(style_entities) = node_bundle_variant.get_style_entities_mut() {
            style_entities.retain(|e| *e != *entity);
        }
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

        if let Some(style_entities) = node_bundle_variant.get_style_entities_mut() {
            style_entities.push(*entity);
        }
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
    // Create a new order mapping
    let new_order_mapping: HashMap<SvgElementId, usize> = new_entities_order
        .iter()
        .enumerate()
        .filter_map(|(index, entity)| {
            style_bundle_variant_query
                .get_mut(*entity)
                .ok()
                .map(|svg_bundle_variant| {
                    let id = svg_bundle_variant
                        .get_svg_bundle()
                        .get_root_element()
                        .get_id();
                    (id, index)
                })
        })
        .collect();

    // Sort children based on their new order, placing any unknown elements at the end
    let styles_wrapper_element = node_bundle_variant
        .get_styles_wrapper_element_mut()
        .ok_or(NoneErr::new("Failed to retrieve styles wrapper element!"))?;
    styles_wrapper_element.reorder_children_mut(|children| {
        children.sort_by(|a, b| {
            let index_a = new_order_mapping.get(&a.id).unwrap_or(&usize::MAX);
            let index_b = new_order_mapping.get(&b.id).unwrap_or(&usize::MAX);
            index_a.cmp(index_b)
        });
    });

    Ok(())
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
