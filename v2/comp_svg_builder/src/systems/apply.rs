use crate::svg::{
    svg_bundle::{
        node::{NodeSvgBundle, NodeSvgBundleMixin},
        style::{StyleSvgBundle, StyleSvgBundleMixin},
    },
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
use dyn_comp_common::{
    common::Visibility,
    error::NoneErr,
    mixins::{
        BlendModeMixin, OpacityMixin, PaintParentMixin, PathMixin, SizeMixin, StrokePathMixin,
        StylesMixin, VisibilityMixin,
    },
    nodes::CompNode,
    paints::{CompPaint, SolidCompPaint},
    styles::{FillCompStyle, StrokeCompStyle},
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
        Query<(Entity, &Children, &mut NodeSvgBundleMixin), (With<CompNode>, Changed<Children>)>,
        Query<&mut NodeSvgBundleMixin>,
    )>,
) {
    let mut changes: SmallVec<[SvgBundleChildrenChange; 2]> = SmallVec::new();

    // Query changes
    {
        let children_query = queries.p0();
        for (entity, children, NodeSvgBundleMixin(bundle)) in children_query.iter() {
            let node_children = match bundle.get_child_nodes() {
                Some(node_children) => node_children,
                None => return,
            };

            // Identify removed and newly added node entities
            let current_node_children_set: HashSet<Entity> =
                node_children.iter().copied().collect();
            let new_node_children_set: HashSet<Entity> = children.iter().copied().collect();
            let removed_node_children: SmallVec<[Entity; 2]> = current_node_children_set
                .difference(&new_node_children_set)
                .copied()
                .collect();
            let added_node_children: SmallVec<[Entity; 2]> = new_node_children_set
                .difference(&current_node_children_set)
                .copied()
                .collect();

            changes.push(SvgBundleChildrenChange {
                parent_entity: entity,
                added_entities: added_node_children,
                removed_entities: removed_node_children,
                new_entities_order: children.iter().copied().collect(),
            });
        }
    }

    // Apply detected changes (Deferred to avoid query conflicts)
    {
        let mut node_bundle_query = queries.p1();
        for change in changes {
            process_removed_node_children(
                change.parent_entity,
                &change.removed_entities,
                &mut node_bundle_query,
            )
            .unwrap();
            process_added_node_children(
                change.parent_entity,
                &change.added_entities,
                &mut node_bundle_query,
            )
            .unwrap();
            reorder_node_children(
                change.parent_entity,
                &change.new_entities_order,
                &mut node_bundle_query,
            )
            .unwrap();
        }
    }
}

fn process_removed_node_children(
    parent_entity: Entity,
    removed_entities: &[Entity],
    node_bundle_query: &mut Query<&mut NodeSvgBundleMixin>,
) -> Result<(), Box<dyn Error>> {
    for entity in removed_entities {
        let [mut node_bundle_mixin, child_bundle_mixin] =
            node_bundle_query.get_many_mut([parent_entity, *entity])?;
        let NodeSvgBundleMixin(node_bundle) = node_bundle_mixin.as_mut();
        let NodeSvgBundleMixin(child_bundle) = child_bundle_mixin.as_ref();

        if let Some(children_wrapper_element) = node_bundle.get_children_wrapper_element_mut() {
            children_wrapper_element
                .remove_child(child_bundle.get_svg_bundle().get_root_element().get_id());
        }
    }

    return Ok(());
}

fn process_added_node_children(
    parent_entity: Entity,
    added_entities: &[Entity],
    node_bundle_query: &mut Query<&mut NodeSvgBundleMixin>,
) -> Result<(), Box<dyn Error>> {
    for entity in added_entities {
        let [mut node_bundle_mixin, mut child_bundle_mixin] =
            node_bundle_query.get_many_mut([parent_entity, *entity])?;
        let NodeSvgBundleMixin(node_bundle) = node_bundle_mixin.as_mut();
        let NodeSvgBundleMixin(child_bundle) = child_bundle_mixin.as_mut();

        if let Some(children_wrapper_element) = node_bundle.get_children_wrapper_element_mut() {
            children_wrapper_element.append_child_in_world_context(
                *entity,
                child_bundle.get_svg_bundle_mut().get_root_element_mut(),
            );
        }
    }

    return Ok(());
}

fn reorder_node_children(
    parent_entity: Entity,
    new_entities_order: &[Entity],
    node_bundle_query: &mut Query<&mut NodeSvgBundleMixin>,
) -> Result<(), Box<dyn Error>> {
    // Track the original positions of the node children
    #[cfg(feature = "output_svg_element_changes")]
    let original_positions: SmallVec<[(Entity, SvgElementId); 2]> = {
        let NodeSvgBundleMixin(node_bundle) = node_bundle_query.get(parent_entity)?;
        let node_children = node_bundle
            .get_child_nodes()
            .ok_or(NoneErr::new("Failed to retrieve node children!"))?;
        node_children
            .iter()
            .filter_map(|entity| {
                let NodeSvgBundleMixin(bundle) = node_bundle_query.get(*entity).ok()?;
                Some((*entity, bundle.get_svg_bundle().get_root_element().get_id()))
            })
            .collect()
    };

    let mut bundle_mixin = node_bundle_query.get_mut(parent_entity)?;
    let NodeSvgBundleMixin(node_bundle) = bundle_mixin.as_mut();
    let node_children = node_bundle
        .get_child_nodes_mut()
        .ok_or(NoneErr::new("Failed to retrieve node children!"))?;

    // Sort bundle children
    node_children.sort_by_key(|bundle_child| {
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

            // If the child has been moved
            if original_index != new_index {
                let children_wrapper_element = node_bundle
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
        (&StylesMixin, &mut NodeSvgBundleMixin),
        (With<CompNode>, Changed<StylesMixin>),
    >,
    mut style_bundle_query: Query<&mut StyleSvgBundleMixin>,
) {
    for (StylesMixin(styles), mut node_bundle_mixin) in query.iter_mut() {
        let NodeSvgBundleMixin(node_bundle) = node_bundle_mixin.as_mut();
        let node_styles = match node_bundle.get_styles() {
            Some(node_styles) => node_styles,
            None => return,
        };

        // Identify removed and newly added style entities
        let current_node_styles_set: HashSet<Entity> = node_styles.iter().copied().collect();
        let new_node_styles_set: HashSet<Entity> = styles.iter().copied().collect();
        let mut removed_node_styles: SmallVec<[Entity; 2]> = current_node_styles_set
            .difference(&new_node_styles_set)
            .copied()
            .collect();
        let mut added_node_styles: SmallVec<[Entity; 2]> = new_node_styles_set
            .difference(&current_node_styles_set)
            .copied()
            .collect();

        // Apply detected changes
        process_removed_node_styles(
            node_bundle,
            &mut removed_node_styles,
            &mut style_bundle_query,
        )
        .unwrap();
        process_added_node_styles(node_bundle, &mut added_node_styles, &mut style_bundle_query)
            .unwrap();
        reorder_node_styles(node_bundle, &styles, &mut style_bundle_query).unwrap();
    }
}

fn process_removed_node_styles(
    node_bundle: &mut NodeSvgBundle,
    removed_entities: &[Entity],
    style_bundle_query: &mut Query<&mut StyleSvgBundleMixin>,
) -> Result<(), Box<dyn Error>> {
    for entity in removed_entities {
        let mut style_bundle_mixin = style_bundle_query.get_mut(*entity)?;
        let StyleSvgBundleMixin(style_bundle) = style_bundle_mixin.as_mut();

        if let Some(styles_wrapper_element) = node_bundle.get_styles_wrapper_element_mut() {
            styles_wrapper_element
                .remove_child(style_bundle.get_svg_bundle().get_root_element().get_id());
        }
    }

    return Ok(());
}

fn process_added_node_styles(
    node_bundle: &mut NodeSvgBundle,
    added_entities: &[Entity],
    style_bundle_query: &mut Query<&mut StyleSvgBundleMixin>,
) -> Result<(), Box<dyn Error>> {
    for entity in added_entities {
        let mut style_bundle_mixin = style_bundle_query.get_mut(*entity)?;
        let StyleSvgBundleMixin(style_bundle) = style_bundle_mixin.as_mut();

        if let Some(styles_wrapper_element) = node_bundle.get_styles_wrapper_element_mut() {
            styles_wrapper_element.append_child_in_world_context(
                *entity,
                style_bundle.get_svg_bundle_mut().get_root_element_mut(),
            );
        }
    }

    return Ok(());
}

fn reorder_node_styles(
    node_bundle: &mut NodeSvgBundle,
    new_entities_order: &[Entity],
    style_bundle_query: &mut Query<&mut StyleSvgBundleMixin>,
) -> Result<(), Box<dyn Error>> {
    // Track the original positions of the node styles
    #[cfg(feature = "output_svg_element_changes")]
    let original_positions: SmallVec<[(Entity, SvgElementId); 2]> = {
        let node_styles = node_bundle
            .get_styles()
            .ok_or(NoneErr::new("Failed to retrieve node styles!"))?;
        node_styles
            .iter()
            .filter_map(|entity| {
                let StyleSvgBundleMixin(bundle) = style_bundle_query.get(*entity).ok()?;
                Some((*entity, bundle.get_svg_bundle().get_root_element().get_id()))
            })
            .collect()
    };

    let node_styles = node_bundle
        .get_styles_mut()
        .ok_or(NoneErr::new("Failed to retrieve node styles!"))?;

    // Sort bundle children
    node_styles.sort_by_key(|bundle_child| {
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

        // Emit SvgElementReorderedChange events for node styles that have been moved
        for (new_index, &element_id) in new_positions.iter().enumerate() {
            let original_index = original_positions
                .iter()
                .position(|(_, e)| *e == element_id)
                .unwrap_or(new_index);

            // If the style has been moved
            if original_index != new_index {
                let styles_wrapper_element = node_bundle
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
    mut query: Query<
        (&VisibilityMixin, &mut NodeSvgBundleMixin),
        (With<CompNode>, Changed<VisibilityMixin>),
    >,
) {
    for (VisibilityMixin(visibility), mut node_bundle_mixin) in query.iter_mut() {
        let NodeSvgBundleMixin(node_bundle) = node_bundle_mixin.as_mut();
        let element = match node_bundle {
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
    for (SizeMixin(size), mut node_bundle_mixin) in query.iter_mut() {
        let NodeSvgBundleMixin(node_bundle) = node_bundle_mixin.as_mut();
        let [width, height] = size.0.to_array();

        // Apply dimension change to node
        match node_bundle {
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
    for (transform, mut node_bundle_mixin) in query.iter_mut() {
        let NodeSvgBundleMixin(node_bundle) = node_bundle_mixin.as_mut();
        let element = match node_bundle {
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
    for (OpacityMixin(opacity), mut node_bundle_mixin) in query.iter_mut() {
        let NodeSvgBundleMixin(node_bundle) = node_bundle_mixin.as_mut();
        let element = match node_bundle {
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
    for (BlendModeMixin(blend_mode), mut node_bundle_mixin) in query.iter_mut() {
        let NodeSvgBundleMixin(node_bundle) = node_bundle_mixin.as_mut();
        let element = match node_bundle {
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
    mut fill_style_bundle_query: Query<&mut StyleSvgBundleMixin, With<FillCompStyle>>,
) {
    for (PathMixin(path), mut node_bundle_mixin) in query.iter_mut() {
        let NodeSvgBundleMixin(node_bundle) = node_bundle_mixin.as_mut();

        // Apply path to bundle
        match node_bundle {
            NodeSvgBundle::Frame(frame_bundle) => frame_bundle
                .children_clipped_path
                .set_attribute(SvgAttribute::D { d: path.into() }),
            _ => {}
        }

        // Apply path to styles
        if let Some(styles) = node_bundle.get_styles_mut() {
            for style_entity in styles.iter_mut() {
                if let Ok(mut style_bundle_mixin) = fill_style_bundle_query.get_mut(*style_entity) {
                    let StyleSvgBundleMixin(style_bundle) = style_bundle_mixin.as_mut();
                    match style_bundle {
                        StyleSvgBundle::Solid(solid_bundle) => solid_bundle
                            .shape_path
                            .set_attribute(SvgAttribute::D { d: path.into() }),
                    }
                }
            }
        }
    }
}

pub fn apply_stroke_path_mixin_changes(
    mut query: Query<
        (&StrokePathMixin, &mut StyleSvgBundleMixin),
        (With<StrokeCompStyle>, Changed<StrokePathMixin>),
    >,
) {
    for (StrokePathMixin(stroke_path), mut style_bundle_mixin) in query.iter_mut() {
        let StyleSvgBundleMixin(style_bundle) = style_bundle_mixin.as_mut();

        // Apply stroke path to styles
        match style_bundle {
            StyleSvgBundle::Solid(solid_bundle) => {
                solid_bundle.shape_path.set_attribute(SvgAttribute::D {
                    d: stroke_path.into(),
                })
            }
        }
    }
}

pub fn apply_solid_paint_changes(
    query: Query<(&SolidCompPaint, &PaintParentMixin), (With<CompPaint>, Changed<SolidCompPaint>)>,
    mut style_bundle_query: Query<&mut StyleSvgBundleMixin>,
) {
    for (solid_paint, PaintParentMixin(node_entities)) in query.iter() {
        for node_entity in node_entities {
            if let Ok(mut style_bundle_mixin) = style_bundle_query.get_mut(*node_entity) {
                let StyleSvgBundleMixin(style_bundle) = style_bundle_mixin.as_mut();

                match style_bundle {
                    StyleSvgBundle::Solid(solid_bundle) => {
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
