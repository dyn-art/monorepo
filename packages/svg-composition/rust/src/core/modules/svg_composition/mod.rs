use std::{collections::HashMap, sync::mpsc::Sender};

use bevy_ecs::{entity::Entity, system::Resource};

use crate::core::events::output_event::OutputEvent;

#[derive(Resource, Debug)]
pub struct SVGCompositionRes {
    svg_composition: SVGComposition,
    output_event_sender: Sender<OutputEvent>,
}

#[derive(Debug, Default)]
pub struct SVGNodeUpdate {
    pub changed_attributes: HashMap<String, String>,
    pub changed_styles: HashMap<String, String>,
}

#[derive(Debug)]
pub struct SVGComposition {
    pub nodes: HashMap<Entity, SVGNode>,
    pub root: Entity,
    pub updated: HashMap<u32, SVGNodeUpdate>,
}

// Represents a node in the SVG structure, corresponding to an ECS entity
#[derive(Debug)]
pub struct SVGNode {
    // Unique identifier for the SVGNode
    pub id: u32,
    // The primary SVG element associated with this node
    pub element: SVGElement,
    // Children that are directly related to this node's context
    pub child_elements: HashMap<u32, SVGElement>,
}

// Defines an individual SVG element
#[derive(Debug)]
pub struct SVGElement {
    // Unique identifier for the SVGElement
    pub id: u32,
    // The type of SVG element (e.g., circle, rect)
    pub tag_name: SVGTagName,
    // Attributes of the SVG element
    pub attributes: HashMap<String, String>,
    // Style properties of the SVG element
    pub styles: HashMap<String, String>,
    // Identifiers for child elements, supporting both in-context and out-of-context children
    pub children: Vec<SVGChildElementIdentifier>,
}

#[derive(Debug)]
pub enum SVGChildElementIdentifier {
    // Child element is within the same SVGNode context
    InContext(u32),
    // Child element is managed by a different entity (and potentially different SVGNode)
    OutContext(Entity, Option<u32>),
}

impl SVGComposition {
    pub fn new(root: Entity) -> Self {
        SVGComposition {
            root,
            nodes: HashMap::new(),
            updated: HashMap::new(),
        }
    }

    pub fn to_string(&self) -> String {
        self.nodes.get(&self.root).unwrap().to_string(&self)
    }
}

impl SVGNode {
    pub fn new(id: u32, element: SVGElement) -> Self {
        SVGNode {
            id,
            element,
            child_elements: HashMap::new(),
        }
    }

    pub fn to_string(&self, composition: &SVGComposition) -> String {
        self.element.to_string(&self, composition)
    }
}

impl SVGElement {
    pub fn new(id: u32, tag_name: SVGTagName) -> Self {
        SVGElement {
            id,
            tag_name,
            attributes: HashMap::new(),
            styles: HashMap::new(),
            children: vec![],
        }
    }

    pub fn set_attribute(&mut self, name: String, value: String) {
        self.attributes.insert(name, value);
    }

    pub fn get_attribute(&self, name: &str) -> Option<&String> {
        self.attributes.get(name)
    }

    pub fn set_style(&mut self, name: String, value: String) {
        self.styles.insert(name, value);
    }

    pub fn get_style(&self, name: &str) -> Option<&String> {
        self.styles.get(name)
    }

    pub fn to_string(&self, node: &SVGNode, composition: &SVGComposition) -> String {
        // Start with the opening tag
        let mut result = format!("<{} id=\"{}\"", self.tag_name.as_str(), self.id);

        // Append attributes and styles...
        // ...

        // Handle children
        for child in &self.children {
            match child {
                SVGChildElementIdentifier::InContext(child_id) => {
                    if let Some(child_element) = node.child_elements.get(child_id) {
                        result.push_str(&child_element.to_string(node, composition));
                    }
                }
                SVGChildElementIdentifier::OutContext(entity, _) => {
                    if let Some(child_element) = composition.nodes.get(entity) {
                        result.push_str(&child_element.to_string(composition));
                    }
                }
            }
        }

        // Close the tag
        result.push_str(&format!("</{}>", self.tag_name.as_str()));

        return result;
    }
}

#[derive(Debug)]
pub enum SVGTagName {
    Circle,
    Rect,
    Path,
    Line,
    Ellipse,
    Polygon,
    Polyline,
    Text,
    Group,
}

impl SVGTagName {
    fn as_str(&self) -> &'static str {
        match self {
            SVGTagName::Circle => "circle",
            SVGTagName::Rect => "rect",
            SVGTagName::Path => "path",
            SVGTagName::Line => "line",
            SVGTagName::Ellipse => "ellipse",
            SVGTagName::Polygon => "polygon",
            SVGTagName::Polyline => "polyline",
            SVGTagName::Text => "text",
            SVGTagName::Group => "g",
        }
    }
}
