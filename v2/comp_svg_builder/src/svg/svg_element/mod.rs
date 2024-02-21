use std::{collections::HashMap, fmt::Display};

use bevy_ecs::{component::Component, entity::Entity};

use self::{
    attributes::SVGAttribute,
    element_changes::{
        SVGAttributeUpdatedChange, SVGElementChange, SVGElementCreatedChange,
        SVGElementDeletedChange, SVGStyleUpdatedChange,
    },
    styles::SVGStyle,
};

pub mod attributes;
pub mod element_changes;
pub mod styles;

#[derive(Component, Debug, Clone)]
pub struct SVGElement {
    /// Unique identifier of the SVGElement
    id: SVGElementId,
    /// The type of SVG element (e.g., circle, rect).
    tag: &'static str,
    /// The attributes of the SVG element.
    attributes: HashMap<&'static str, SVGAttribute>,
    /// The style properties of the SVG element.
    styles: HashMap<&'static str, SVGStyle>,
    /// Children of the SVG element in the SVG tree.
    children: Vec<SVGElementChild>,
    /// Render change updates
    #[cfg(feature = "output_events")]
    changes: Vec<SVGElementChange>,
    /// Whether the element was created in the current update cycle (before first update drain).
    #[cfg(feature = "output_events")]
    was_created_in_current_update_cycle: bool,
}

impl SVGElement {
    pub fn new(tag: &'static str, id: SVGElementId) -> Self {
        let id_attribute = SVGAttribute::Id { id };
        let inital_attributes: HashMap<&'static str, SVGAttribute> =
            HashMap::from([(id_attribute.key(), id_attribute)]);
        let inital_styles: HashMap<&'static str, SVGStyle> = HashMap::new();

        return Self {
            id,
            tag,
            attributes: inital_attributes,
            styles: inital_styles,
            children: Vec::new(),
            #[cfg(feature = "output_events")]
            changes: Vec::new(),
            #[cfg(feature = "output_events")]
            was_created_in_current_update_cycle: true,
        };
    }

    pub fn get_id(&self) -> SVGElementId {
        self.id
    }

    pub fn get_tag(&self) -> &'static str {
        &self.tag
    }

    // =========================================================================
    // Attributes
    // =========================================================================

    pub fn set_attribute(&mut self, attribute: SVGAttribute) {
        #[cfg(feature = "output_events")]
        self.register_change(SVGElementChange::AttributeUpdated(
            SVGAttributeUpdatedChange {
                key: attribute.key(),
                new_value: attribute.into_svg_string(),
            },
        ));

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

    // =========================================================================
    // Styles
    // =========================================================================

    pub fn set_style(&mut self, style: SVGStyle) {
        #[cfg(feature = "output_events")]
        self.register_change(SVGElementChange::StyleUpdated(SVGStyleUpdatedChange {
            key: style.key(),
            new_value: style.into_svg_string(),
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

    // =========================================================================
    // Children
    // =========================================================================

    pub fn append_child_in_world_context(
        &mut self,
        entity: Entity,
        child_element: &mut SVGElement,
    ) {
        self.append_child_element(
            child_element,
            SVGElementChildIdentifier::InWorldContext(entity),
        )
    }

    pub fn append_child_in_node_context(&mut self, entity: Entity, child_element: &mut SVGElement) {
        self.append_child_element(
            child_element,
            SVGElementChildIdentifier::InSVGNodeContext(entity),
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
        #[cfg(feature = "output_events")]
        child_element.append_to_parent(self.id);
    }

    #[cfg(feature = "output_events")]
    pub fn append_to_parent(&mut self, parent_id: SVGElementId) {
        self.register_change(SVGElementChange::ElementAppended(
            self::element_changes::SVGElementAppendedChange { parent_id },
        ));
    }

    pub fn remove_child(&mut self, id: SVGElementId) {
        self.children.retain(|child| child.id != id);
    }

    pub fn clear_children(&mut self) {
        self.children.clear()
    }

    // =========================================================================
    // Other
    // =========================================================================

    #[cfg(feature = "output_events")]
    pub fn init(&mut self, entity: Option<Entity>) {
        self.register_change(SVGElementChange::ElementCreated(SVGElementCreatedChange {
            parent_id: None,
            tag_name: self.tag,
            attributes: self
                .attributes
                .values()
                .map(|value| value.into_tuple())
                .collect(),
            styles: self
                .styles
                .values()
                .map(|value| value.into_tuple())
                .collect(),
            entity,
        }));
    }

    #[cfg(feature = "output_events")]
    fn register_change(&mut self, element_change: SVGElementChange) {
        if self.was_created_in_current_update_cycle {
            if let Some(update) = self.changes.first_mut() {
                match update {
                    SVGElementChange::ElementCreated(element_created_event) => match element_change
                    {
                        SVGElementChange::AttributeUpdated(event) => {
                            element_created_event
                                .attributes
                                .push((event.key, event.new_value));
                            return;
                        }
                        SVGElementChange::StyleUpdated(event) => {
                            element_created_event
                                .styles
                                .push((event.key, event.new_value));
                            return;
                        }
                        SVGElementChange::ElementAppended(event) => {
                            element_created_event.parent_id = Some(event.parent_id);
                            return;
                        }
                        SVGElementChange::ElementDeleted(event) => {
                            self.changes.clear();
                            return;
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
        self.changes.push(element_change);
    }

    /// Destroys this SVG element.
    /// This method only handles the destruction of the element itself.
    /// It is the responsibility of the caller to ensure that any references to this element are properly managed.
    #[cfg(feature = "output_events")]
    pub fn destroy(&mut self) {
        self.register_change(SVGElementChange::ElementDeleted(SVGElementDeletedChange {}));
    }

    #[cfg(feature = "output_events")]
    pub fn drain_changes(&mut self) -> Vec<SVGElementChange> {
        self.was_created_in_current_update_cycle = false;
        self.changes.drain(..).collect()
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct SVGElementId(usize);

impl SVGElementId {
    pub const ZERO: SVGElementId = SVGElementId(0);

    pub fn next_id(&mut self) -> Self {
        let old = self.0;
        self.0 += 1;
        Self(old)
    }
}

impl Display for SVGElementId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SVGElementChild {
    pub id: SVGElementId,
    pub identifier: SVGElementChildIdentifier,
}

#[derive(Debug, Copy, Clone)]
pub enum SVGElementChildIdentifier {
    /// Child element is root element of SVGNode.
    InWorldContext(Entity),
    /// Child element is child element of SVGNode.
    InSVGNodeContext(Entity),
}
