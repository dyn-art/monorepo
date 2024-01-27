use std::collections::{BTreeMap, HashSet};

use bevy_ecs::entity::Entity;
use dyn_composition::core::utils::continuous_id::ContinuousId;

use crate::{
    mixin_change::MixinChange,
    resources::{
        changed_entities::{ChangedEntity, ChangedEntityType},
        svg_composition::{
            svg_bundle::SVGBundle,
            svg_context::SVGContext,
            svg_element::{
                attributes::{SVGAttribute, SVGMeasurementUnit},
                mapper::map_mat3_to_svg_transform,
                SVGElement, SVGTag,
            },
        },
    },
};

#[derive(Debug)]
pub struct FrameNodeSVGBundle {
    entity: Entity,

    root: SVGElement,
    defs: SVGElement,

    // Content elements
    content_clip_path: SVGElement,
    content_clipped_rect: SVGElement,
    content_wrapper_g: SVGElement,

    // Children elements
    children_wrapper_g: SVGElement,

    // Fill elements
    fill_clip_path: SVGElement,
    fill_clipped_path: SVGElement,
    fill_wrapper_g: SVGElement,

    // Children
    paint_children: Vec<Entity>,
    node_children: Vec<Entity>,
}

impl SVGBundle for FrameNodeSVGBundle {
    fn get_entity(&self) -> &Entity {
        &self.entity
    }

    fn get_type(&self) -> ChangedEntityType {
        ChangedEntityType::ShapeNode
    }

    fn append_child(&mut self, svg_bundle: &mut Box<dyn SVGBundle>) -> () {
        let svg_bundle_type = svg_bundle.get_type();
        match svg_bundle_type {
            ChangedEntityType::SolidPaint
            | ChangedEntityType::ImageFillPaint
            | ChangedEntityType::ImageFitPaint
            | ChangedEntityType::ImageCropPaint
            | ChangedEntityType::ImageTilePaint
            | ChangedEntityType::LinearGradientPaint
            | ChangedEntityType::RadialGradientPaint => {
                self.fill_wrapper_g
                    .append_child_in_svg_context(self.entity, svg_bundle.get_root_element_mut());
            }
            ChangedEntityType::FrameNode | ChangedEntityType::ShapeNode => {
                self.children_wrapper_g
                    .append_child_in_svg_context(self.entity, svg_bundle.get_root_element_mut());
            }
            _ => {}
        }
    }

    fn update(&mut self, changed_entity: ChangedEntity, cx: &mut SVGContext) -> () {
        for change in &changed_entity.changes {
            match change {
                MixinChange::Dimension(mixin) => {
                    self.root.set_attributes(vec![
                        SVGAttribute::Width {
                            width: mixin.width,
                            unit: SVGMeasurementUnit::Pixel,
                        },
                        SVGAttribute::Height {
                            height: mixin.height,
                            unit: SVGMeasurementUnit::Pixel,
                        },
                    ]);
                    self.fill_clipped_path.set_attributes(vec![
                        SVGAttribute::Width {
                            width: mixin.width,
                            unit: SVGMeasurementUnit::Pixel,
                        },
                        SVGAttribute::Height {
                            height: mixin.height,
                            unit: SVGMeasurementUnit::Pixel,
                        },
                    ]);
                    self.content_clipped_rect.set_attributes(vec![
                        SVGAttribute::Width {
                            width: mixin.width,
                            unit: SVGMeasurementUnit::Pixel,
                        },
                        SVGAttribute::Height {
                            height: mixin.height,
                            unit: SVGMeasurementUnit::Pixel,
                        },
                    ]);
                }
                MixinChange::RelativeTransform(mixin) => {
                    self.root.set_attribute(SVGAttribute::Transform {
                        transform: map_mat3_to_svg_transform(&mixin.relative_transform.0),
                    });
                }
                MixinChange::Children(mixin) => {
                    let new_children = &mixin.children.0;
                    let mut new_paint_children = Vec::new();
                    let mut new_node_children = Vec::new();

                    log::info!("[MixinChange::Children] Start {:?}", mixin.children.0);
                    log::info!("{:#?}", cx);

                    // Classify new children into paint and node categories
                    for &entity in new_children.iter() {
                        if let Some(bundle) = cx.get_bundle(&entity) {
                            match bundle.get_type() {
                                ChangedEntityType::FrameNode | ChangedEntityType::ShapeNode => {
                                    new_node_children.push(entity);
                                }
                                ChangedEntityType::SolidPaint => {
                                    new_paint_children.push(entity);
                                }
                                _ => {}
                            }
                        }
                    }

                    log::info!(
                        "[MixinChange::Children] Clasified: Paint {:?} | Node {:?}",
                        new_paint_children,
                        new_node_children
                    );

                    // Process node children
                    let current_node_children_set: HashSet<_> =
                        self.node_children.iter().cloned().collect();
                    let new_node_children_set: HashSet<_> =
                        new_node_children.iter().cloned().collect();

                    // 1. Identify removed elements
                    let removed_node_elements: Vec<_> = self
                        .node_children
                        .iter()
                        .filter(|&e| !new_node_children_set.contains(e))
                        .cloned()
                        .collect();

                    // 2. Identify newly added elements
                    let added_node_elements: Vec<_> = new_node_children
                        .iter()
                        .filter(|&e| !current_node_children_set.contains(e))
                        .cloned()
                        .collect();

                    log::info!(
                        "[MixinChange::Children] removed_node_elements: {:#?}",
                        removed_node_elements
                    );
                    log::info!(
                        "[MixinChange::Children] added_node_elements: {:#?}",
                        added_node_elements
                    );

                    // Process paint children
                    let current_paint_children_set: HashSet<_> =
                        self.paint_children.iter().cloned().collect();
                    let new_paint_children_set: HashSet<_> =
                        new_paint_children.iter().cloned().collect();

                    // 1. Identify removed elements
                    let removed_paint_elements: Vec<_> = self
                        .paint_children
                        .iter()
                        .filter(|&e| !new_paint_children_set.contains(e))
                        .cloned()
                        .collect();

                    // 2. Identify newly added elements
                    let added_paint_elements: Vec<_> = new_paint_children
                        .iter()
                        .filter(|&e| !current_paint_children_set.contains(e))
                        .cloned()
                        .collect();

                    log::info!(
                        "[MixinChange::Children] removed_paint_elements: {:#?}",
                        removed_paint_elements
                    );
                    log::info!(
                        "[MixinChange::Children] added_paint_elements: {:#?}",
                        added_paint_elements
                    );

                    // Process removed elements
                    // TODO

                    // Process added elements
                    // TODO

                    // Reorder elements
                    // TODO

                    // Update the current children
                    self.node_children = new_node_children;
                    self.paint_children = new_paint_children;
                }
                _ => {}
            }
        }
    }

    fn get_child_elements(&self) -> BTreeMap<ContinuousId, &SVGElement> {
        let mut children = BTreeMap::new();
        children.insert(self.defs.get_id(), &self.defs);
        children.insert(self.content_clip_path.get_id(), &self.content_clip_path);
        children.insert(
            self.content_clipped_rect.get_id(),
            &self.content_clipped_rect,
        );
        children.insert(self.content_wrapper_g.get_id(), &self.content_wrapper_g);
        children.insert(self.fill_clip_path.get_id(), &self.fill_clip_path);
        children.insert(self.fill_clipped_path.get_id(), &self.fill_clipped_path);
        children.insert(self.fill_wrapper_g.get_id(), &self.fill_wrapper_g);
        children.insert(self.children_wrapper_g.get_id(), &self.children_wrapper_g);
        return children;
    }

    fn get_child_elements_mut(&mut self) -> BTreeMap<ContinuousId, &mut SVGElement> {
        let mut children = BTreeMap::new();
        children.insert(self.defs.get_id(), &mut self.defs);
        children.insert(self.content_clip_path.get_id(), &mut self.content_clip_path);
        children.insert(
            self.content_clipped_rect.get_id(),
            &mut self.content_clipped_rect,
        );
        children.insert(self.content_wrapper_g.get_id(), &mut self.content_wrapper_g);
        children.insert(self.fill_clip_path.get_id(), &mut self.fill_clip_path);
        children.insert(self.fill_clipped_path.get_id(), &mut self.fill_clipped_path);
        children.insert(self.fill_wrapper_g.get_id(), &mut self.fill_wrapper_g);
        children.insert(
            self.children_wrapper_g.get_id(),
            &mut self.children_wrapper_g,
        );
        return children;
    }

    fn get_root_element(&self) -> &SVGElement {
        return &self.root;
    }

    fn get_root_element_mut(&mut self) -> &mut SVGElement {
        return &mut self.root;
    }

    fn to_string(&self, cx: &SVGContext) -> String {
        self.get_root_element().to_string(self, cx)
    }
}

impl FrameNodeSVGBundle {
    pub fn new(entity: Entity, cx: &mut SVGContext) -> Self {
        let mut root_element = cx.create_element(SVGTag::Group);

        // Create content elements

        let mut defs_element = cx.create_element(SVGTag::Defs);
        root_element.append_child_in_bundle_context(entity, &mut defs_element);

        let mut content_clip_path_element = cx.create_element(SVGTag::ClipPath);
        defs_element.append_child_in_bundle_context(entity, &mut content_clip_path_element);

        let mut content_clipped_rect_element = cx.create_element(SVGTag::Rect);
        content_clip_path_element
            .append_child_in_bundle_context(entity, &mut content_clipped_rect_element);

        let mut content_wrapper_g_element = cx.create_element(SVGTag::Group);
        content_wrapper_g_element.set_attribute(SVGAttribute::ClipPath {
            clip_path: content_clip_path_element.get_id(),
        });

        // Create fill elements

        let mut fill_clip_path_element = cx.create_element(SVGTag::ClipPath);
        defs_element.append_child_in_bundle_context(entity, &mut fill_clip_path_element);

        let mut fill_clipped_path_element = cx.create_element(SVGTag::Rect);
        fill_clip_path_element
            .append_child_in_bundle_context(entity, &mut fill_clipped_path_element);

        let mut fill_wrapper_g_element = cx.create_element(SVGTag::Group);
        fill_wrapper_g_element.set_attribute(SVGAttribute::ClipPath {
            clip_path: fill_clip_path_element.get_id(),
        });
        content_wrapper_g_element
            .append_child_in_bundle_context(entity, &mut fill_wrapper_g_element);

        // Create children wrapper element

        let mut children_wrapper_g_element = cx.create_element(SVGTag::Group);
        content_wrapper_g_element
            .append_child_in_bundle_context(entity, &mut children_wrapper_g_element);

        Self {
            entity,
            root: root_element,
            defs: defs_element,
            content_clip_path: content_clip_path_element,
            content_clipped_rect: content_clipped_rect_element,
            content_wrapper_g: content_wrapper_g_element,
            children_wrapper_g: children_wrapper_g_element,
            fill_clip_path: fill_clip_path_element,
            fill_clipped_path: fill_clipped_path_element,
            fill_wrapper_g: fill_wrapper_g_element,
            paint_children: Vec::new(),
            node_children: Vec::new(),
        }
    }
}
