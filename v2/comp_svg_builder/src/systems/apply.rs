use crate::{
    resources::delayed_svg_bundle_modifications::{
        DelayedSvgBundleModificationsRes, SvgBundleChildrenModification,
    },
    svg::{
        svg_bundle::{FillSvgBundle, NodeSvgBundle, NodeSvgBundleMixin},
        svg_element::{
            attributes::{SvgAttribute, SvgMeasurementUnit},
            styles::{SvgDisplayStyle, SvgStyle},
        },
    },
};
use bevy_ecs::{
    entity::Entity,
    query::{Changed, With},
    system::{Query, ResMut},
};
use bevy_hierarchy::Children;
use bevy_transform::components::Transform;
use dyn_comp_types::{
    common::Visibility,
    mixins::{BlendModeMixin, OpacityMixin, PaintAppliedOn, SizeMixin, VisibilityMixin},
    nodes::CompNode,
    paints::{CompPaint, SolidCompPaint},
};
use std::collections::HashSet;

pub fn collect_node_children_changes(
    mut delayed_node_modification_res: ResMut<DelayedSvgBundleModificationsRes>,
    query: Query<(Entity, &Children, &mut NodeSvgBundleMixin), (With<CompNode>, Changed<Children>)>,
) {
    for (entity, children, NodeSvgBundleMixin(bundle)) in query.iter() {
        let node_children = match &bundle {
            NodeSvgBundle::Frame(bundle) => &bundle.node_children,
            _ => return,
        };

        // Identify removed and newly added node entities
        let current_node_children_set = node_children.iter().copied().collect::<HashSet<_>>();
        let new_node_children_set = children.iter().copied().collect::<HashSet<_>>();
        let removed_node_entities: Vec<_> = current_node_children_set
            .difference(&new_node_children_set)
            .cloned()
            .collect();
        let added_node_entities: Vec<_> = new_node_children_set
            .difference(&current_node_children_set)
            .cloned()
            .collect();

        delayed_node_modification_res
            .children_modifications
            .push(SvgBundleChildrenModification {
                parent_entity: entity,
                added_entities: added_node_entities,
                removed_entities: removed_node_entities,
            });
    }
}

pub fn apply_node_children_changes(
    mut delayed_svg_bundle_modifications_res: ResMut<DelayedSvgBundleModificationsRes>,
    mut bundle_query: Query<&mut NodeSvgBundleMixin>,
) {
    let modifications = delayed_svg_bundle_modifications_res
        .children_modifications
        .drain(..)
        .collect::<Vec<_>>();

    for modification in modifications {
        // Process removed entities
        for entity in modification.removed_entities {
            let [mut bundle_mixin, child_bundle_mixin] =
                bundle_query.many_mut([modification.parent_entity, entity]);
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

        // Process added entities
        for entity in modification.added_entities {
            let [mut bundle_mixin, mut child_bundle_mixin] =
                bundle_query.many_mut([modification.parent_entity, entity]);
            let NodeSvgBundleMixin(bundle) = bundle_mixin.as_mut();
            let NodeSvgBundleMixin(child_bundle) = child_bundle_mixin.as_mut();
            match bundle {
                NodeSvgBundle::Frame(frame_node) => {
                    frame_node.children_wrapper_g.append_child_in_world_context(
                        entity,
                        child_bundle.get_svg_bundle_mut().get_root_element_mut(),
                    );
                }
                _ => {}
            }
        }

        // TODO: Reorder
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
    query: Query<(&SolidCompPaint, &PaintAppliedOn), (With<CompPaint>, Changed<SolidCompPaint>)>,
    mut node_bundle_query: Query<&mut NodeSvgBundleMixin>,
) {
    for (solid_paint, PaintAppliedOn(entites)) in query.iter() {
        for entity in entites {
            if let Ok(mut bundle_mixin) = node_bundle_query.get_mut(*entity) {
                let NodeSvgBundleMixin(bundle) = bundle_mixin.as_mut();
                let bundle_fills = match bundle {
                    NodeSvgBundle::Frame(bundle) => &mut bundle.fills,
                    NodeSvgBundle::Shape(bundle) => &mut bundle.fills,
                    _ => return,
                };

                if let Some(fill) = bundle_fills
                    .iter_mut()
                    .find(|fill| fill.get_paint_entity() == entity)
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
