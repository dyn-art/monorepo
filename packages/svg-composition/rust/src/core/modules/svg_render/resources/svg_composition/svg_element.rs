use std::collections::HashMap;

use bevy_ecs::entity::Entity;

use super::{svg_composition::SVGComposition, svg_node::BaseSVGNode};

// Defines an individual SVG element
#[derive(Debug)]
pub struct SVGElement {
    // Unique identifier for the SVGElement
    id: u32,
    // The type of SVG element (e.g., circle, rect)
    tag_name: SVGTag,
    // Attributes of the SVG element
    attributes: HashMap<String, String>,
    // Style properties of the SVG element
    styles: HashMap<String, String>,
    // Identifiers for child elements, supporting both in-context and out-of-context children
    children: Vec<SVGChildElementIdentifier>,
}

#[derive(Debug)]
pub enum SVGChildElementIdentifier {
    // Child element is within the same SVGNode context (query by index)
    InContext(usize),
    // Child element belongs to a different entity (query by entity)
    OutOfContext(Entity),
}

impl SVGElement {
    pub fn new(tag_name: SVGTag) -> Self {
        SVGElement {
            id: rand::random(),
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

    pub fn append_child(&mut self, identifier: SVGChildElementIdentifier) {
        self.children.push(identifier);
    }

    pub fn to_string(&self, node: &BaseSVGNode, composition: &SVGComposition) -> String {
        // Start with the opening tag
        let mut result = format!("<{} id=\"{}\"", self.tag_name.as_str(), self.id);

        // TODO Append attributes and styles..

        // Handle children
        for child in &self.children {
            match child {
                SVGChildElementIdentifier::InContext(child_index) => {
                    if let Some(child_element) = node.get_children().get(*child_index) {
                        result.push_str(&child_element.to_string(node, composition));
                    }
                }
                SVGChildElementIdentifier::OutOfContext(entity) => {
                    if let Some(child_element) = composition.get_node(entity) {
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
pub enum SVGTag {
    Circle,
    Rect,
    Path,
    Line,
    Ellipse,
    Polygon,
    Polyline,
    Text,
    Group,
    Defs,
    ClipPath,
}

impl SVGTag {
    fn as_str(&self) -> &'static str {
        match self {
            SVGTag::Circle => "circle",
            SVGTag::Rect => "rect",
            SVGTag::Path => "path",
            SVGTag::Line => "line",
            SVGTag::Ellipse => "ellipse",
            SVGTag::Polygon => "polygon",
            SVGTag::Polyline => "polyline",
            SVGTag::Text => "text",
            SVGTag::Group => "g",
            SVGTag::Defs => "defs",
            SVGTag::ClipPath => "clipPath",
        }
    }
}
