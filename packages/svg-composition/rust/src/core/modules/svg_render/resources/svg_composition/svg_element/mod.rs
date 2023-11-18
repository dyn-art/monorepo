use std::collections::HashMap;

use bevy_ecs::entity::Entity;

use self::{attributes::SVGAttribute, styles::SVGStyle};

use super::{svg_node::SVGNode, SVGComposition};

pub mod attributes;
pub mod events;
pub mod helper;
pub mod mapper;
pub mod styles;

/// Defines an individual SVG element
#[derive(Debug)]
pub struct SVGElement {
    // Unique identifier of the SVGElement
    id: u32,
    // The type of SVG element (e.g., circle, rect)
    tag_name: SVGTag,
    // Attributes of the SVG element
    attributes: HashMap<&'static str, SVGAttribute>,
    // Style properties of the SVG element
    styles: HashMap<&'static str, SVGStyle>,
    // Identifiers for child elements, supporting both in-context and out-of-context children
    children: Vec<SVGChildElementIdentifier>,
}

#[derive(Debug)]
pub enum SVGChildElementIdentifier {
    // Child element is within the same SVGNode context (query by index in "child_elements")
    InNodeContext(usize),
    // Child element belongs to a different entity (query by entity in "nodes")
    OutOfNodeContext(Entity),
    // Child element belongs to fill in the same SVGNode
    Fill,
    // Child element belongs to paint in the same SVGNode (query by index in "paints")
    InFillContext(usize),
    // Child element belongs to paint in the same SVGNode (query by index in "paints")
    InPaintContext(usize, usize), // paints vec index in fill & child element index in paint
}

impl SVGElement {
    pub fn new(tag_name: SVGTag) -> Self {
        let id: u32 = rand::random();
        let id_attribute = SVGAttribute::Id { id };
        SVGElement {
            id,
            tag_name,
            attributes: HashMap::from([(id_attribute.key(), id_attribute)]),
            styles: HashMap::new(),
            children: vec![],
        }
    }

    // =============================================================================
    // Getter & Setter
    // =============================================================================

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

    pub fn get_attributes(&self) -> Vec<SVGAttribute> {
        self.attributes.values().cloned().collect()
    }

    pub fn set_style(&mut self, style: SVGStyle) {
        self.styles.insert(style.key(), style);
    }

    pub fn set_styles(&mut self, styles: Vec<SVGStyle>) {
        for style in styles {
            self.set_style(style);
        }
    }

    pub fn get_style(&self, key: &'static str) -> Option<&SVGStyle> {
        self.styles.get(key)
    }

    pub fn get_styles(&self) -> Vec<SVGStyle> {
        self.styles.values().cloned().collect()
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_tag_name(&self) -> &SVGTag {
        &self.tag_name
    }

    // =============================================================================
    // Children
    // =============================================================================

    pub fn append_child(&mut self, identifier: SVGChildElementIdentifier) {
        self.children.push(identifier);
    }

    // =============================================================================
    // Other
    // =============================================================================

    pub fn to_string(&self, node: &dyn SVGNode, composition: &SVGComposition) -> String {
        let mut result = String::new();

        // Open the SVG tag
        {
            result.push_str(&format!("<{}", self.tag_name.as_str()));

            // Append attributes
            for (key, value) in &self.attributes {
                result.push_str(&format!(" {}=\"{}\"", key, value.to_svg_string()));
            }

            // Append styles as a single 'style' attribute
            if !self.styles.is_empty() {
                let style_string: String = self
                    .styles
                    .iter()
                    .map(|(key, value)| format!("{}: {}", key, value.to_svg_string()))
                    .collect::<Vec<String>>()
                    .join("; ");
                result.push_str(&format!(" style=\"{}\"", style_string));
            }

            result.push('>');
        }

        // Append children
        for child in &self.children {
            match child {
                SVGChildElementIdentifier::InNodeContext(child_index) => {
                    if let Some(child_element) = node.get_bundle().get_children().get(*child_index)
                    {
                        result.push_str(&child_element.to_string(node, composition));
                    }
                }
                SVGChildElementIdentifier::OutOfNodeContext(entity) => {
                    if let Some(child_element) = composition.get_node(entity) {
                        result.push_str(&child_element.to_string(composition));
                    }
                }
                SVGChildElementIdentifier::Fill => {
                    if let Some(fill) = node.get_fill() {
                        result.push_str(&fill.to_string(node, composition))
                    }
                }
                _ => {} // TODO
            }
        }

        // Close the SVG tag
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
