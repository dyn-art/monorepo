use bevy_ecs::entity::Entity;

use crate::core::modules::svg_render::mixin_change::MixinChange;

use super::{
    svg_composition::SVGComposition,
    svg_element::{SVGChildElementIdentifier, SVGElement, SVGTag},
};
use std::fmt::Debug;

#[derive(Debug)]
pub struct ElementReference {
    id: u32,
    index: usize,
}

pub trait SVGNode: Sync + Send + Debug {
    fn get_base(&self) -> &BaseSVGNode;
    fn get_base_mut(&mut self) -> &mut BaseSVGNode;
    fn to_string(&self, composition: &SVGComposition) -> String;
    fn apply_mixin_changes(&mut self, changes: &[MixinChange]) -> ();
    fn append_external_child(&mut self, entity: Entity) -> ();
}

// =============================================================================
// Base SVG Node
// =============================================================================

// Represents a node in the SVG structure, corresponding to an ECS entity
#[derive(Debug)]
pub struct BaseSVGNode {
    // Unique identifier for the SVGNode
    id: u32,
    // The primary SVG element associated with this node
    element: SVGElement,
    // Children that are directly related to this node's context
    child_elements: Vec<SVGElement>,
}

impl BaseSVGNode {
    pub fn new(element: SVGElement) -> Self {
        BaseSVGNode {
            id: rand::random(),
            element,
            child_elements: vec![],
        }
    }

    pub fn get_children(&self) -> &Vec<SVGElement> {
        &self.child_elements
    }

    pub fn get_element(&self) -> &SVGElement {
        &self.element
    }

    pub fn append_child_to(&mut self, index: usize, element: SVGElement) -> Result<usize, String> {
        let next_index = self.child_elements.len();
        if let Some(target_element) = self.child_elements.get_mut(index) {
            target_element.append_child(SVGChildElementIdentifier::InContext(next_index));
            self.child_elements.push(element);
            return Ok(next_index);
        } else {
            return Err("Invalid parent index".to_string());
        }
    }

    pub fn append_child(&mut self, element: SVGElement) -> usize {
        let index = self.child_elements.len();
        self.child_elements.push(element);
        self.element
            .append_child(SVGChildElementIdentifier::InContext(index));
        return index;
    }

    pub fn get_child_at(&mut self, index: usize) -> Option<&mut SVGElement> {
        self.child_elements.get_mut(index)
    }

    pub fn to_string(&self, composition: &SVGComposition) -> String {
        self.element.to_string(&self, composition)
    }
}

// =============================================================================
// Frame SVG Node
// =============================================================================
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

                    let base = self.get_base_mut();
                    base.element
                        .set_attribute(String::from("width"), mixin.width.to_string());
                    base.element
                        .set_attribute(String::from("height"), mixin.height.to_string());

                    if let Some(fill_clipped_shape) = base.get_child_at(fill_clipped_shape_index) {
                        fill_clipped_shape
                            .set_attribute(String::from("width"), mixin.width.to_string());
                        fill_clipped_shape
                            .set_attribute(String::from("height"), mixin.height.to_string());
                    }

                    if let Some(content_clipped_shape) =
                        base.get_child_at(content_clipped_shape_index)
                    {
                        content_clipped_shape
                            .set_attribute(String::from("width"), mixin.width.to_string());
                        content_clipped_shape
                            .set_attribute(String::from("height"), mixin.height.to_string());
                    }
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
            .get_child_at(children_wrapper_index)
            .unwrap()
            .append_child(SVGChildElementIdentifier::OutOfContext(entity));
    }

    fn to_string(&self, composition: &SVGComposition) -> String {
        self.base.to_string(composition)
    }
}

impl FrameSVGNode {
    pub fn new() -> Self {
        // Create root element and apply it to SVG node
        let mut element = SVGElement::new(SVGTag::Group);
        #[cfg(feature = "trace")]
        element.set_attribute(
            String::from("name"),
            FrameSVGNode::create_element_name(element.get_id(), String::from("root"), false),
        );
        let mut base = BaseSVGNode::new(element);

        // Create content elements
        // if clips_content {
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
        let content_clip_path_defs_index = base.append_child(content_clip_path_defs_element);

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
            .append_child_to(content_clip_path_defs_index, content_clip_path_element)
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
            .append_child_to(content_clip_path_index, content_clipped_shape_element)
            .unwrap();
        // }

        let mut content_wrapper = SVGElement::new(SVGTag::Group);
        let content_wrapper_id = content_wrapper.get_id();
        #[cfg(feature = "trace")]
        content_wrapper.set_attribute(
            String::from("name"),
            FrameSVGNode::create_element_name(content_wrapper_id, String::from("content"), false),
        );
        // if clips_content {
        content_wrapper.set_attribute(
            String::from("clipPath"),
            format!("url(#{content_clip_path_id})"),
        );
        // }
        let content_wrapper_index = base.append_child(content_wrapper);

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
            .append_child_to(content_wrapper_index, fill_clip_path_defs)
            .unwrap();

        let mut fill_clip_path_element = SVGElement::new(SVGTag::ClipPath);
        let fill_clip_path_id = fill_clip_path_element.get_id();
        #[cfg(feature = "trace")]
        fill_clip_path_element.set_attribute(
            String::from("name"),
            FrameSVGNode::create_element_name(fill_clip_path_id, String::from("fill-clip"), true),
        );
        let fill_clip_path_index = base
            .append_child_to(fill_clip_path_defs_index, fill_clip_path_element)
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
            .append_child_to(fill_clip_path_index, fill_clipped_shape_element)
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
            .append_child_to(content_wrapper_index, fill_element)
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
            .append_child_to(content_wrapper_index, children_wrapper)
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

// =============================================================================
// Shape SVG Node
// =============================================================================

#[derive(Debug)]
pub struct ShapeSVGNode {
    pub base: BaseSVGNode,

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
                    base.element
                        .set_attribute(String::from("width"), mixin.width.to_string());
                    base.element
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
    pub fn new() -> Self {
        // Create root element and apply it to SVG node
        let mut element = SVGElement::new(SVGTag::Group);
        #[cfg(feature = "trace")]
        element.set_attribute(
            String::from("name"),
            ShapeSVGNode::create_element_name(element.get_id(), String::from("root"), false),
        );
        let mut base = BaseSVGNode::new(element);

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
        let fill_clip_path_defs_index = base.append_child(fill_clip_path_defs);

        let mut fill_clip_path_element = SVGElement::new(SVGTag::Defs);
        let fill_clip_path_id = fill_clip_path_element.get_id();
        #[cfg(feature = "trace")]
        fill_clip_path_element.set_attribute(
            String::from("name"),
            ShapeSVGNode::create_element_name(fill_clip_path_id, String::from("fill-clip"), true),
        );
        let fill_clip_path_index = base
            .append_child_to(fill_clip_path_defs_index, fill_clip_path_element)
            .unwrap();

        let mut fill_clipped_shape_element = SVGElement::new(SVGTag::Rect);
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

        let mut fill_element = SVGElement::new(SVGTag::Group);
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
