use std::collections::HashMap;

use bevy_ecs::entity::Entity;

use self::{
    attributes::SVGAttribute,
    events::{AttributeUpdated, ElementAppended, ElementCreated, RenderChange, StyleUpdated},
    styles::SVGStyle,
};

use super::{svg_bundle::BaseSVGBundle, svg_node::SVGNode, SVGComposition};

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
    updates: Vec<RenderChange>,
}

/// Used to efficiently locate SVG child elements within various SVG structures.
///
/// This approach is designed to reduce the reliance on hash map lookups,
/// which can be expensive in terms of performance.
/// Instead, it categorizes SVG child elements based on their location in the SVG structure,
/// allowing for more direct and efficient retrieval.
#[derive(Debug)]
pub enum SVGChildElementIdentifier {
    // Child element is owned by SVGBundle (query by index in "child_elements")
    InBundleContext(usize),
    // Child element is owned by SVGComposition and can be found there
    InCompositionContext(InCompositionContextType),
}

#[derive(Debug)]
pub enum InCompositionContextType {
    // Query by entity id in "nodes"
    Node(Entity),
    // Query by entity id in "paints"
    Paint(Entity),
}

impl SVGElement {
    pub fn new(tag_name: SVGTag) -> Self {
        let id: u32 = rand::random();
        let id_attribute = SVGAttribute::Id { id };
        let inital_attributes: HashMap<&'static str, SVGAttribute> =
            HashMap::from([(id_attribute.key(), id_attribute)]);
        let intial_styles: HashMap<&'static str, SVGStyle> = HashMap::new();
        let initial_updates = vec![RenderChange::ElementCreated(ElementCreated {
            parent_id: None,
            tag_name: tag_name.as_str(),
            attributes: inital_attributes.values().cloned().collect(),
            styles: intial_styles.values().cloned().collect(),
        })];

        return Self {
            id,
            tag_name,
            attributes: inital_attributes,
            styles: intial_styles,
            children: Vec::new(),
            updates: initial_updates,
        };
    }

    // =============================================================================
    // Getter & Setter
    // =============================================================================

    pub fn get_children(&self) -> &Vec<SVGChildElementIdentifier> {
        &self.children
    }

    pub fn set_attribute(&mut self, attribute: SVGAttribute) {
        self.updates
            .push(RenderChange::AttributeUpdated(AttributeUpdated {
                new_value: attribute.clone(),
            }));
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
        self.updates.push(RenderChange::StyleUpdated(StyleUpdated {
            new_value: style.clone(),
        }));
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

    pub fn append_child(
        &mut self,
        element: &mut SVGElement,
        identifier: SVGChildElementIdentifier,
    ) {
        element.append_to_parent(self.id);
        self.children.push(identifier);
    }

    pub fn clear_children(&mut self) {
        self.children.clear()
    }

    // =============================================================================
    // Other
    // =============================================================================

    fn append_to_parent(&mut self, parent_id: u32) {
        let mut updated = false;

        // Attempt to set the parent id of the first 'ElementCreated' render change for the element.
        // This ensures the element is correctly attached to its parent during the initial rendering.
        if let Some(update) = self.updates.first_mut() {
            match update {
                RenderChange::ElementCreated(element_created) => {
                    if element_created.parent_id.is_none() {
                        element_created.parent_id = Some(parent_id);
                        updated = true;
                    }
                }
                _ => {}
            }
        }

        if !updated {
            self.updates
                .push(RenderChange::ElementAppended(ElementAppended { parent_id }))
        }
    }

    pub fn drain_updates(&mut self) -> Vec<RenderChange> {
        self.updates.drain(..).collect()
    }

    pub fn to_string(
        &self,
        bundle: &BaseSVGBundle,
        node: &dyn SVGNode,
        composition: &SVGComposition,
    ) -> String {
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
                SVGChildElementIdentifier::InBundleContext(child_index) => {
                    if let Some(child_element) = bundle.get_children().get(*child_index) {
                        result.push_str(&child_element.to_string(bundle, node, composition));
                    }
                }
                SVGChildElementIdentifier::InCompositionContext(context_type) => match context_type
                {
                    InCompositionContextType::Node(entity) => {
                        if let Some(child_element) = composition.get_node(entity) {
                            result.push_str(&child_element.to_string(composition));
                        }
                    }
                    InCompositionContextType::Paint(entity) => {
                        if let Some(child_element) = composition.get_paint(entity) {
                            result.push_str(&child_element.to_string(node, composition));
                        }
                    }
                },
            }
        }

        // Close the SVG tag
        result.push_str(&format!("</{}>", self.tag_name.as_str()));

        return result;
    }
}

#[derive(Debug, Clone)]
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
