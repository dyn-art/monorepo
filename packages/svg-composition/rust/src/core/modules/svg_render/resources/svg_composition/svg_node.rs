use super::{
    svg_composition::SVGComposition,
    svg_element::{SVGChildElementIdentifier, SVGElement, SVGTag},
};
use std::fmt::Debug;

pub trait SVGNode: Sync + Send + Debug {
    fn get_base(&self) -> &BaseSVGNode;
    fn get_base_mut(&mut self) -> &mut BaseSVGNode;
    fn to_string(&self, composition: &SVGComposition) -> String;
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
    pub fn get_children(&self) -> &Vec<SVGElement> {
        &self.child_elements
    }

    pub fn append_child_to(&mut self, index: usize, element: SVGElement) -> usize {
        let next_index = self.child_elements.len();
        {
            let target_element = self.child_elements.get_mut(index).unwrap();
            target_element.append_child(SVGChildElementIdentifier::InContext(next_index));
        }
        self.child_elements.push(element);
        return index;
    }

    pub fn append_child(&mut self, element: SVGElement) -> usize {
        let index = self.child_elements.len();
        self.child_elements.push(element);
        self.element
            .append_child(SVGChildElementIdentifier::InContext(index));
        return index;
    }

    pub fn to_string(&self, composition: &SVGComposition) -> String {
        self.element.to_string(&self, composition)
    }
}

// =============================================================================
// Shape SVG Node
// =============================================================================

#[derive(Debug)]
pub struct ShapeSVGNode {
    pub base: BaseSVGNode,
    fill_clip_path_index: usize,
    fill_clip_path_defs_index: usize,
    fill_clipped_shape_index: usize,
    fill_element_index: usize,
}

impl SVGNode for ShapeSVGNode {
    fn get_base(&self) -> &BaseSVGNode {
        &self.base
    }

    fn get_base_mut(&mut self) -> &mut BaseSVGNode {
        &mut self.base
    }

    fn to_string(&self, composition: &SVGComposition) -> String {
        self.base.to_string(&composition)
    }
}

impl ShapeSVGNode {
    pub fn new() -> Self {
        // Create root element
        let element = SVGElement::new(SVGTag::Group);

        // Create base
        let mut base = BaseSVGNode {
            id: rand::random(),
            element,
            child_elements: vec![],
        };

        let fill_clip_path_defs_element = SVGElement::new(SVGTag::Defs);
        let fill_clip_path_defs_index = base.append_child(fill_clip_path_defs_element);

        let fill_clip_path_element = SVGElement::new(SVGTag::Defs);
        let fill_clip_path_index =
            base.append_child_to(fill_clip_path_defs_index, fill_clip_path_element);

        let fill_clipped_shape_element = SVGElement::new(SVGTag::Path);
        let fill_clipped_shape_index =
            base.append_child_to(fill_clip_path_index, fill_clipped_shape_element);

        // TODO: append fill element

        Self {
            base,
            fill_clip_path_defs_index,
            fill_clip_path_index,
            fill_clipped_shape_index,
            fill_element_index: 0,
        }
    }
}
