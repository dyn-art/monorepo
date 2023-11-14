use std::sync::mpsc::Sender;

use bevy_ecs::entity::Entity;

use crate::core::{
    events::output_event::OutputEvent,
    modules::svg_render::{
        mixin_change::MixinChange,
        resources::svg_composition::{
            svg_composition::SVGComposition,
            svg_element::{SVGElement, SVGTag},
        },
    },
};

use super::{base_svg_node::BaseSVGNode, ElementReference, SVGNode};

#[derive(Debug)]
pub struct ShapeSVGNode {
    pub base: BaseSVGNode,

    // Sender to enque events for frontend
    output_event_sender: Sender<OutputEvent>,

    // Fill elements
    fill_clip_path: ElementReference,
    fill_clip_path_defs: ElementReference,
    fill_clipped_shape: ElementReference,
    fill_element: ElementReference,
}

impl SVGNode for ShapeSVGNode {
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
                    let base = self.get_base_mut();
                    base.get_element_mut()
                        .set_attribute(String::from("width"), mixin.width.to_string());
                    base.get_element_mut()
                        .set_attribute(String::from("height"), mixin.height.to_string());
                }
                _ => {
                    // do nothing
                }
            }
        }
    }

    fn append_external_child(&mut self, entity: Entity) -> () {
        // do nothing as not supported
    }

    fn to_string(&self, composition: &SVGComposition) -> String {
        self.base.to_string(composition)
    }
}

impl ShapeSVGNode {
    pub fn new(
        output_event_sender: Sender<OutputEvent>,
        maybe_parent_element_id: Option<u32>,
    ) -> Self {
        // Create root element and apply it to SVG node
        let mut element = SVGElement::new(SVGTag::Group, output_event_sender.clone());
        #[cfg(feature = "trace")]
        element.set_attribute(
            String::from("name"),
            ShapeSVGNode::create_element_name(element.get_id(), String::from("root"), false),
        );
        let mut base = BaseSVGNode::new(
            element,
            maybe_parent_element_id,
            output_event_sender.clone(),
        );

        // Create fill elements
        let mut fill_clip_path_defs = base.create_element(SVGTag::Defs);
        let fill_clip_path_defs_id = fill_clip_path_defs.get_id();
        #[cfg(feature = "trace")]
        fill_clip_path_defs.set_attribute(
            String::from("name"),
            ShapeSVGNode::create_element_name(
                fill_clip_path_defs_id,
                String::from("fill-defs"),
                false,
            ),
        );
        let fill_clip_path_defs_index = base.append_child(fill_clip_path_defs);

        let mut fill_clip_path_element = base.create_element(SVGTag::Defs);
        let fill_clip_path_id = fill_clip_path_element.get_id();
        #[cfg(feature = "trace")]
        fill_clip_path_element.set_attribute(
            String::from("name"),
            ShapeSVGNode::create_element_name(fill_clip_path_id, String::from("fill-clip"), true),
        );
        let fill_clip_path_index = base
            .append_child_to(fill_clip_path_defs_index, fill_clip_path_element)
            .unwrap();

        let mut fill_clipped_shape_element = base.create_element(SVGTag::Rect);
        let fill_clipped_shape_id = fill_clipped_shape_element.get_id();
        #[cfg(feature = "trace")]
        fill_clipped_shape_element.set_attribute(
            String::from("name"),
            ShapeSVGNode::create_element_name(
                fill_clipped_shape_id,
                String::from("fill-clipped-shape"),
                false,
            ),
        );
        let fill_clipped_shape_index = base
            .append_child_to(fill_clip_path_index, fill_clipped_shape_element)
            .unwrap();

        let mut fill_element = base.create_element(SVGTag::Group);
        let fill_element_id = fill_element.get_id();
        #[cfg(feature = "trace")]
        fill_element.set_attribute(
            String::from("name"),
            ShapeSVGNode::create_element_name(fill_element_id, String::from("fill"), false),
        );
        fill_element.set_attribute(
            String::from("clipPath"),
            format!("url(#{fill_clip_path_id})"),
        );
        let fill_element_index = base.append_child(fill_element);

        Self {
            base,
            output_event_sender,

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
        format!("shape_{}_{}{}", category, id, def_part)
    }
}
