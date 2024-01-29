use std::collections::HashMap;

use bevy_ecs::entity::Entity;
use dyn_composition::utils::continuous_id::ContinuousId;

#[cfg(feature = "output-event")]
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
    // Identifier to identify element in SVG context.
    identifier: SVGElementChildIdentifier,
    /// The type of SVG element (e.g., circle, rect)
    tag: SVGTag,
    /// The attributes of the SVG element
    attributes: HashMap<&'static str, SVGAttribute>,
    /// The style properties of the SVG element
    styles: HashMap<&'static str, SVGStyle>,
    /// Identifiers to identify child elements in SVG context.
    children: Vec<SVGElementChild>,
    /// Identifier to identify parent element in SVG context.
    parent: Option<SVGElementChild>,
    /// Render change updates
    #[cfg(feature = "output-event")]
    changes: Vec<ElementChange>,
    /// Whether the element was created in the current update cycle (before first update drain).
    #[cfg(feature = "output-event")]
    was_created_in_current_update_cycle: bool,
}

impl SVGElement {
    pub fn new_as_bundle_root(tag: SVGTag, entity: Entity, id: ContinuousId) -> Self {
        Self::new_internal(tag, true, entity, id)
    }

    pub fn new(tag: SVGTag, entity: Entity, id: ContinuousId) -> Self {
        Self::new_internal(tag, false, entity, id)
    }

    fn new_internal(tag: SVGTag, is_bundle_root: bool, entity: Entity, id: ContinuousId) -> Self {
        let id_attribute = SVGAttribute::Id { id };
        let inital_attributes: HashMap<&'static str, SVGAttribute> =
            HashMap::from([(id_attribute.key(), id_attribute)]);
        let intial_styles: HashMap<&'static str, SVGStyle> = HashMap::new();
        #[cfg(feature = "output-event")]
        let initial_changes = vec![ElementChange::ElementCreated(ElementCreated {
            parent_id: None,
            tag_name: tag.as_str(),
            attributes: inital_attributes.values().cloned().collect(),
            styles: intial_styles.values().cloned().collect(),
            is_bundle_root,
            entity,
        })];

        return Self {
            id,
            identifier: if is_bundle_root {
                SVGElementChildIdentifier::InSVGContext(entity)
            } else {
                SVGElementChildIdentifier::InSVGBundleContext(entity)
            },
            tag,
            attributes: inital_attributes,
            styles: intial_styles,
            children: Vec::new(),
            parent: None,
            #[cfg(feature = "output-event")]
            changes: initial_changes,
            #[cfg(feature = "output-event")]
            was_created_in_current_update_cycle: true,
        };
    }

    pub fn get_id(&self) -> ContinuousId {
        self.id
    }

    pub fn get_tag(&self) -> &SVGTag {
        &self.tag
    }

    // =========================================================================
    // Attributes
    // =========================================================================

    pub fn set_attribute(&mut self, attribute: SVGAttribute) {
        #[cfg(feature = "output-event")]
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

    // =========================================================================
    // Styles
    // =========================================================================

    pub fn set_style(&mut self, style: SVGStyle) {
        #[cfg(feature = "output-event")]
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

    // =========================================================================
    // Children
    // =========================================================================

    pub fn get_parent(&self) -> &Option<SVGElementChild> {
        &self.parent
    }

    pub fn append_child_in_svg_context(&mut self, entity: Entity, child_element: &mut SVGElement) {
        self.append_child_element(
            child_element,
            SVGElementChildIdentifier::InSVGContext(entity),
        )
    }

    pub fn append_child_in_bundle_context(
        &mut self,
        entity: Entity,
        child_element: &mut SVGElement,
    ) {
        self.append_child_element(
            child_element,
            SVGElementChildIdentifier::InSVGBundleContext(entity),
        );
    }

    fn append_child_element(
        &mut self,
        child_element: &mut SVGElement,
        identifier: SVGElementChildIdentifier,
    ) {
        self.children.push(SVGElementChild {
            id: child_element.get_id(),
            identifier,
        });
        child_element.append_to_parent(self.id, self.identifier);
    }

    pub fn append_to_parent(
        &mut self,
        parent_id: ContinuousId,
        parent_identifier: SVGElementChildIdentifier,
    ) {
        self.parent = Some(SVGElementChild {
            id: parent_id,
            identifier: parent_identifier,
        });

        // Attempt to set the parent id of the first 'ElementCreated' render change for the element.
        // This ensures the element is correctly attached to its parent during the initial rendering.
        #[cfg(feature = "output-event")]
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

    pub fn remove_child(&mut self, id: ContinuousId) {
        log::info!("[remove_child] By {:?} - {:?}", id, self.children);
        self.children.retain(|child| child.id != id);
    }

    pub fn clear_children(&mut self) {
        self.children.clear()
    }

    // =========================================================================
    // Other
    // =========================================================================

    /// Removes an SVG element from its parent in the hierarchy.
    /// The `destroyed_by` bundle is passed explicitly to handle cases where the element's
    /// immediate parent is not in the current SVG context. This is particularly relevant
    /// when updates are being applied to the parent, necessitating its temporary removal
    /// to allow a mutable reference to the SVGContext.
    pub fn destroy(&mut self, cx: &mut SVGContext, destroyed_by: &mut dyn SVGBundle, soft: bool) {
        log::info!(
            "[destroy] Parent: {:?}, Destroyed by: {:?}",
            self.parent,
            destroyed_by.get_entity()
        );

        // Remove element from potential parent element
        if !soft {
            if let Some(parent) = &self.parent {
                match parent.identifier {
                    SVGElementChildIdentifier::InSVGBundleContext(entity) => {
                        if entity == *destroyed_by.get_entity() {
                            if let Some(element) =
                                destroyed_by.get_child_elements_mut().get_mut(&parent.id)
                            {
                                element.remove_child(self.get_id());
                            }
                        } else {
                            if let Some(bundle) = cx.get_bundle_mut(&entity) {
                                if let Some(element) =
                                    bundle.get_child_elements_mut().remove(&parent.id)
                                {
                                    element.remove_child(self.get_id());
                                }
                            }
                        }
                    }
                    SVGElementChildIdentifier::InSVGContext(entity) => {
                        if entity == *destroyed_by.get_entity() {
                            destroyed_by
                                .get_root_element_mut()
                                .remove_child(self.get_id());
                        } else {
                            if let Some(bundle) = cx.get_bundle_mut(&entity) {
                                bundle.get_root_element_mut().remove_child(self.get_id());
                            }
                        }
                    }
                };
            }
        }

        #[cfg(feature = "output-event")]
        self.changes
            .push(ElementChange::ElementDeleted(ElementDeleted {}));
    }

    #[cfg(feature = "output-event")]
    pub fn drain_changes(&mut self) -> Vec<ElementChange> {
        self.was_created_in_current_update_cycle = false;
        self.changes.drain(..).collect()
    }

    pub fn to_string(&self, bundle: &dyn SVGBundle, cx: &SVGContext) -> String {
        let mut result = String::new();

        // Open SVG tag
        {
            result.push_str(&format!("<{}", self.tag.as_str()));

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
            match child.identifier {
                SVGElementChildIdentifier::InSVGBundleContext(_) => {
                    if let Some(child_element) = bundle.get_child_elements().get(&child.id) {
                        result.push_str(&child_element.to_string(bundle, cx));
                    }
                }
                SVGElementChildIdentifier::InSVGContext(entity) => {
                    if let Some(bundle) = cx.get_bundle(&entity) {
                        result.push_str(&bundle.to_string(cx));
                    }
                }
            }
        }

        // Close SVG tag
        result.push_str(&format!("</{}>", self.tag.as_str()));

        return result;
    }
}

#[derive(Debug)]
pub struct SVGElementChild {
    pub id: ContinuousId,
    pub identifier: SVGElementChildIdentifier,
}

#[derive(Debug, Clone, Copy)]
pub enum SVGElementChildIdentifier {
    /// Child element is root element of SVGBundle.
    InSVGContext(Entity),
    /// Child element is child element of SVGBundle.
    InSVGBundleContext(Entity),
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
