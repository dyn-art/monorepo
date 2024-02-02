use std::{
    collections::{BTreeMap, HashSet},
    mem::take,
};

use bevy_ecs::entity::Entity;
use dyn_composition::utils::continuous_id::ContinuousId;

use crate::{
    mixin_change::MixinChange,
    resources::{
        changed_entities::{ChangedEntity, ChangedEntityType},
        svg_composition::{
            svg_bundle::SVGBundle,
            svg_context::SVGContext,
            svg_element::{
                attributes::{SVGAttribute, SVGDAttribute, SVGMeasurementUnit},
                mapper::{
                    map_anchors_to_svg_path_commands, map_anchors_to_svg_path_string,
                    map_blend_mode, map_mat3_to_svg_transform, map_path_commands_to_string,
                },
                styles::{SVGDisplayStyle, SVGStyle},
                SVGElement, SVGTag,
            },
        },
    },
};

#[derive(Debug)]
pub struct ShapeNodeSVGBundle {
    entity: Entity,

    root: SVGElement,
    defs: SVGElement,

    // Fill elements
    fill_clip_path: SVGElement,
    fill_clipped_path: SVGElement,
    fill_wrapper_g: SVGElement,

    // Click area elements
    click_area_rect: SVGElement,

    // Children
    paint_children: Vec<Entity>,
}

impl SVGBundle for ShapeNodeSVGBundle {
    fn get_entity(&self) -> &Entity {
        &self.entity
    }

    fn get_type(&self) -> ChangedEntityType {
        ChangedEntityType::ShapeNode
    }

    fn update(&mut self, changed_entity: ChangedEntity, cx: &mut SVGContext) -> () {
        for change in &changed_entity.changes {
            match change {
                MixinChange::NodeComposition(mixin) => {
                    self.root.set_styles(vec![SVGStyle::Display {
                        display: if mixin.is_visible {
                            SVGDisplayStyle::Block
                        } else {
                            SVGDisplayStyle::None
                        },
                    }]);
                }
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
                    self.click_area_rect.set_attributes(vec![
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
                MixinChange::Blend(mixin) => {
                    self.root.set_attributes(vec![SVGAttribute::Opacity {
                        opacity: mixin.opacity,
                    }]);
                    self.root.set_styles(vec![SVGStyle::BlendMode {
                        blend_mode: map_blend_mode(&mixin.blend_mode),
                    }]);
                }
                MixinChange::Path(mixin) => {
                    self.fill_clipped_path.set_attributes(vec![SVGAttribute::D {
                        d: SVGDAttribute::String {
                            value: map_anchors_to_svg_path_string(&mixin.vertices),
                        },
                    }]);
                }
                MixinChange::Children(mixin) => {
                    let new_children = &mixin.children.0;
                    let mut new_paint_children = Vec::new();

                    // Classify new children into paint and node categories
                    for &entity in new_children.iter() {
                        if let Some(bundle) = cx.get_bundle(&entity) {
                            match bundle.get_type() {
                                ChangedEntityType::SolidPaint
                                | ChangedEntityType::ImagePaint(..)
                                | ChangedEntityType::GradientPaint(..) => {
                                    new_paint_children.push(entity);
                                }
                                _ => {}
                            }
                        }
                    }

                    // Identify added and removed paint elements
                    let current_paint_children_set: HashSet<_> =
                        self.paint_children.iter().cloned().collect();
                    let new_paint_children_set: HashSet<_> =
                        new_paint_children.iter().cloned().collect();
                    let removed_paint_entities: Vec<_> = self
                        .paint_children
                        .iter()
                        .filter(|&e| !new_paint_children_set.contains(e))
                        .cloned()
                        .collect();
                    let added_paint_entities: Vec<_> = new_paint_children
                        .iter()
                        .filter(|&e| !current_paint_children_set.contains(e))
                        .cloned()
                        .collect();

                    // Process removed entities
                    for entity in removed_paint_entities {
                        if let Some(removed_bundle) = cx.remove_bundle(&entity) {
                            self.fill_wrapper_g
                                .remove_child(removed_bundle.get_root_element().get_id());
                        }
                    }

                    // Process added entities
                    for entity in added_paint_entities {
                        if let Some(bundle) = cx.get_bundle_mut(&entity) {
                            self.fill_wrapper_g
                                .append_child_in_svg_context(entity, bundle.get_root_element_mut());
                        }
                    }

                    // Reorder entities
                    // TODO

                    // Update the current children
                    self.paint_children = new_paint_children;
                }
                _ => {}
            }
        }
    }

    fn get_child_elements(&self) -> BTreeMap<ContinuousId, &SVGElement> {
        let mut children = BTreeMap::new();
        children.insert(self.defs.get_id(), &self.defs);
        children.insert(self.click_area_rect.get_id(), &self.click_area_rect);
        children.insert(self.fill_clip_path.get_id(), &self.fill_clip_path);
        children.insert(self.fill_clipped_path.get_id(), &self.fill_clipped_path);
        children.insert(self.fill_wrapper_g.get_id(), &self.fill_wrapper_g);
        return children;
    }

    fn get_child_elements_mut(&mut self) -> BTreeMap<ContinuousId, &mut SVGElement> {
        let mut children = BTreeMap::new();
        children.insert(self.defs.get_id(), &mut self.defs);
        children.insert(self.click_area_rect.get_id(), &mut self.click_area_rect);
        children.insert(self.fill_clip_path.get_id(), &mut self.fill_clip_path);
        children.insert(self.fill_clipped_path.get_id(), &mut self.fill_clipped_path);
        children.insert(self.fill_wrapper_g.get_id(), &mut self.fill_wrapper_g);
        return children;
    }

    fn get_root_element(&self) -> &SVGElement {
        return &self.root;
    }

    fn get_root_element_mut(&mut self) -> &mut SVGElement {
        return &mut self.root;
    }

    fn destroy(&mut self, cx: &mut SVGContext) {
        // Destroy child entities
        for entity in take(&mut self.paint_children) {
            if let Some(removed_bundle) = cx.remove_bundle(&entity) {
                self.fill_wrapper_g
                    .remove_child(removed_bundle.get_root_element().get_id());
            }
        }

        // Destroy elements associated with the bundle.
        // Removing the root also implicitly removes its child elements.
        cx.destroy_element(self.get_root_element_mut());
    }

    fn to_string(&self, cx: &SVGContext) -> String {
        self.get_root_element().to_string(self, cx)
    }
}

impl ShapeNodeSVGBundle {
    pub fn new(entity: Entity, cx: &mut SVGContext) -> Self {
        let mut root_element = cx.create_bundle_root_element(SVGTag::Group, entity);
        #[cfg(feature = "tracing")]
        root_element.set_attribute(SVGAttribute::Name {
            name: Self::create_element_name(root_element.get_id(), String::from("root"), false),
        });

        let mut defs_element = cx.create_element(SVGTag::Defs);
        #[cfg(feature = "tracing")]
        defs_element.set_attribute(SVGAttribute::Name {
            name: Self::create_element_name(defs_element.get_id(), String::from("defs"), false),
        });
        root_element.append_child_in_bundle_context(entity, &mut defs_element);

        // Create click area elements

        let mut click_area_rect_element = cx.create_element(SVGTag::Rect);
        #[cfg(feature = "tracing")]
        click_area_rect_element.set_attributes(vec![
            SVGAttribute::Name {
                name: Self::create_element_name(
                    click_area_rect_element.get_id(),
                    String::from("click-area-rect"),
                    false,
                ),
            },
            SVGAttribute::Fill {
                fill: String::from("rgba(255, 204, 203, 0.5)"),
            },
        ]);
        #[cfg(not(feature = "tracing"))]
        click_area_rect_element.set_attribute(SVGAttribute::Fill {
            fill: String::from("transparent"),
        });
        root_element.append_child_in_bundle_context(entity, &mut click_area_rect_element);

        // Create fill elements

        let mut fill_clip_path_element = cx.create_element(SVGTag::ClipPath);
        #[cfg(feature = "tracing")]
        fill_clip_path_element.set_attribute(SVGAttribute::Name {
            name: Self::create_element_name(
                fill_clip_path_element.get_id(),
                String::from("fill-clip-path"),
                true,
            ),
        });
        defs_element.append_child_in_bundle_context(entity, &mut fill_clip_path_element);

        let mut fill_clipped_path_element = cx.create_element(SVGTag::Path);
        #[cfg(feature = "tracing")]
        fill_clipped_path_element.set_attribute(SVGAttribute::Name {
            name: Self::create_element_name(
                fill_clipped_path_element.get_id(),
                String::from("fill-clipped-path"),
                false,
            ),
        });
        fill_clip_path_element
            .append_child_in_bundle_context(entity, &mut fill_clipped_path_element);

        let mut fill_wrapper_g_element = cx.create_element(SVGTag::Group);
        #[cfg(feature = "tracing")]
        fill_wrapper_g_element.set_attribute(SVGAttribute::Name {
            name: Self::create_element_name(
                fill_wrapper_g_element.get_id(),
                String::from("fill-wrapper-g"),
                false,
            ),
        });
        fill_wrapper_g_element.set_attribute(SVGAttribute::ClipPath {
            clip_path: fill_clip_path_element.get_id(),
        });
        root_element.append_child_in_bundle_context(entity, &mut fill_wrapper_g_element);

        Self {
            entity,
            root: root_element,
            defs: defs_element,

            // Click area elements
            click_area_rect: click_area_rect_element,

            // Fill elements
            fill_clip_path: fill_clip_path_element,
            fill_clipped_path: fill_clipped_path_element,
            fill_wrapper_g: fill_wrapper_g_element,

            paint_children: Vec::new(),
        }
    }

    #[cfg(feature = "tracing")]
    fn create_element_name(id: ContinuousId, category: String, is_definition: bool) -> String {
        let def_part = if is_definition { "_def" } else { "" };
        format!("shape_{}_{}{}", category, id, def_part)
    }
}
