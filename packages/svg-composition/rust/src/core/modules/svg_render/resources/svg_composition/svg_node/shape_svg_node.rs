use std::sync::mpsc::Sender;

use bevy_ecs::entity::Entity;

use crate::core::{
    events::output_event::OutputEvent,
    modules::svg_render::{
        mixin_change::MixinChange,
        resources::svg_composition::{
            helper::{construct_svg_path, transform_to_css_matrix},
            svg_composition::SVGComposition,
            svg_element::{SVGElement, SVGTag},
        },
    },
};

use super::{base_svg_node::BaseSVGNode, ElementReference, SVGNode};

#[derive(Debug)]
pub struct ShapeSVGNode {
    pub base: BaseSVGNode,

    // Fill elements
    fill_clip_path: ElementReference,
    fill_clip_path_defs: ElementReference,
    fill_clipped_shape: ElementReference,
    fill_wrapper: ElementReference,
    temp_solid_fill: ElementReference, // TODO: REMOVE
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
                    let temp_solid_fill_index = self.temp_solid_fill.index;

                    let width_attr = (String::from("width"), mixin.width.to_string());
                    let height_attr = (String::from("height"), mixin.height.to_string());

                    let base = self.get_base_mut();
                    base.set_attributes(vec![width_attr.clone(), height_attr.clone()]);
                    base.set_attributes_at(
                        temp_solid_fill_index,
                        vec![width_attr.clone(), height_attr.clone()],
                    );
                }
                MixinChange::RelativeTransform(mixin) => {
                    let base = self.get_base_mut();
                    base.set_attributes(vec![(
                        String::from("transform"),
                        transform_to_css_matrix(mixin.relative_transform.0),
                    )]);
                }
                MixinChange::Path(mixin) => {
                    let fill_clipped_shape_index = self.fill_clipped_shape.index;
                    let base = self.get_base_mut();
                    base.set_attributes_at(
                        fill_clipped_shape_index,
                        vec![(String::from("p"), construct_svg_path(&mixin.vertices))],
                    )
                }
                MixinChange::Blend(mixin) => {
                    let base = self.get_base_mut();
                    base.set_attributes(vec![(String::from("opacity"), mixin.opacity.to_string())]);
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
        let mut element = SVGElement::new(SVGTag::Group);
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
        let mut fill_clip_path_defs = SVGElement::new(SVGTag::Defs);
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
        let fill_clip_path_defs_index = base.append_child_element(fill_clip_path_defs);

        let mut fill_clip_path_element = SVGElement::new(SVGTag::Defs);
        let fill_clip_path_id = fill_clip_path_element.get_id();
        #[cfg(feature = "trace")]
        fill_clip_path_element.set_attribute(
            String::from("name"),
            ShapeSVGNode::create_element_name(fill_clip_path_id, String::from("fill-clip"), true),
        );
        let fill_clip_path_index = base
            .append_child_element_to(fill_clip_path_defs_index, fill_clip_path_element)
            .unwrap();

        let mut fill_clipped_shape_element = SVGElement::new(SVGTag::Path);
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
            .append_child_element_to(fill_clip_path_index, fill_clipped_shape_element)
            .unwrap();

        let mut fill_wrapper_element = SVGElement::new(SVGTag::Group);
        let fill_wrapper_id = fill_wrapper_element.get_id();
        #[cfg(feature = "trace")]
        fill_wrapper_element.set_attribute(
            String::from("name"),
            ShapeSVGNode::create_element_name(fill_wrapper_id, String::from("fill"), false),
        );
        fill_wrapper_element.set_attribute(
            String::from("clipPath"),
            format!("url(#{fill_clip_path_id})"),
        );
        let fill_wrapper_index = base.append_child_element(fill_wrapper_element);

        // TODO: REMOVE
        let mut temp_solid_fill_element = SVGElement::new(SVGTag::Rect);
        let temp_solid_fill_id = temp_solid_fill_element.get_id();
        temp_solid_fill_element.set_attribute(String::from("fill"), String::from("red"));
        let temp_solid_fill_index = base
            .append_child_element_to(fill_wrapper_index, temp_solid_fill_element)
            .unwrap();

        Self {
            base,

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
            fill_wrapper: ElementReference {
                id: fill_wrapper_id,
                index: fill_wrapper_index,
            },
            temp_solid_fill: ElementReference {
                id: temp_solid_fill_id,
                index: temp_solid_fill_index,
            }, // TODO: REMOVE
        }
    }

    #[cfg(feature = "trace")]
    fn create_element_name(id: u32, category: String, is_definition: bool) -> String {
        let def_part = if is_definition { "def" } else { "" };
        format!("shape_{}_{}{}", category, id, def_part)
    }
}
