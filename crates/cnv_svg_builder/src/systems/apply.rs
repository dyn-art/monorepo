use crate::{
    resources::svg_context::SvgContextRes,
    svg::{
        svg_bundle::{style::image_fill::ImageFillStyleVariant, SvgBundleVariant},
        svg_element::{
            attributes::{
                ColorMatrix, SvgAttribute, SvgAttributeValues, SvgHrefAttribute,
                SvgHrefContentType, SvgMeasurementUnit, SvgTransformAttribute,
            },
            styles::{SvgDisplayStyle, SvgStyle},
            SvgElementId, SvgTag,
        },
    },
};
use base64::prelude::*;
use bevy_ecs::{
    entity::Entity,
    query::{Changed, Or, With, Without},
    removal_detection::RemovedComponents,
    system::{ParamSet, Query, Res, ResMut},
};
use bevy_hierarchy::Children;
use bevy_transform::components::Transform;
use dyn_cnv_asset::{asset::ImageAssetContentType, resources::AssetsRes};
use dyn_cnv_bundles::components::{
    mixins::{
        BlendModeMixin, ImageAssetMixin, OpacityMixin, PaintParentMixin, PathMixin, SizeMixin,
        StrokePathMixin, StyleChildrenMixin, VisibilityMixin,
    },
    nodes::{CnvNode, FrameCnvNode},
    paints::{
        CnvPaint, GradientCnvPaint, GradientVariant, ImageCnvPaint, ImageScaleMode, SolidCnvPaint,
    },
    styles::{CnvStyle, DropShadowCnvStyle, FillCnvStyle, StrokeCnvStyle},
};
use dyn_utils::{error::NoneErr, properties::size::Size};
use glam::{Mat3, Vec2};
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
    mut query_set: ParamSet<(
        Query<(Entity, &Children, &mut SvgBundleVariant), (With<CnvNode>, Changed<Children>)>,
        Query<&mut SvgBundleVariant, With<CnvNode>>,
        Query<&mut SvgBundleVariant, With<CnvNode>>,
    )>,
    mut removals: RemovedComponents<Children>,
) {
    let mut changes: SmallVec<[SvgBundleChildrenChange; 2]> = SmallVec::new();

    // Query changes
    {
        let children_query = query_set.p0();
        for (parent_entity, children, node_bundle_variant) in children_query.iter() {
            if let Some(child_node_entities) = node_bundle_variant.get_child_node_entities() {
                // Note: Reverse the order because in a SVG, the top-most element appears last in the children "array",
                // contrary to the DtifCanvas convention where the first element (index = 0) is at the top
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

        // Query changes for special case where parent got leaf (has no children anymore)
        // https://github.com/bevyengine/bevy/issues/6010
        let mut bundle_variant_query = query_set.p2();
        for entity in removals.read() {
            if let Ok(node_bundle_variant) = bundle_variant_query.get_mut(entity) {
                if let Some(child_node_entities) = node_bundle_variant.get_child_node_entities() {
                    // Note: Reverse the order because in a SVG, the top-most element appears last in the children "array",
                    // contrary to the DtifCanvas convention where the first element (index = 0) is at the top
                    let new_child_node_entities: SmallVec<[Entity; 2]> = SmallVec::new();

                    // Identify removed node entities
                    let current_entities_set: HashSet<Entity> =
                        child_node_entities.iter().copied().collect();
                    let new_entities_set: HashSet<Entity> =
                        new_child_node_entities.iter().copied().collect();
                    let removed_entities: SmallVec<[Entity; 2]> = current_entities_set
                        .difference(&new_entities_set)
                        .copied()
                        .collect();

                    changes.push(SvgBundleChildrenChange {
                        parent_entity: entity,
                        added_entities: SmallVec::new(),
                        removed_entities,
                        new_entities_order: new_child_node_entities,
                    });
                }
            }
        }
    }

    // Apply detected changes (Deferred to avoid query conflicts)
    {
        let mut node_bundle_variant_query = query_set.p1();
        for change in changes {
            process_removed_node_children(
                change.parent_entity,
                &change.removed_entities,
                &mut node_bundle_variant_query,
            )
            .expect("Failed to process removed node children!");
            process_added_node_children(
                change.parent_entity,
                &change.added_entities,
                &mut node_bundle_variant_query,
            )
            .expect("Failed to process added node children!");
            reorder_node_children(
                change.parent_entity,
                &change.new_entities_order,
                &mut node_bundle_variant_query,
            )
            .expect("Failed to reorder node children!");
        }
    }
}

fn process_removed_node_children(
    parent_entity: Entity,
    removed_entities: &[Entity],
    node_bundle_variant_query: &mut Query<&mut SvgBundleVariant, With<CnvNode>>,
) -> Result<(), Box<dyn Error>> {
    for entity in removed_entities {
        let [mut node_bundle_variant, mut child_node_bundle_variant] =
            node_bundle_variant_query.get_many_mut([parent_entity, *entity])?;

        if let Some(children_wrapper_element) =
            node_bundle_variant.get_children_wrapper_element_mut()
        {
            children_wrapper_element.remove_child_element(
                child_node_bundle_variant
                    .get_svg_bundle_mut()
                    .get_root_element_mut(),
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
    node_bundle_variant_query: &mut Query<&mut SvgBundleVariant, With<CnvNode>>,
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
    node_bundle_variant_query: &mut Query<&mut SvgBundleVariant, With<CnvNode>>,
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
            With<CnvNode>,
            Without<CnvStyle>,
            Changed<StyleChildrenMixin>,
        ),
    >,
    mut style_bundle_variant_query: Query<
        &mut SvgBundleVariant,
        (With<CnvStyle>, Without<CnvNode>),
    >,
) {
    for (StyleChildrenMixin(styles), mut node_bundle_variant) in query.iter_mut() {
        if let Some(style_entities) = node_bundle_variant.get_style_entities() {
            // Note: Reverse the order because in a SVG, the top-most element appears last in the children "array",
            // contrary to the DtifCanvas convention where the first element (index = 0) is at the top
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
            .expect("Failed to process removed node styles!");
            process_added_node_styles(
                node_bundle_variant.as_mut(),
                &mut added_entities,
                &mut style_bundle_variant_query,
            )
            .expect("Failed to process added node styles!");
            reorder_node_styles(
                node_bundle_variant.as_mut(),
                &new_style_entities,
                &mut style_bundle_variant_query,
            )
            .expect("Failed to reorder node styles!");
        }
    }
}

fn process_removed_node_styles(
    node_bundle_variant: &mut SvgBundleVariant,
    removed_entities: &[Entity],
    style_bundle_variant_query: &mut Query<
        &mut SvgBundleVariant,
        (With<CnvStyle>, Without<CnvNode>),
    >,
) -> Result<(), Box<dyn Error>> {
    for entity in removed_entities {
        let mut style_bundle_variant = style_bundle_variant_query.get_mut(*entity)?;

        if let Some(styles_wrapper_element) = node_bundle_variant.get_styles_wrapper_element_mut() {
            styles_wrapper_element.remove_child_element(
                style_bundle_variant
                    .get_svg_bundle_mut()
                    .get_root_element_mut(),
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
        (With<CnvStyle>, Without<CnvNode>),
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
        (With<CnvStyle>, Without<CnvNode>),
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
    for (VisibilityMixin(visible), mut bundle_variant) in query.iter_mut() {
        let display = if *visible {
            SvgDisplayStyle::Block
        } else {
            SvgDisplayStyle::None
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
        bundle_variant.get_root_element_mut().set_attributes(vec![
            SvgAttribute::Width {
                width: size.width(),
                unit: SvgMeasurementUnit::Pixel,
            },
            SvgAttribute::Height {
                height: size.height(),
                unit: SvgMeasurementUnit::Pixel,
            },
        ]);

        if let Some(click_area_element) = bundle_variant.get_click_area_element_mut() {
            click_area_element.set_attributes(vec![
                SvgAttribute::Width {
                    width: size.width(),
                    unit: SvgMeasurementUnit::Pixel,
                },
                SvgAttribute::Height {
                    height: size.height(),
                    unit: SvgMeasurementUnit::Pixel,
                },
            ]);
        }

        match bundle_variant.as_mut() {
            SvgBundleVariant::ImageFill(bundle) => match bundle.variant {
                ImageFillStyleVariant::Fill | ImageFillStyleVariant::Fit => {
                    bundle.pattern.set_attributes(vec![
                        SvgAttribute::Width {
                            width: size.width(),
                            unit: SvgMeasurementUnit::Pixel,
                        },
                        SvgAttribute::Height {
                            height: size.height(),
                            unit: SvgMeasurementUnit::Pixel,
                        },
                    ]);
                    bundle.image.set_attributes(vec![
                        SvgAttribute::Width {
                            width: size.width(),
                            unit: SvgMeasurementUnit::Pixel,
                        },
                        SvgAttribute::Height {
                            height: size.height(),
                            unit: SvgMeasurementUnit::Pixel,
                        },
                    ]);
                }
                _ => {}
            },
            _ => {}
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
                opacity: opacity.get(),
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

// TODO: Improve like remove also clip path, ..
pub fn apply_clip_content_changes(
    mut query: Query<(&FrameCnvNode, &mut SvgBundleVariant), Changed<FrameCnvNode>>,
) {
    for (FrameCnvNode { clip_content, .. }, mut bundle_variant) in query.iter_mut() {
        match bundle_variant.as_mut() {
            SvgBundleVariant::FrameNode(bundle) => {
                if *clip_content {
                    bundle
                        .children_wrapper_g
                        .set_attribute(SvgAttribute::ClipPath {
                            clip_path: bundle.children_clip_path.get_id(),
                        });
                } else {
                    bundle.children_wrapper_g.remove_attribute("clip-path");
                }
            }
            _ => {}
        }
    }
}

pub fn apply_path_mixin_changes(
    mut query: Query<
        (&PathMixin, &mut SvgBundleVariant),
        (With<CnvNode>, Without<CnvStyle>, Changed<PathMixin>),
    >,
    mut style_bundle_query: Query<
        &mut SvgBundleVariant,
        (
            Or<(With<FillCnvStyle>, With<DropShadowCnvStyle>)>,
            Without<CnvNode>,
        ),
    >,
) {
    for (PathMixin { path, winding_rule }, mut node_bundle_variant) in query.iter_mut() {
        // Apply path to node bundle
        match node_bundle_variant.as_mut() {
            SvgBundleVariant::FrameNode(bundle) => {
                bundle.children_clipped_path.set_attributes(vec![
                    SvgAttribute::D { d: path.into() },
                    SvgAttribute::FillRule {
                        fill_rule: *winding_rule,
                    },
                ])
            }
            _ => {}
        }

        // Apply path to style bundles of node
        if let Some(style_entities) = node_bundle_variant.get_style_entities() {
            for style_entity in style_entities {
                if let Ok(mut style_bundle_variant) = style_bundle_query.get_mut(*style_entity) {
                    match style_bundle_variant.as_mut() {
                        SvgBundleVariant::SolidFill(bundle) => {
                            bundle.shape_path.set_attributes(vec![
                                SvgAttribute::D { d: path.into() },
                                SvgAttribute::FillRule {
                                    fill_rule: *winding_rule,
                                },
                            ])
                        }
                        SvgBundleVariant::GradientFill(bundle) => {
                            bundle.shape_path.set_attributes(vec![
                                SvgAttribute::D { d: path.into() },
                                SvgAttribute::FillRule {
                                    fill_rule: *winding_rule,
                                },
                            ])
                        }
                        SvgBundleVariant::ImageFill(bundle) => {
                            bundle.shape_path.set_attributes(vec![
                                SvgAttribute::D { d: path.into() },
                                SvgAttribute::FillRule {
                                    fill_rule: *winding_rule,
                                },
                            ])
                        }
                        SvgBundleVariant::DropShadowEffect(bundle) => {
                            bundle.shape_path.set_attributes(vec![
                                SvgAttribute::D { d: path.into() },
                                SvgAttribute::FillRule {
                                    fill_rule: *winding_rule,
                                },
                            ])
                        }
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
        (With<StrokeCnvStyle>, Changed<StrokePathMixin>),
    >,
) {
    for (StrokePathMixin { path, winding_rule }, mut bundle_variant) in query.iter_mut() {
        // Apply stroke path to styles
        match bundle_variant.as_mut() {
            SvgBundleVariant::SolidFill(bundle) => bundle.shape_path.set_attributes(vec![
                SvgAttribute::D { d: path.into() },
                SvgAttribute::FillRule {
                    fill_rule: *winding_rule,
                },
            ]),
            SvgBundleVariant::GradientFill(bundle) => bundle.shape_path.set_attributes(vec![
                SvgAttribute::D { d: path.into() },
                SvgAttribute::FillRule {
                    fill_rule: *winding_rule,
                },
            ]),
            SvgBundleVariant::ImageFill(bundle) => bundle.shape_path.set_attributes(vec![
                SvgAttribute::D { d: path.into() },
                SvgAttribute::FillRule {
                    fill_rule: *winding_rule,
                },
            ]),
            _ => {}
        }
    }
}

pub fn apply_solid_paint_changes(
    paint_query: Query<
        (&SolidCnvPaint, &PaintParentMixin),
        (With<CnvPaint>, Changed<SolidCnvPaint>),
    >,
    mut style_query: Query<&mut SvgBundleVariant>,
) {
    for (solid_paint, PaintParentMixin(paint_parent_entities)) in paint_query.iter() {
        for paint_parent_entity in paint_parent_entities {
            if let Ok(mut bundle_variant) = style_query.get_mut(*paint_parent_entity) {
                match bundle_variant.as_mut() {
                    SvgBundleVariant::SolidFill(bundle) => {
                        bundle.shape_path.set_style(SvgStyle::Fill {
                            fill: (&solid_paint.color).into(),
                        })
                    }
                    _ => {}
                }
            }
        }
    }
}

// TODO: This system doesn't account for size changes
// -> Either new system to handle those or integrate into this system
pub fn apply_gradient_paint_changes(
    mut svg_context_res: ResMut<SvgContextRes>,
    paint_query: Query<
        (&GradientCnvPaint, &PaintParentMixin),
        (With<CnvPaint>, Changed<GradientCnvPaint>),
    >,
    mut style_query: Query<(&mut SvgBundleVariant, &SizeMixin)>,
) {
    for (gradient_paint, PaintParentMixin(paint_parent_entities)) in paint_query.iter() {
        for paint_parent_entity in paint_parent_entities {
            if let Ok((mut bundle_variant, SizeMixin(size))) =
                style_query.get_mut(*paint_parent_entity)
            {
                match bundle_variant.as_mut() {
                    SvgBundleVariant::GradientFill(bundle) => {
                        match gradient_paint.variant {
                            GradientVariant::Linear { transform } => {
                                let (start, end) =
                                    extract_start_end_point_from_mat3(size, &transform);
                                bundle.gradient.set_attributes(vec![
                                    SvgAttribute::X1 { x1: start.x },
                                    SvgAttribute::Y1 { y1: start.y },
                                    SvgAttribute::X2 { x2: end.x },
                                    SvgAttribute::Y2 { y2: end.y },
                                ]);

                                // Remove old gradient stop elements
                                bundle.gradient.clear_children();
                                #[cfg(feature = "output_svg_element_changes")]
                                bundle
                                    .gradient_stops
                                    .drain(..)
                                    .for_each(|mut gradient_stop| gradient_stop.destroy());

                                // Add new gradient stop elements
                                for gradient_stop in &gradient_paint.stops {
                                    let mut gradient_stop_element =
                                        svg_context_res.create_element(SvgTag::Stop);
                                    gradient_stop_element.set_attributes(vec![
                                        SvgAttribute::Offset {
                                            offset: gradient_stop.position.get(),
                                        },
                                        SvgAttribute::StopColor {
                                            stop_color: (&gradient_stop.color).into(),
                                        },
                                        SvgAttribute::StopOpacity {
                                            stop_opacity: gradient_stop.opacity.get(),
                                        },
                                    ]);
                                    bundle
                                        .gradient
                                        .append_child_in_bundle_context(&mut gradient_stop_element);
                                    bundle.gradient_stops.push(gradient_stop_element);
                                }
                            }
                            GradientVariant::Radial { transform } => {
                                // TODO
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

/// Helper function to extract the x and y positions of the start and end of the linear gradient
/// (scale is not important here).
///
/// Inspired by:
/// https://github.com/figma-plugin-helper-functions/figma-plugin-helpers/tree/master
fn extract_start_end_point_from_mat3(shape_size: &Size, transform: &Mat3) -> (Vec2, Vec2) {
    let mat3_inv = transform.inverse();
    let start_end_point =
        [Vec2::new(0.0, 0.5), Vec2::new(1.0, 0.5)].map(|p| mat3_inv.transform_point2(p));

    (
        start_end_point[0] * shape_size.to_vec2(),
        start_end_point[1] * shape_size.to_vec2(),
    )
}

// TODO: This system doesn't account for size changes
// -> Either new system to handle those or integrate into this system
pub fn apply_image_paint_changes(
    assets_res: Res<AssetsRes>,
    paint_query: Query<
        (&ImageCnvPaint, &ImageAssetMixin, &PaintParentMixin),
        (With<CnvPaint>, Changed<ImageCnvPaint>),
    >,
    mut style_query: Query<(&mut SvgBundleVariant, &SizeMixin)>,
) {
    for (image_paint, ImageAssetMixin(maybe_image_id), PaintParentMixin(paint_parent_entities)) in
        paint_query.iter()
    {
        if let Some(image) = maybe_image_id.and_then(|id| assets_res.get_image(id)) {
            for paint_parent_entity in paint_parent_entities {
                if let Ok((mut bundle_variant, SizeMixin(size))) =
                    style_query.get_mut(*paint_parent_entity)
                {
                    match bundle_variant.as_mut() {
                        SvgBundleVariant::ImageFill(bundle) => match image_paint.scale_mode {
                            ImageScaleMode::Tile {
                                rotation,
                                scaling_factor,
                            } => {
                                let tile_width = f32::from(image.width) * scaling_factor;
                                let tile_height = f32::from(image.height) * scaling_factor;

                                bundle.pattern.set_attributes(vec![
                                    SvgAttribute::PatternTransform {
                                        pattern_transform: SvgTransformAttribute::Rotate {
                                            rotation: rotation,
                                        },
                                    },
                                    SvgAttribute::Width {
                                        width: tile_width,
                                        unit: SvgMeasurementUnit::Pixel,
                                    },
                                    SvgAttribute::Height {
                                        height: tile_height,
                                        unit: SvgMeasurementUnit::Pixel,
                                    },
                                ]);
                                bundle.image.set_attributes(vec![
                                    SvgAttribute::Width {
                                        width: tile_width,
                                        unit: SvgMeasurementUnit::Pixel,
                                    },
                                    SvgAttribute::Height {
                                        height: tile_height,
                                        unit: SvgMeasurementUnit::Pixel,
                                    },
                                ]);
                            }
                            ImageScaleMode::Crop { transform } => {
                                let (image_width, image_height, image_transform) =
                                    calculate_cropped_image_transform(
                                        size,
                                        (f32::from(image.width), f32::from(image.height)),
                                        &transform,
                                    );

                                bundle.pattern.set_attributes(vec![
                                    SvgAttribute::Width {
                                        width: image_width,
                                        unit: SvgMeasurementUnit::Pixel,
                                    },
                                    SvgAttribute::Height {
                                        height: image_height,
                                        unit: SvgMeasurementUnit::Pixel,
                                    },
                                ]);
                                bundle.image.set_attributes(vec![
                                    SvgAttribute::Transform {
                                        transform: (&image_transform).into(),
                                    },
                                    SvgAttribute::Width {
                                        width: image_width,
                                        unit: SvgMeasurementUnit::Pixel,
                                    },
                                    SvgAttribute::Height {
                                        height: image_height,
                                        unit: SvgMeasurementUnit::Pixel,
                                    },
                                ]);
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
            }
        }
    }
}

// TODO: Improve
fn calculate_cropped_image_transform(
    parent_size: &Size,
    image_size: (f32, f32),
    transform: &Mat3,
) -> (f32, f32, Mat3) {
    let (parent_width, parent_height) = parent_size.to_tuple();
    let (image_width, image_height) = image_size;

    // Calculate aspect ratios for container and image
    let container_ratio = parent_width / parent_height;
    let image_ratio = image_width / image_height;

    // Determine new image dimensions based on aspect ratio cnvarison
    let (adjusted_image_width, adjusted_image_height) = if image_ratio > container_ratio {
        (parent_height * image_ratio, parent_height)
    } else {
        (parent_width, parent_width / image_ratio)
    };

    // Calculate scale adjustment ratios
    let x_ratio = parent_width / adjusted_image_width;
    let y_ratio = parent_height / adjusted_image_height;

    // Extract scale components from the matrix and adjust them
    let scale_x = transform.x_axis.x;
    let scale_y = transform.y_axis.y;
    let adjusted_scale_x = (1.0 / scale_x) * x_ratio;
    let adjusted_scale_y = (1.0 / scale_y) * y_ratio;

    // Calculate adjusted translation.
    let tx = -adjusted_image_width * transform.z_axis.x * adjusted_scale_x;
    let ty = -adjusted_image_height * transform.z_axis.y * adjusted_scale_y;

    // Construct the adjusted transformation matrix
    let adjusted_transform = Mat3::from_scale_angle_translation(
        Vec2::new(adjusted_scale_x, adjusted_scale_y),
        0.0,
        Vec2::new(tx, ty),
    );

    return (
        adjusted_image_width,
        adjusted_image_height,
        adjusted_transform,
    );
}

pub fn apply_image_asset_mixin_changes(
    assets_res: Res<AssetsRes>,
    paint_query: Query<
        (&ImageAssetMixin, &PaintParentMixin),
        (With<CnvPaint>, Changed<ImageAssetMixin>),
    >,
    mut style_query: Query<&mut SvgBundleVariant>,
) {
    for (ImageAssetMixin(maybe_image_id), PaintParentMixin(paint_parent_entities)) in
        paint_query.iter()
    {
        if let Some(image) = maybe_image_id.and_then(|id| assets_res.get_image(id)) {
            for paint_parent_entity in paint_parent_entities {
                if let Ok(mut bundle_variant) = style_query.get_mut(*paint_parent_entity) {
                    match bundle_variant.as_mut() {
                        SvgBundleVariant::ImageFill(bundle) => {
                            bundle.image.set_attribute(SvgAttribute::Href {
                                href: SvgHrefAttribute::Base64 {
                                    content: BASE64_STANDARD.encode(image.content.clone()),
                                    content_type: match image.content_type {
                                        ImageAssetContentType::Png => SvgHrefContentType::Png,
                                        ImageAssetContentType::Jpeg => SvgHrefContentType::Jpeg,
                                        ImageAssetContentType::Svg => SvgHrefContentType::Svg,
                                        _ => return,
                                    },
                                },
                            });
                        }
                        _ => {}
                    }
                }
            }
        } else {
            // TODO: Show placeholder image or so?
            log::warn!("Couldn't find image at {:?}", maybe_image_id);
        }
    }
}

pub fn apply_drop_shadow_changes(
    mut query: Query<
        (&DropShadowCnvStyle, &mut SvgBundleVariant, &SizeMixin),
        (With<DropShadowCnvStyle>, Changed<DropShadowCnvStyle>),
    >,
) {
    for (
        DropShadowCnvStyle {
            color,
            position,
            spread,
            blur,
        },
        mut bundle_variant,
        SizeMixin(size),
    ) in query.iter_mut()
    {
        match bundle_variant.as_mut() {
            SvgBundleVariant::DropShadowEffect(bundle) => {
                // Those values are based on penpot.app's shadow factors
                // Note: Didn't match them 1 to 1 but close enough for now
                let base_pos = Vec2::new(-15.0, -15.0);
                let base_size = Vec2::new(30.0, 30.0);
                let blur_pos_factor = blur.to_pt() * -3.0;
                let blur_size_factor = blur.to_pt() * 6.0;
                let spread_pos_factor = spread.to_pt() * -3.0;
                let spread_size_factor = spread.to_pt() * 6.0;
                let position_size_factor = *position * 3.0;

                let new_pos = (base_pos + blur_pos_factor + spread_pos_factor) / size.to_vec2();
                let new_size =
                    (base_size + blur_size_factor + spread_size_factor + position_size_factor)
                        / size.to_vec2()
                        + 1.0;

                bundle.filter.set_attributes(vec![
                    SvgAttribute::X {
                        x: new_pos.x * 100.0,
                        unit: SvgMeasurementUnit::Percent,
                    },
                    SvgAttribute::Y {
                        y: new_pos.y * 100.0,
                        unit: SvgMeasurementUnit::Percent,
                    },
                    SvgAttribute::Width {
                        width: new_size.x * 100.0,
                        unit: SvgMeasurementUnit::Percent,
                    },
                    SvgAttribute::Height {
                        height: new_size.y * 100.0,
                        unit: SvgMeasurementUnit::Percent,
                    },
                ]);
                bundle.fe_color_matrix.set_attribute(SvgAttribute::Values {
                    values: SvgAttributeValues::ColorMatrix(ColorMatrix::from_rgba(
                        color.get_red(),
                        color.get_green(),
                        color.get_blue(),
                        1.0, // Opacity is applied on wrapping group tag
                    )),
                });
                bundle.fe_offset.set_attributes(vec![
                    SvgAttribute::DX { dx: position.x },
                    SvgAttribute::DY { dy: position.y },
                ]);
                bundle
                    .source_alpha_fe_morphology
                    .set_attribute(SvgAttribute::Radius {
                        radius: spread.to_pt(),
                    });
                bundle
                    .fe_gaussian_blur
                    .set_attributes(vec![SvgAttribute::StdDeviation {
                        std_deviation: blur.to_pt() / 2.0,
                    }]);
            }
            _ => {}
        }
    }
}
