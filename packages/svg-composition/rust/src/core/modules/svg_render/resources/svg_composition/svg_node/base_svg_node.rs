use std::sync::mpsc::Sender;

use bevy_ecs::entity::Entity;

use crate::core::{
    events::output_event::{ElementCreated, OutputEvent},
    modules::svg_render::resources::svg_composition::{
        svg_composition::SVGComposition,
        svg_element::{SVGChildElementIdentifier, SVGElement, SVGTag},
    },
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
    // Sender to enque events for frontend
    output_event_sender: Sender<OutputEvent>,
}

impl BaseSVGNode {
    pub fn new(
        element: SVGElement,
        maybe_parent_element_id: Option<u32>,
        output_event_sender: Sender<OutputEvent>,
    ) -> Self {
        let _ = output_event_sender.send(OutputEvent::ElementCreated(ElementCreated {
            id: element.get_id(),
            parent_id: maybe_parent_element_id,
            tag_name: element.get_tag_name().as_str().to_string(),
            attributes: element.get_attributes().clone(),
            styles: element.get_styles().clone(),
        }));
        BaseSVGNode {
            id: rand::random(),
            element,
            child_elements: vec![],
            output_event_sender,
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
            let _ = self
                .output_event_sender
                .send(OutputEvent::ElementCreated(ElementCreated {
                    id: element.get_id(),
                    parent_id: Some(target_element.get_id()),
                    tag_name: element.get_tag_name().as_str().to_string(),
                    attributes: element.get_attributes().clone(),
                    styles: element.get_styles().clone(),
                }));
            self.child_elements.push(element);
            return Ok(next_index);
        } else {
            return Err("Invalid parent index".to_string());
        }
    }

    pub fn append_child(&mut self, element: SVGElement) -> usize {
        let index = self.child_elements.len();
        let _ = self
            .output_event_sender
            .send(OutputEvent::ElementCreated(ElementCreated {
                id: element.get_id(),
                parent_id: Some(self.element.get_id()),
                tag_name: element.get_tag_name().as_str().to_string(),
                attributes: element.get_attributes().clone(),
                styles: element.get_styles().clone(),
            }));
        self.child_elements.push(element);
        self.element
            .append_child(SVGChildElementIdentifier::InContext(index));
        return index;
    }

    pub fn get_child_at(&mut self, index: usize) -> Option<&mut SVGElement> {
        self.child_elements.get_mut(index)
    }

    pub fn create_element(&self, tag_name: SVGTag) -> SVGElement {
        SVGElement::new(tag_name, self.output_event_sender.clone())
    }

    pub fn to_string(&self, composition: &SVGComposition) -> String {
        self.element.to_string(&self, composition)
    }
}
