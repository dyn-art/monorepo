use std::collections::HashMap;

use bevy_ecs::entity::Entity;

use self::attributes::SVGAttribute;

use super::{svg_composition::SVGComposition, svg_node::base_svg_node::BaseSVGNode};

pub mod attributes;
pub mod events;
pub mod helper;

// Defines an individual SVG element
#[derive(Debug)]
pub struct SVGElement {
    // Unique identifier for the SVGElement
    id: u32,
    // The type of SVG element (e.g., circle, rect)
    tag_name: SVGTag,
    // Attributes of the SVG element
    attributes: HashMap<&'static str, SVGAttribute>,
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
        let id: u32 = rand::random();
        let id_attribute = SVGAttribute::Id(id);
        SVGElement {
            id,
            tag_name,
            attributes: HashMap::from([(id_attribute.key(), id_attribute)]),
            styles: HashMap::new(),
            children: vec![],
        }
    }

    pub fn set_attribute(&mut self, attribute: SVGAttribute) {
        self.attributes.insert(attribute.key(), attribute);
    }

    pub fn set_attributes(&mut self, attributes: Vec<SVGAttribute>) {
        for attribute in attributes {
            self.set_attribute(attribute);
        }
    }

    pub fn get_attribute(&self, key: &'static str) -> Option<&SVGAttribute> {
        self.attributes.get(key)
    }

    pub fn set_style(&mut self, name: String, value: String) {
        self.styles.insert(name, value);
    }

    pub fn set_styles(&mut self, styles: Vec<(String, String)>) {
        for (name, value) in styles {
            self.set_style(name, value);
        }
    }

    pub fn get_style(&self, key: &str) -> Option<&String> {
        self.styles.get(key)
    }

    pub fn append_child(&mut self, identifier: SVGChildElementIdentifier) {
        self.children.push(identifier);
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_tag_name(&self) -> &SVGTag {
        &self.tag_name
    }

    pub fn get_attributes(&self) -> Vec<SVGAttribute> {
        self.attributes.values().cloned().collect()
    }

    pub fn get_styles(&self) -> &HashMap<String, String> {
        &self.styles
    }

    pub fn to_string(&self, node: &BaseSVGNode, composition: &SVGComposition) -> String {
        // Start with the opening tag and the tag name
        let mut result = format!("<{}", self.tag_name.as_str());

        // Append attributes from the hash map, including 'id'
        for (key, value) in &self.attributes {
            result.push_str(&format!(" {}=\"{}\"", key, value.to_svg_string()));
        }

        // Append styles as a single 'style' attribute
        if !self.styles.is_empty() {
            let style_string: String = self
                .styles
                .iter()
                .map(|(key, value)| format!("{}: {}", key, value))
                .collect::<Vec<String>>()
                .join("; ");
            result.push_str(&format!(" style=\"{}\"", style_string));
        }

        // Add the closing bracket of the opening tag
        result.push('>');

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
    pub fn as_str(&self) -> &'static str {
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
