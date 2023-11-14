use std::sync::mpsc::Sender;

use crate::core::{
    events::output_event::{AttributeUpdated, ElementCreated, OutputEvent, StyleUpdated},
    modules::svg_render::resources::svg_composition::{
        svg_composition::SVGComposition,
        svg_element::{SVGChildElementIdentifier, SVGElement},
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
        let node = BaseSVGNode {
            id: rand::random(),
            element,
            child_elements: vec![],
            output_event_sender,
        };
        node.sync_element_creation(node.get_element(), maybe_parent_element_id);
        return node;
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

    pub fn append_child_element_to(
        &mut self,
        index: usize,
        element: SVGElement,
    ) -> Result<usize, String> {
        let next_index = self.child_elements.len();
        if let Some(target_element) = self.child_elements.get_mut(index) {
            target_element.append_child(SVGChildElementIdentifier::InContext(next_index));
            let target_element_id = target_element.get_id();
            self.sync_element_creation(&element, Some(target_element_id));
            self.child_elements.push(element);
            return Ok(next_index);
        } else {
            return Err("Invalid parent index".to_string());
        }
    }

    pub fn append_child_element(&mut self, element: SVGElement) -> usize {
        let index = self.child_elements.len();
        self.sync_element_creation(&element, Some(self.element.get_id()));
        self.child_elements.push(element);
        self.element
            .append_child(SVGChildElementIdentifier::InContext(index));
        return index;
    }

    pub fn get_child_element_at_mut(&mut self, index: usize) -> Option<&mut SVGElement> {
        self.child_elements.get_mut(index)
    }

    // TODO: Refactor to group attribute changes or something before sending event
    // e.g. Hashmap where changed attributes are collected (last come, will go)
    //      Then in the SVGComposition we keep track of changed nodes
    //      and at the end collect all attribute changes at the end or so
    pub fn set_attributes_at(&mut self, index: usize, attributes: Vec<(String, String)>) {
        if let Some(element) = self.get_child_element_at_mut(index) {
            let mut output_events: Vec<OutputEvent> = vec![];
            for (name, value) in attributes {
                output_events.push(OutputEvent::AttributeUpdated(AttributeUpdated {
                    id: element.get_id(),
                    name: name.clone(),
                    new_value: Some(value.clone()),
                }));
                element.set_attribute(name, value);
            }
            for output_event in output_events {
                let _ = self.output_event_sender.send(output_event);
            }
        }
    }

    pub fn set_attributes(&mut self, attributes: Vec<(String, String)>) {
        let mut output_events: Vec<OutputEvent> = vec![];
        for (name, value) in attributes {
            output_events.push(OutputEvent::AttributeUpdated(AttributeUpdated {
                id: self.element.get_id(),
                name: name.clone(),
                new_value: Some(value.clone()),
            }));
            self.element.set_attribute(name, value);
        }
        for output_event in output_events {
            let _ = self.output_event_sender.send(output_event);
        }
    }

    pub fn set_styles_at(&mut self, index: usize, styles: Vec<(String, String)>) {
        if let Some(element) = self.get_child_element_at_mut(index) {
            let mut output_events: Vec<OutputEvent> = vec![];
            for (name, value) in styles {
                output_events.push(OutputEvent::StyleUpdated(StyleUpdated {
                    id: element.get_id(),
                    name: name.clone(),
                    new_value: Some(value.clone()),
                }));
                element.set_style(name, value);
            }
            for output_event in output_events {
                let _ = self.output_event_sender.send(output_event);
            }
        }
    }

    pub fn set_styles(&mut self, styles: Vec<(String, String)>) {
        let mut output_events: Vec<OutputEvent> = vec![];
        for (name, value) in styles {
            output_events.push(OutputEvent::StyleUpdated(StyleUpdated {
                id: self.element.get_id(),
                name: name.clone(),
                new_value: Some(value.clone()),
            }));
            self.element.set_style(name, value);
        }
        for output_event in output_events {
            let _ = self.output_event_sender.send(output_event);
        }
    }

    pub fn sync_element_creation(&self, element: &SVGElement, parent_id: Option<u32>) {
        let _ = self
            .output_event_sender
            .send(OutputEvent::ElementCreated(ElementCreated {
                id: element.get_id(),
                parent_id,
                tag_name: element.get_tag_name().as_str().to_string(),
                attributes: element.get_attributes().clone(),
                styles: element.get_styles().clone(),
            }));
    }

    pub fn to_string(&self, composition: &SVGComposition) -> String {
        self.element.to_string(&self, composition)
    }
}
