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
                    let mut base = self.get_base_mut();
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

    fn to_string(&self, composition: &SVGComposition) -> String {
        self.base.to_string(&composition)
    }
}

impl FrameSVGNode {
    pub fn new() -> Self {
        // Create root element and apply it to SVG node
        let element = SVGElement::new(SVGTag::Group);
        let mut base = BaseSVGNode::new(element);

        // Create content elements
        let content_wrapper = SVGElement::new(SVGTag::Group);
        let content_wrapper_id = content_wrapper.get_id();
        let content_wrapper_index = base.append_child(content_wrapper);

        // Create fill elements
        let fill_clip_path_defs = SVGElement::new(SVGTag::Defs);
        let fill_clip_path_defs_id = fill_clip_path_defs.get_id();
        let fill_clip_path_defs_index = base.append_child(fill_clip_path_defs);

        let fill_clip_path_element = SVGElement::new(SVGTag::Defs);
        let fill_clip_path_id = fill_clip_path_element.get_id();
        let fill_clip_path_index = base
            .append_child_to(fill_clip_path_defs_index, fill_clip_path_element)
            .unwrap();

        let fill_clipped_shape_element = SVGElement::new(SVGTag::Path);
        let fill_clipped_shape_id = fill_clipped_shape_element.get_id();
        let fill_clipped_shape_index = base
            .append_child_to(fill_clip_path_index, fill_clipped_shape_element)
            .unwrap();

        let mut fill_element = SVGElement::new(SVGTag::Group);
        fill_element.set_attribute(
            String::from("clipPath"),
            format!("url(#{fill_clip_path_id})"),
        );
        let fill_element_id = fill_element.get_id();
        let fill_element_index = base.append_child(fill_element);

        Self {
            base,
            content_wrapper: ElementReference {
                index: content_wrapper_index,
                id: content_wrapper_id,
            },
            fill_clip_path_defs: ElementReference {
                index: fill_clip_path_defs_index,
                id: fill_clip_path_defs_id,
            },
            fill_clip_path: ElementReference {
                index: fill_clip_path_index,
                id: fill_clip_path_id,
            },
            fill_clipped_shape: ElementReference {
                index: fill_clipped_shape_index,
                id: fill_clipped_shape_id,
            },
            fill_element: ElementReference {
                index: fill_element_index,
                id: fill_element_id,
            },
        }
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
                    let mut base = self.get_base_mut();
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

    fn to_string(&self, composition: &SVGComposition) -> String {
        self.base.to_string(&composition)
    }
}

impl ShapeSVGNode {
    pub fn new() -> Self {
        // Create root element and apply it to SVG node
        let element = SVGElement::new(SVGTag::Group);
        let mut base = BaseSVGNode::new(element);

        // Create fill elements
        let fill_clip_path_defs = SVGElement::new(SVGTag::Defs);
        let fill_clip_path_defs_id = fill_clip_path_defs.get_id();
        let fill_clip_path_defs_index = base.append_child(fill_clip_path_defs);

        let fill_clip_path_element = SVGElement::new(SVGTag::Defs);
        let fill_clip_path_id = fill_clip_path_element.get_id();
        let fill_clip_path_index = base
            .append_child_to(fill_clip_path_defs_index, fill_clip_path_element)
            .unwrap();

        let fill_clipped_shape_element = SVGElement::new(SVGTag::Path);
        let fill_clipped_shape_id = fill_clipped_shape_element.get_id();
        let fill_clipped_shape_index = base
            .append_child_to(fill_clip_path_index, fill_clipped_shape_element)
            .unwrap();

        let mut fill_element = SVGElement::new(SVGTag::Group);
        fill_element.set_attribute(
            String::from("clipPath"),
            format!("url(#{fill_clip_path_id})"),
        );
        let fill_element_id = fill_element.get_id();
        let fill_element_index = base.append_child(fill_element);

        Self {
            base,
            fill_clip_path_defs: ElementReference {
                index: fill_clip_path_defs_index,
                id: fill_clip_path_defs_id,
            },
            fill_clip_path: ElementReference {
                index: fill_clip_path_index,
                id: fill_clip_path_id,
            },
            fill_clipped_shape: ElementReference {
                index: fill_clipped_shape_index,
                id: fill_clipped_shape_id,
            },
            fill_element: ElementReference {
                index: fill_element_index,
                id: fill_element_id,
            },
        }
    }
}
