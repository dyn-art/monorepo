pub mod attributes;
pub mod conversions;
pub mod element_changes;
pub mod styles;

use self::{attributes::SvgAttribute, styles::SvgStyle};
use bevy_ecs::{component::Component, entity::Entity};
use std::{collections::HashMap, fmt::Display};

#[cfg(feature = "output_events")]
use self::element_changes::{
    SvgAttributeUpdatedChange, SvgElementChange, SvgElementCreatedChange, SvgElementDeletedChange,
    SvgStyleUpdatedChange,
};

#[derive(Component, Debug, Clone)]
pub struct SvgElement {
    /// Unique identifier of the SvgElement
    id: SvgElementId,
    /// The type of SvgElement (e.g., circle, rect).
    tag: &'static str,
    /// The attributes of the SvgElement.
    attributes: HashMap<&'static str, SvgAttribute>,
    /// The style properties of the SvgElement.
    styles: HashMap<&'static str, SvgStyle>,
    /// Children of the SvgElement in the Svg tree.
    children: Vec<SvgElementChild>,
    /// Applied changes after last drain.
    #[cfg(feature = "output_events")]
    changes: Vec<SvgElementChange>,
    /// Whether the element was created in the current update cycle (before first update drain).
    #[cfg(feature = "output_events")]
    was_created_in_current_update_cycle: bool,
}

impl SvgElement {
    pub fn new(tag: &'static str, id: SvgElementId) -> Self {
        let id_attribute = SvgAttribute::Id { id };
        let inital_attributes: HashMap<&'static str, SvgAttribute> =
            HashMap::from([(id_attribute.key(), id_attribute)]);
        let inital_styles: HashMap<&'static str, SvgStyle> = HashMap::new();

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

    pub fn get_id(&self) -> SvgElementId {
        self.id
    }

    pub fn get_tag(&self) -> &'static str {
        &self.tag
    }

    // =========================================================================
    // Attributes
    // =========================================================================

    pub fn set_attribute(&mut self, attribute: SvgAttribute) {
        #[cfg(feature = "output_events")]
        self.register_change(SvgElementChange::AttributeUpdated(
            SvgAttributeUpdatedChange {
                key: attribute.key(),
                new_value: attribute.into_svg_string(),
            },
        ));

        self.attributes.insert(attribute.key(), attribute);
    }

    pub fn set_attributes(&mut self, attributes: Vec<SvgAttribute>) {
        for attribute in attributes {
            self.set_attribute(attribute);
        }
    }

    pub fn get_attribute(&self, key: &'static str) -> Option<&SvgAttribute> {
        self.attributes.get(key)
    }

    // =========================================================================
    // Styles
    // =========================================================================

    pub fn set_style(&mut self, style: SvgStyle) {
        #[cfg(feature = "output_events")]
        self.register_change(SvgElementChange::StyleUpdated(SvgStyleUpdatedChange {
            key: style.key(),
            new_value: style.into_svg_string(),
        }));

        self.styles.insert(style.key(), style);
    }

    pub fn set_styles(&mut self, styles: Vec<SvgStyle>) {
        for style in styles {
            self.set_style(style);
        }
    }

    pub fn get_style(&self, key: &'static str) -> Option<&SvgStyle> {
        self.styles.get(key)
    }

    // =========================================================================
    // Children
    // =========================================================================

    pub fn append_child_in_world_context(
        &mut self,
        entity: Entity,
        child_element: &mut SvgElement,
    ) {
        self.append_child_element(
            child_element,
            SvgElementChildIdentifier::InWorldContext(entity),
        )
    }

    pub fn append_child_in_node_context(&mut self, entity: Entity, child_element: &mut SvgElement) {
        self.append_child_element(
            child_element,
            SvgElementChildIdentifier::InSvgNodeContext(entity),
        );
    }

    fn append_child_element(
        &mut self,
        child_element: &mut SvgElement,
        identifier: SvgElementChildIdentifier,
    ) {
        self.children.push(SvgElementChild {
            id: child_element.get_id(),
            identifier,
        });
        #[cfg(feature = "output_events")]
        child_element.append_to_parent(self.id);
    }

    #[cfg(feature = "output_events")]
    pub fn append_to_parent(&mut self, parent_id: SvgElementId) {
        self.register_change(SvgElementChange::ElementAppended(
            self::element_changes::SvgElementAppendedChange { parent_id },
        ));
    }

    pub fn remove_child(&mut self, id: SvgElementId) {
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
        self.register_change(SvgElementChange::ElementCreated(SvgElementCreatedChange {
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
    fn register_change(&mut self, element_change: SvgElementChange) {
        if self.was_created_in_current_update_cycle {
            if let Some(update) = self.changes.first_mut() {
                match update {
                    SvgElementChange::ElementCreated(element_created_event) => match element_change
                    {
                        SvgElementChange::AttributeUpdated(event) => {
                            element_created_event
                                .attributes
                                .push((event.key, event.new_value));
                            return;
                        }
                        SvgElementChange::StyleUpdated(event) => {
                            element_created_event
                                .styles
                                .push((event.key, event.new_value));
                            return;
                        }
                        SvgElementChange::ElementAppended(event) => {
                            element_created_event.parent_id = Some(event.parent_id);
                            return;
                        }
                        SvgElementChange::ElementDeleted(_) => {
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

    /// Destroys this SvgElement.
    /// This method only handles the destruction of the element itself.
    /// It is the responsibility of the caller to ensure that any references to this element are properly managed.
    #[cfg(feature = "output_events")]
    pub fn destroy(&mut self) {
        self.register_change(SvgElementChange::ElementDeleted(SvgElementDeletedChange {}));
    }

    #[cfg(feature = "output_events")]
    pub fn drain_changes(&mut self) -> Vec<SvgElementChange> {
        self.was_created_in_current_update_cycle = false;
        self.changes.drain(..).collect()
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct SvgElementId(usize);

impl SvgElementId {
    pub const ZERO: SvgElementId = SvgElementId(0);

    pub fn next_id(&mut self) -> Self {
        let old = self.0;
        self.0 += 1;
        Self(old)
    }
}

impl Display for SvgElementId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SvgElementChild {
    pub id: SvgElementId,
    pub identifier: SvgElementChildIdentifier,
}

#[derive(Debug, Copy, Clone)]
pub enum SvgElementChildIdentifier {
    /// Child element is root element of SvgNode.
    InWorldContext(Entity),
    /// Child element is child element of SvgNode.
    InSvgNodeContext(Entity),
}
