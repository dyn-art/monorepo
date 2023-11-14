use crate::core::modules::svg_render::resources::svg_composition::{
    svg_composition::SVGComposition,
    svg_element::{SVGChildElementIdentifier, SVGElement},
};

// Represents a node in the SVG structure, corresponding to an ECS entity
#[derive(Debug)]
pub struct BaseSVGNode {
    // Unique identifier for the SVGNode
    id: u32,
    // The primary SVG element associated with this node
    element: SVGElement,
    // Children that are directly related to this node's context.
    // Using a Vector for child_elements as:
    // - The size is known at compile time, minimizing dynamic changes.
    // - Offers efficient O(1) access by index, suitable for our use case.
    // - More memory-efficient and simpler than a HashMap for fixed-size collections.
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

    pub fn get_element_mut(&mut self) -> &mut SVGElement {
        &mut self.element
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
