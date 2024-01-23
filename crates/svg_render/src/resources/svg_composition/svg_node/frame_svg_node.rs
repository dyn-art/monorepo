use bevy_ecs::entity::Entity;
use dyn_composition::core::utils::continuous_id::ContinuousId;

use crate::{
    events::output_event::ElementChangeEvent,
    mixin_change::NodeMixinChange,
    resources::{
        changed_components::ChangedNode,
        svg_composition::{
            svg_bundle::{BaseSVGBundle, SVGBundle},
            svg_element::{
                attributes::{SVGAttribute, SVGMeasurementUnit},
                helper::mat3_to_svg_transform,
                mapper::map_blend_mode,
                styles::{SVGDisplayStyle, SVGStyle},
                SVGElement, SVGTag,
            },
            svg_node::ElementReference,
            SVGCompositionRes,
        },
    },
};

use super::SVGNode;

#[derive(Debug)]
pub struct FrameSVGNode {
    bundle: BaseSVGBundle,

    defs: ElementReference,

    // Content elements
    content_clip_path: ElementReference,
    content_clipped_rect: ElementReference,
    content_wrapper_g: ElementReference,

    // Children elements
    children_wrapper_g: ElementReference,

    // Fill elements
    fill_clip_path: ElementReference,
    fill_clipped_path: ElementReference,
    fill_wrapper_g: ElementReference,
}

impl SVGBundle for FrameSVGNode {
    fn get_bundle(&self) -> &BaseSVGBundle {
        &self.bundle
    }

    fn get_bundle_mut(&mut self) -> &mut BaseSVGBundle {
        &mut self.bundle
    }

    fn drain_changes(&mut self) -> Vec<ElementChangeEvent> {
        self.get_bundle_mut().drain_changes()
    }

    fn to_string(&self, composition: &SVGCompositionRes) -> String {
        self.bundle.to_string(composition)
    }
}

impl SVGNode for FrameSVGNode {
    fn apply_node_change(&mut self, changed_node: &ChangedNode) {
        for change in &changed_node.changes {
            match change {
                NodeMixinChange::NodeComposition(mixin) => {
                    self.bundle
                        .get_root_mut()
                        .set_styles(vec![SVGStyle::Display {
                            display: if mixin.is_visible {
                                SVGDisplayStyle::Block
                            } else {
                                SVGDisplayStyle::None
                            },
                        }]);
                }
                NodeMixinChange::Dimension(mixin) => {
                    self.bundle.get_root_mut().set_attributes(vec![
                        SVGAttribute::Width {
                            width: mixin.width,
                            unit: SVGMeasurementUnit::Pixel,
                        },
                        SVGAttribute::Height {
                            height: mixin.height,
                            unit: SVGMeasurementUnit::Pixel,
                        },
                    ]);
                    self.bundle
                        .get_child_mut(self.fill_clipped_path.index)
                        .unwrap()
                        .set_attributes(vec![
                            SVGAttribute::Width {
                                width: mixin.width,
                                unit: SVGMeasurementUnit::Pixel,
                            },
                            SVGAttribute::Height {
                                height: mixin.height,
                                unit: SVGMeasurementUnit::Pixel,
                            },
                        ]);
                    self.bundle
                        .get_child_mut(self.content_clipped_rect.index)
                        .unwrap()
                        .set_attributes(vec![
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
                NodeMixinChange::RelativeTransform(mixin) => {
                    self.bundle.get_root_mut().set_attributes(vec![
                        (SVGAttribute::Transform {
                            transform: mat3_to_svg_transform(mixin.relative_transform.0),
                        }),
                    ]);
                }
                NodeMixinChange::Blend(mixin) => {
                    let root_element = self.bundle.get_root_mut();
                    root_element.set_attributes(vec![SVGAttribute::Opacity {
                        opacity: mixin.opacity,
                    }]);
                    root_element.set_styles(vec![SVGStyle::BlendMode {
                        blend_mode: map_blend_mode(&mixin.blend_mode),
                    }]);
                }
                NodeMixinChange::Children(mixin) => {
                    self.bundle
                        .get_child_mut(self.children_wrapper_g.index)
                        .unwrap()
                        .reorder_children(&mixin.children.0);
                }
                _ => {
                    // do nothing
                }
            }
        }
    }

    fn get_node_append_id(&self) -> Option<&ElementReference> {
        Some(&self.children_wrapper_g)
    }

    fn get_paint_append_id(&self) -> Option<&ElementReference> {
        Some(&self.fill_wrapper_g)
    }
}

impl FrameSVGNode {
    pub fn new(entity: Entity, id_generator: &mut ContinuousId) -> Self {
        // TODO: implment clip path without having to remove or add elements
        // as the size should be known at compile time so that we can use Vector
        // over Hashmap for storing SVGElements

        // Create root element
        let mut element = SVGElement::new(SVGTag::Group, id_generator);
        let element_id = element.get_id();
        #[cfg(feature = "tracing")]
        element.set_attribute(SVGAttribute::Name {
            name: FrameSVGNode::create_element_name(element.get_id(), String::from("root"), false),
        });
        let mut bundle = BaseSVGBundle::new(element, entity);

        let mut defs_element = SVGElement::new(SVGTag::Defs, id_generator);
        let defs_id = defs_element.get_id();
        #[cfg(feature = "tracing")]
        defs_element.set_attribute(SVGAttribute::Name {
            name: FrameSVGNode::create_element_name(defs_id, String::from("defs"), false),
        });
        let defs_index = bundle.append_child(defs_element);

        // Create content elements
        let mut content_clip_path_element = SVGElement::new(SVGTag::ClipPath, id_generator);
        let content_clip_path_id = content_clip_path_element.get_id();
        #[cfg(feature = "tracing")]
        content_clip_path_element.set_attribute(SVGAttribute::Name {
            name: FrameSVGNode::create_element_name(
                content_clip_path_id,
                String::from("content-clip-path"),
                true,
            ),
        });
        let content_clip_path_index = bundle
            .append_child_to(defs_index, content_clip_path_element)
            .unwrap();

        let mut content_clipped_rect_element = SVGElement::new(SVGTag::Rect, id_generator);
        let content_clipped_rect_id = content_clipped_rect_element.get_id();
        #[cfg(feature = "tracing")]
        content_clipped_rect_element.set_attribute(SVGAttribute::Name {
            name: FrameSVGNode::create_element_name(
                content_clipped_rect_id,
                String::from("content-clipped-rect"),
                false,
            ),
        });
        let content_clipped_rect_index = bundle
            .append_child_to(content_clip_path_index, content_clipped_rect_element)
            .unwrap();

        let mut content_wrapper_g = SVGElement::new(SVGTag::Group, id_generator);
        let content_wrapper_g_id = content_wrapper_g.get_id();
        #[cfg(feature = "tracing")]
        content_wrapper_g.set_attribute(SVGAttribute::Name {
            name: FrameSVGNode::create_element_name(
                content_wrapper_g_id,
                String::from("content-wrapper-g"),
                false,
            ),
        });
        content_wrapper_g.set_attribute(SVGAttribute::ClipPath {
            clip_path: content_clip_path_id,
        });
        let content_wrapper_g_index = bundle.append_child(content_wrapper_g);

        // Create fill elements
        let mut fill_clip_path_element = SVGElement::new(SVGTag::ClipPath, id_generator);
        let fill_clip_path_id = fill_clip_path_element.get_id();
        #[cfg(feature = "tracing")]
        fill_clip_path_element.set_attribute(SVGAttribute::Name {
            name: FrameSVGNode::create_element_name(
                fill_clip_path_id,
                String::from("fill-clip-path"),
                true,
            ),
        });
        let fill_clip_path_index = bundle
            .append_child_to(defs_index, fill_clip_path_element)
            .unwrap();

        let mut fill_clipped_path_element = SVGElement::new(SVGTag::Rect, id_generator);
        let fill_clipped_path_id = fill_clipped_path_element.get_id();
        #[cfg(feature = "tracing")]
        fill_clipped_path_element.set_attribute(SVGAttribute::Name {
            name: FrameSVGNode::create_element_name(
                fill_clipped_path_id,
                String::from("fill-clipped-path"),
                false,
            ),
        });
        let fill_clipped_path_index = bundle
            .append_child_to(fill_clip_path_index, fill_clipped_path_element)
            .unwrap();

        let mut fill_wrapper_g_element = SVGElement::new(SVGTag::Group, id_generator);
        let fill_wrapper_g_id = fill_wrapper_g_element.get_id();
        #[cfg(feature = "tracing")]
        fill_wrapper_g_element.set_attribute(SVGAttribute::Name {
            name: FrameSVGNode::create_element_name(
                fill_wrapper_g_id,
                String::from("fill-wrapper-g"),
                false,
            ),
        });
        fill_wrapper_g_element.set_attribute(SVGAttribute::ClipPath {
            clip_path: fill_clip_path_id,
        });
        let fill_wrapper_g_index = bundle
            .append_child_to(content_wrapper_g_index, fill_wrapper_g_element)
            .unwrap();

        // Create children wrapper element
        let mut children_wrapper_g = SVGElement::new(SVGTag::Group, id_generator);
        let children_wrapper_g_id = children_wrapper_g.get_id();
        #[cfg(feature = "tracing")]
        children_wrapper_g.set_attribute(SVGAttribute::Name {
            name: FrameSVGNode::create_element_name(
                children_wrapper_g_id,
                String::from("children-wrapper-g"),
                false,
            ),
        });
        let children_wrapper_g_index = bundle
            .append_child_to(content_wrapper_g_index, children_wrapper_g)
            .unwrap();

        Self {
            bundle,

            // Content element references
            defs: ElementReference {
                id: defs_id,
                index: defs_index,
            },
            content_clip_path: ElementReference {
                id: content_clip_path_id,
                index: content_clip_path_index,
            },
            content_clipped_rect: ElementReference {
                id: content_clipped_rect_id,
                index: content_clipped_rect_index,
            },
            content_wrapper_g: ElementReference {
                id: content_wrapper_g_id,
                index: content_wrapper_g_index,
            },

            // Children element references
            children_wrapper_g: ElementReference {
                id: children_wrapper_g_id,
                index: children_wrapper_g_index,
            },

            // Fill element references
            fill_clip_path: ElementReference {
                id: fill_clip_path_id,
                index: fill_clip_path_index,
            },
            fill_clipped_path: ElementReference {
                id: fill_clipped_path_id,
                index: fill_clipped_path_index,
            },
            fill_wrapper_g: ElementReference {
                id: fill_wrapper_g_id,
                index: fill_wrapper_g_index,
            },
        }
    }

    #[cfg(feature = "tracing")]
    fn create_element_name(id: ContinuousId, category: String, is_definition: bool) -> String {
        let def_part = if is_definition { "_def" } else { "" };
        format!("frame_{}_{}{}", category, id, def_part)
    }
}
