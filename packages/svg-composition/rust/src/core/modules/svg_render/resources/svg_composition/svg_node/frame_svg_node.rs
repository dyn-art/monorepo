use std::sync::mpsc::Sender;

use bevy_ecs::entity::Entity;

use crate::core::{
    events::output_event::OutputEvent,
    modules::svg_render::{
        mixin_change::MixinChange,
        resources::svg_composition::{
            svg_composition::SVGComposition,
            svg_element::{SVGChildElementIdentifier, SVGElement, SVGTag},
        },
    },
};

use super::{base_svg_node::BaseSVGNode, ElementReference, SVGNode};

#[derive(Debug)]
pub struct FrameSVGNode {
    pub base: BaseSVGNode,

    // Content elements
    content_wrapper: ElementReference,
    content_clip_path: ElementReference,
    content_clip_path_defs: ElementReference,
    content_clipped_shape: ElementReference,

    // Children elements
    children_wrapper: ElementReference,

    // Fill elements
    fill_clip_path: ElementReference,
    fill_clip_path_defs: ElementReference,
    fill_clipped_shape: ElementReference,
    fill_element: ElementReference,
}

impl SVGNode for FrameSVGNode {
    fn get_base(&self) -> &BaseSVGNode {
        &self.base
    }

    fn get_base_mut(&mut self) -> &mut BaseSVGNode {
        &mut self.base
    }

    fn apply_mixin_changes(&mut self, changes: &[MixinChange]) {
        for change in changes {
            match change {
                MixinChange::Dimension(mixin) => {
                    let content_clipped_shape_index = self.content_clipped_shape.index;
                    let fill_clipped_shape_index = self.fill_clipped_shape.index;

                    let width_attr = ("width".to_string(), mixin.width.to_string());
                    let height_attr = ("height".to_string(), mixin.height.to_string());

                    let base = self.get_base_mut();
                    base.set_attributes(vec![width_attr.clone(), height_attr.clone()]);
                    base.set_attributes_at(
                        fill_clipped_shape_index,
                        vec![width_attr.clone(), height_attr.clone()],
                    );
                    base.set_attributes_at(
                        content_clipped_shape_index,
                        vec![width_attr, height_attr],
                    );
                }
                _ => {
                    // do nothing
                }
            }
        }
    }

    fn append_external_child(&mut self, entity: Entity) {
        let children_wrapper_index = self.children_wrapper.index;
        self.get_base_mut()
            .get_child_element_at_mut(children_wrapper_index)
            .unwrap()
            .append_child(SVGChildElementIdentifier::OutOfContext(entity));
    }

    fn to_string(&self, composition: &SVGComposition) -> String {
        self.base.to_string(composition)
    }
}

impl FrameSVGNode {
    pub fn new(
        output_event_sender: Sender<OutputEvent>,
        maybe_parent_element_id: Option<u32>,
    ) -> Self {
        // TODO: implment clip path without having to remove or add elements
        // as the size should be known at compile time so that we can use Vector
        // over Hashmap for storing SVGElements

        // Create root element and apply it to SVG node
        let mut element = SVGElement::new(SVGTag::Group);
        #[cfg(feature = "trace")]
        element.set_attribute(
            String::from("name"),
            FrameSVGNode::create_element_name(element.get_id(), String::from("root"), false),
        );
        let mut base = BaseSVGNode::new(
            element,
            maybe_parent_element_id,
            output_event_sender.clone(),
        );

        // Create content elements
        let mut content_clip_path_defs_element = SVGElement::new(SVGTag::Defs);
        let content_clip_path_defs_id = content_clip_path_defs_element.get_id();
        #[cfg(feature = "trace")]
        content_clip_path_defs_element.set_attribute(
            String::from("name"),
            FrameSVGNode::create_element_name(
                content_clip_path_defs_id,
                String::from("content-defs"),
                false,
            ),
        );
        let content_clip_path_defs_index =
            base.append_child_element(content_clip_path_defs_element);

        let mut content_clip_path_element = SVGElement::new(SVGTag::ClipPath);
        let content_clip_path_id = content_clip_path_element.get_id();
        #[cfg(feature = "trace")]
        content_clip_path_element.set_attribute(
            String::from("name"),
            FrameSVGNode::create_element_name(
                content_clip_path_id,
                String::from("content-clip"),
                true,
            ),
        );
        let content_clip_path_index = base
            .append_child_element_to(content_clip_path_defs_index, content_clip_path_element)
            .unwrap();

        let mut content_clipped_shape_element = SVGElement::new(SVGTag::Rect);
        let content_clipped_shape_id = content_clipped_shape_element.get_id();
        #[cfg(feature = "trace")]
        content_clipped_shape_element.set_attribute(
            String::from("name"),
            FrameSVGNode::create_element_name(
                content_clipped_shape_id,
                String::from("content-clipped-shape"),
                false,
            ),
        );
        let content_clipped_shape_index = base
            .append_child_element_to(content_clip_path_index, content_clipped_shape_element)
            .unwrap();

        let mut content_wrapper = SVGElement::new(SVGTag::Group);
        let content_wrapper_id = content_wrapper.get_id();
        #[cfg(feature = "trace")]
        content_wrapper.set_attribute(
            String::from("name"),
            FrameSVGNode::create_element_name(content_wrapper_id, String::from("content"), false),
        );
        content_wrapper.set_attribute(
            String::from("clipPath"),
            format!("url(#{content_clip_path_id})"),
        );
        let content_wrapper_index = base.append_child_element(content_wrapper);

        // Create fill elements
        let mut fill_clip_path_defs = SVGElement::new(SVGTag::Defs);
        let fill_clip_path_defs_id = fill_clip_path_defs.get_id();
        #[cfg(feature = "trace")]
        fill_clip_path_defs.set_attribute(
            String::from("name"),
            FrameSVGNode::create_element_name(
                fill_clip_path_defs_id,
                String::from("fill-defs"),
                false,
            ),
        );
        let fill_clip_path_defs_index = base
            .append_child_element_to(content_wrapper_index, fill_clip_path_defs)
            .unwrap();

        let mut fill_clip_path_element = SVGElement::new(SVGTag::ClipPath);
        let fill_clip_path_id = fill_clip_path_element.get_id();
        #[cfg(feature = "trace")]
        fill_clip_path_element.set_attribute(
            String::from("name"),
            FrameSVGNode::create_element_name(fill_clip_path_id, String::from("fill-clip"), true),
        );
        let fill_clip_path_index = base
            .append_child_element_to(fill_clip_path_defs_index, fill_clip_path_element)
            .unwrap();

        let mut fill_clipped_shape_element = SVGElement::new(SVGTag::Rect);
        let fill_clipped_shape_id = fill_clipped_shape_element.get_id();
        #[cfg(feature = "trace")]
        fill_clipped_shape_element.set_attribute(
            String::from("name"),
            FrameSVGNode::create_element_name(
                fill_clipped_shape_id,
                String::from("fill-clipped-shape"),
                false,
            ),
        );
        let fill_clipped_shape_index = base
            .append_child_element_to(fill_clip_path_index, fill_clipped_shape_element)
            .unwrap();

        let mut fill_element = SVGElement::new(SVGTag::Group);
        let fill_element_id = fill_element.get_id();
        #[cfg(feature = "trace")]
        fill_element.set_attribute(
            String::from("name"),
            FrameSVGNode::create_element_name(fill_element_id, String::from("fill"), false),
        );
        fill_element.set_attribute(
            String::from("clipPath"),
            format!("url(#{fill_clip_path_id})"),
        );
        let fill_element_index = base
            .append_child_element_to(content_wrapper_index, fill_element)
            .unwrap();

        // Create children wrapper element
        let mut children_wrapper = SVGElement::new(SVGTag::Group);
        let children_wrapper_id = children_wrapper.get_id();
        #[cfg(feature = "trace")]
        children_wrapper.set_attribute(
            String::from("name"),
            FrameSVGNode::create_element_name(children_wrapper_id, String::from("children"), false),
        );
        let children_wrapper_index = base
            .append_child_element_to(content_wrapper_index, children_wrapper)
            .unwrap();

        Self {
            base,

            // Content element references
            content_clip_path_defs: ElementReference {
                id: content_clip_path_defs_id,
                index: content_clip_path_defs_index,
            },
            content_clip_path: ElementReference {
                id: content_clip_path_id,
                index: content_clip_path_index,
            },
            content_clipped_shape: ElementReference {
                id: content_clipped_shape_id,
                index: content_clipped_shape_index,
            },
            content_wrapper: ElementReference {
                id: content_wrapper_id,
                index: content_wrapper_index,
            },

            // Children element references
            children_wrapper: ElementReference {
                id: children_wrapper_id,
                index: children_wrapper_index,
            },

            // Fill element references
            fill_clip_path_defs: ElementReference {
                id: fill_clip_path_defs_id,
                index: fill_clip_path_defs_index,
            },
            fill_clip_path: ElementReference {
                id: fill_clip_path_id,
                index: fill_clip_path_index,
            },
            fill_clipped_shape: ElementReference {
                id: fill_clipped_shape_id,
                index: fill_clipped_shape_index,
            },
            fill_element: ElementReference {
                id: fill_element_id,
                index: fill_element_index,
            },
        }
    }

    #[cfg(feature = "trace")]
    fn create_element_name(id: u32, category: String, is_definition: bool) -> String {
        let def_part = if is_definition { "def" } else { "" };
        format!("frame_{}_{}{}", category, id, def_part)
    }
}
