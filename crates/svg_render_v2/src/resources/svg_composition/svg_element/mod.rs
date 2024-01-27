use std::collections::HashMap;

use bevy_ecs::entity::Entity;
use dyn_composition::core::utils::continuous_id::ContinuousId;

use crate::element_change::{
    AttributeUpdated, ElementAppended, ElementChange, ElementCreated, ElementDeleted, StyleUpdated,
};

use self::{attributes::SVGAttribute, styles::SVGStyle};

use super::{svg_bundle::SVGBundle, svg_context::SVGContext};

pub mod attributes;
pub mod mapper;
pub mod styles;

#[derive(Debug)]
pub struct SVGElement {
    /// Unique identifier of the SVGElement
    id: ContinuousId,
    /// The type of SVG element (e.g., circle, rect)
    tag: SVGTag,
    /// The attributes of the SVG element
    attributes: HashMap<&'static str, SVGAttribute>,
    /// The style properties of the SVG element
    styles: HashMap<&'static str, SVGStyle>,
    /// Identifiers for child elements, supporting both in-context and out-of-context children.
    children: Vec<SVGElementChild>,
    /// Render change updates
    changes: Vec<ElementChange>,
    /// Whether the element was created in the current update cycle (before first update drain).
    was_created_in_current_update_cycle: bool,
}

impl SVGElement {
    pub fn new(tag: SVGTag, id: ContinuousId) -> Self {
        let id_attribute = SVGAttribute::Id { id };
        let inital_attributes: HashMap<&'static str, SVGAttribute> =
            HashMap::from([(id_attribute.key(), id_attribute)]);
        let intial_styles: HashMap<&'static str, SVGStyle> = HashMap::new();
        let initial_changes = vec![ElementChange::ElementCreated(ElementCreated {
            parent_id: None,
            tag_name: tag.as_str(),
            attributes: inital_attributes.values().cloned().collect(),
            styles: intial_styles.values().cloned().collect(),
            is_bundle_root: false,
            entity: None,
        })];

        return Self {
            id,
            tag,
            attributes: inital_attributes,
            styles: intial_styles,
            children: Vec::new(),
            changes: initial_changes,
            was_created_in_current_update_cycle: true,
        };
    }

    // =========================================================================
    // Getter & Setter
    // =========================================================================

    pub fn set_attribute(&mut self, attribute: SVGAttribute) {
        self.changes
            .push(ElementChange::AttributeUpdated(AttributeUpdated {
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
        self.changes.push(ElementChange::StyleUpdated(StyleUpdated {
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

    pub fn get_id(&self) -> ContinuousId {
        self.id
    }

    pub fn get_tag(&self) -> &SVGTag {
        &self.tag
    }

    // =========================================================================
    // Children
    // =========================================================================

    pub fn clear_children(&mut self) {
        self.children.clear()
    }

    pub fn append_child_element(
        &mut self,
        child_element: &mut SVGElement,
        identifier: SVGElementChildIdentifier,
    ) {
        child_element.append_to_parent(self.id);
        self.children.push(SVGElementChild {
            id: child_element.get_id(),
            identifier,
        });
    }

    pub fn append_to_parent(&mut self, parent_id: ContinuousId) {
        // Attempt to set the parent id of the first 'ElementCreated' render change for the element.
        // This ensures the element is correctly attached to its parent during the initial rendering.
        if self.was_created_in_current_update_cycle {
            if let Some(update) = self.changes.first_mut() {
                match update {
                    ElementChange::ElementCreated(element_created) => {
                        if element_created.parent_id.is_none() {
                            element_created.parent_id = Some(parent_id);
                        }
                    }
                    _ => {}
                }
            }
        } else {
            self.changes
                .push(ElementChange::ElementAppended(ElementAppended {
                    parent_id,
                }))
        }
    }

    // =========================================================================
    // Remove
    // =========================================================================

    pub fn remove(&mut self) {
        self.changes
            .push(ElementChange::ElementDeleted(ElementDeleted {}));
    }

    // =========================================================================
    // Other
    // =========================================================================

    pub fn drain_changes(&mut self) -> Vec<ElementChange> {
        self.was_created_in_current_update_cycle = false;
        self.changes.drain(..).collect()
    }

    pub fn to_string(&self, bundle: &Box<dyn SVGBundle>, cx: &SVGContext) -> String {
        String::from("todo")
    }
}

#[derive(Debug)]
pub struct SVGElementChild {
    pub id: ContinuousId,
    pub identifier: SVGElementChildIdentifier,
}

#[derive(Debug)]
pub enum SVGElementChildIdentifier {
    InSVGBundleContext(Entity, ContinuousId),
    InSVGContext(Entity),
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
    Pattern,
    Image,
    LinearGradient,
    RadialGradient,
    Stop,
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
            SVGTag::Pattern => "pattern",
            SVGTag::Image => "image",
            SVGTag::LinearGradient => "linearGradient",
            SVGTag::RadialGradient => "radialGradient",
            SVGTag::Stop => "stop",
        }
    }
}
