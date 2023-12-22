use std::collections::HashMap;

use bevy_ecs::entity::Entity;

use crate::core::modules::svg_render::render_change::RenderChange;

use self::{
    attributes::SVGAttribute,
    events::{AttributeUpdated, ElementAppended, ElementCreated, StyleUpdated},
    styles::SVGStyle,
};

use super::{svg_bundle::BaseSVGBundle, svg_bundle_variant::bundle_to_string, SVGCompositionRes};

pub mod attributes;
pub mod events;
pub mod helper;
pub mod mapper;
pub mod styles;

/// An individual SVG element
#[derive(Debug)]
pub struct SVGElement {
    /// Unique identifier of the SVGElement
    id: u32,
    /// The type of SVG element (e.g., circle, rect)
    tag_name: SVGTag,
    /// The attributes of the SVG element
    attributes: HashMap<&'static str, SVGAttribute>,
    /// The style properties of the SVG element
    styles: HashMap<&'static str, SVGStyle>,
    /// Identifiers for child elements, supporting both in-context and out-of-context children.
    children: Vec<SVGChildElementIdentifier>,
    /// Render change updates
    updates: Vec<RenderChange>,
    /// Whether the SVG element is the root of a SVG bundle.
    is_bundle_root: bool,
    /// Whether the element was created in the current update cycle (before first update drain).
    was_created_in_current_update_cycle: bool,
}

/// Used to efficiently locate SVG child elements within various SVG structures.
///
/// This approach is designed to reduce the reliance on hash map lookups,
/// which can be expensive in terms of performance.
/// Instead, it categorizes SVG child elements based on their location in the SVG structure,
/// allowing for more direct and efficient retrieval.
#[derive(Debug)]
pub enum SVGChildElementIdentifier {
    /// Child element is owned by SVGBundle (queried by index in "child_elements").
    InBundleContext(Entity, usize),

    /// Child element is owned by SVGComposition and can be found there.
    InCompositionContext(Entity),
}

impl SVGChildElementIdentifier {
    fn entity(&self) -> Entity {
        match self {
            SVGChildElementIdentifier::InBundleContext(entity, _)
            | SVGChildElementIdentifier::InCompositionContext(entity) => *entity,
        }
    }
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
            is_bundle_root: false,
            entity: None,
        })];

        return Self {
            id,
            tag_name,
            attributes: inital_attributes,
            styles: intial_styles,
            children: Vec::new(),
            updates: initial_updates,
            is_bundle_root: false,
            was_created_in_current_update_cycle: true,
        };
    }

    // =========================================================================
    // Getter & Setter
    // =========================================================================

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

    // =========================================================================
    // Children
    // =========================================================================

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

    // TODO
    pub fn reorder_children(&mut self, new_order: &Vec<Entity>) {
        // let mut index_map = BTreeMap::new();

        // // Mapping each Entity to its index in the children vector
        // for (index, child) in self.children.iter().enumerate() {
        //     let entity = child.entity();
        //     index_map.insert(entity, index);
        // }

        // // Process new order to determine target positions and insertions
        // let mut target_positions = Vec::with_capacity(new_order.len());
        // let mut insertions = Vec::new();
        // for entity in new_order {
        //     match index_map.get(entity) {
        //         Some(&index) => target_positions.push(Some(index)),
        //         None => {
        //             // Placeholder for new entities
        //             target_positions.push(None);
        //             insertions.push((
        //                 target_positions.len() - 1,
        //                 SVGChildElementIdentifier::Placeholder(*entity),
        //             ));
        //         }
        //     }
        // }

        // // Insert placeholders
        // for (pos, placeholder) in insertions {
        //     self.children.insert(pos, placeholder);
        // }

        // // Reorder children based on the target positions
        // let mut swap_done = vec![false; self.children.len()];
        // for (new_position, target) in target_positions
        //     .iter()
        //     .enumerate()
        //     .filter_map(|(np, &t)| t.map(|t| (np, t)))
        // {
        //     if swap_done[new_position] || swap_done[target] {
        //         continue;
        //     }
        //     self.children.swap(new_position, target);
        //     swap_done[new_position] = true;
        //     swap_done[target] = true;
        // }

        // // Push an update event if order has changed
        // // if target_positions.iter().any(|&pos| pos.is_none()) || swap_done.iter().any(|&done| done) {
        // //     self.updates.push(RenderChange::OrderChanged);
        // // }

        // info!("reorder_children: {:?}", self.children); // TODO: REMOVE
    }

    fn append_to_parent(&mut self, parent_id: u32) {
        // Attempt to set the parent id of the first 'ElementCreated' render change for the element.
        // This ensures the element is correctly attached to its parent during the initial rendering.
        if self.was_created_in_current_update_cycle {
            if let Some(update) = self.updates.first_mut() {
                match update {
                    RenderChange::ElementCreated(element_created) => {
                        if element_created.parent_id.is_none() {
                            element_created.parent_id = Some(parent_id);
                        }
                    }
                    _ => {}
                }
            }
        } else {
            self.updates
                .push(RenderChange::ElementAppended(ElementAppended { parent_id }))
        }
    }

    // =========================================================================
    // Other
    // =========================================================================

    pub fn define_as_bundle_root(&mut self, entity: Entity) {
        self.is_bundle_root = true;
        if self.was_created_in_current_update_cycle {
            if let Some(update) = self.updates.first_mut() {
                match update {
                    RenderChange::ElementCreated(element_created) => {
                        element_created.is_bundle_root = true;
                        element_created.entity = Some(entity);
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn drain_updates(&mut self) -> Vec<RenderChange> {
        if self.was_created_in_current_update_cycle {
            self.was_created_in_current_update_cycle = false;
        }

        self.updates.drain(..).collect()
    }

    pub fn to_string(&self, bundle: &BaseSVGBundle, composition: &SVGCompositionRes) -> String {
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
                SVGChildElementIdentifier::InBundleContext(_, child_index) => {
                    if let Some(child_element) = bundle.get_children().get(*child_index) {
                        result.push_str(&child_element.to_string(bundle, composition));
                    }
                }
                SVGChildElementIdentifier::InCompositionContext(entity) => {
                    if let Some(bundle) = composition.get_bundle(entity) {
                        result.push_str(&bundle_to_string(&bundle, composition))
                    }
                }
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
