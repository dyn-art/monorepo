pub mod attributes;
pub mod conversions;
pub mod element_changes;
pub mod styles;

use self::{
    attributes::SvgAttribute, element_changes::SvgElementChildrenReorderedChange, styles::SvgStyle,
};
use super::svg_bundle::{SvgBundle, SvgBundleVariant};
use bevy_ecs::{component::Component, entity::Entity, query::Without, system::Query};
use dyn_comp_common::mixins::Root;
use smallvec::SmallVec;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

#[cfg(feature = "output_svg_element_changes")]
use self::element_changes::{
    SvgAttributeUpdatedChange, SvgElementChange, SvgElementCreatedChange, SvgElementDeletedChange,
    SvgStyleUpdatedChange,
};

#[derive(Component, Debug, Clone)]
pub struct SvgElement {
    /// Unique identifier of the SvgElement
    id: SvgElementId,
    /// The type of SvgElement (e.g., circle, rect).
    tag: SvgTag,
    /// The attributes of the SvgElement.
    attributes: HashMap<&'static str, SvgAttribute>,
    /// The style properties of the SvgElement.
    styles: HashMap<&'static str, SvgStyle>,
    /// Children of the SvgElement in the Svg tree.
    children: SmallVec<[SvgElementChild; 2]>,
    /// Applied changes after last drain.
    #[cfg(feature = "output_svg_element_changes")]
    changes: Vec<SvgElementChange>,
    /// Applied deffered changes after last drain.
    #[cfg(feature = "output_svg_element_changes")]
    deferred_changes: Vec<SvgElementChange>,
    /// Whether the element was created in the current update cycle (before first update drain).
    #[cfg(feature = "output_svg_element_changes")]
    was_created_in_current_update_cycle: bool,
}

impl SvgElement {
    pub fn new(tag: SvgTag, id: SvgElementId) -> Self {
        let id_attribute = SvgAttribute::Id { id };
        let inital_attributes: HashMap<&'static str, SvgAttribute> =
            HashMap::from([(id_attribute.key(), id_attribute)]);
        let inital_styles: HashMap<&'static str, SvgStyle> = HashMap::new();

        return Self {
            id,
            tag,
            attributes: inital_attributes,
            styles: inital_styles,
            children: SmallVec::new(),
            #[cfg(feature = "output_svg_element_changes")]
            changes: Vec::new(),
            #[cfg(feature = "output_svg_element_changes")]
            deferred_changes: Vec::new(),
            #[cfg(feature = "output_svg_element_changes")]
            was_created_in_current_update_cycle: true,
        };
    }

    pub fn get_id(&self) -> SvgElementId {
        self.id
    }

    // =========================================================================
    // Attributes
    // =========================================================================

    pub fn set_attribute(&mut self, attribute: SvgAttribute) {
        #[cfg(feature = "output_svg_element_changes")]
        {
            if let Some(current_attribute) = self.get_attribute(attribute.key()) {
                if *current_attribute == attribute {
                    return;
                }
            }

            self.register_change(
                SvgElementChange::AttributeUpdated(SvgAttributeUpdatedChange {
                    key: attribute.key(),
                    new_value: attribute.to_svg_string(),
                }),
                false,
            );
        }

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
        #[cfg(feature = "output_svg_element_changes")]
        {
            if let Some(current_style) = self.get_style(style.key()) {
                if *current_style == style {
                    return;
                }
            }

            self.register_change(
                SvgElementChange::StyleUpdated(SvgStyleUpdatedChange {
                    key: style.key(),
                    new_value: style.to_svg_string(),
                }),
                false,
            );
        }

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

    pub fn append_child_in_bundle_context(&mut self, child_element: &mut SvgElement) {
        self.append_child_element(child_element, SvgElementChildIdentifier::InSvgBundleContext);
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
        #[cfg(feature = "output_svg_element_changes")]
        child_element.append_to_parent(self.id);
    }

    #[cfg(feature = "output_svg_element_changes")]
    pub fn append_to_parent(&mut self, parent_id: SvgElementId) {
        self.register_change(
            SvgElementChange::ElementAppended(self::element_changes::SvgElementAppendedChange {
                parent_id,
            }),
            false,
        );
    }

    pub fn remove_child(&mut self, id: SvgElementId) {
        self.children.retain(|child| child.id != id);
    }

    pub fn remove_children(&mut self, ids: &[SvgElementId]) {
        let ids_set = ids.iter().collect::<HashSet<_>>();
        self.children.retain(|child| !ids_set.contains(&child.id));
    }

    pub fn clear_children(&mut self) {
        self.children.clear()
    }

    pub fn reorder_children_mut<F>(&mut self, reorder_operation: F)
    where
        F: FnOnce(&mut SmallVec<[SvgElementChild; 2]>),
    {
        let original_order: SmallVec<[SvgElementId; 2]> =
            self.children.iter().map(|child| child.id).collect();

        // Apply the reorder operation provided by the caller
        reorder_operation(&mut self.children);

        // Check if the order has changed and emit reorder event if so
        let new_order: SmallVec<[SvgElementId; 2]> =
            self.children.iter().map(|child| child.id).collect();
        if new_order != original_order {
            self.register_change(
                SvgElementChange::ElementChildrenReordered(SvgElementChildrenReorderedChange {
                    new_order: new_order.into_vec(),
                }),
                true,
            );
        }
    }

    // =========================================================================
    // Changes
    // =========================================================================

    #[cfg(feature = "output_svg_element_changes")]
    pub fn init_element_created(&mut self, entity: Option<Entity>) {
        self.register_change(
            SvgElementChange::ElementCreated(SvgElementCreatedChange {
                parent_id: None,
                tag_name: self.tag.as_str(),
                attributes: self
                    .attributes
                    .values()
                    .map(|value| value.to_tuple())
                    .collect(),
                styles: self.styles.values().map(|value| value.to_tuple()).collect(),
                entity,
            }),
            false,
        );
    }

    #[cfg(feature = "output_svg_element_changes")]
    fn register_change(&mut self, element_change: SvgElementChange, deferred: bool) {
        // Try to minimize events if element was created by applying changes to the ElementCreated event
        if self.was_created_in_current_update_cycle && !deferred {
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

        if deferred {
            self.deferred_changes.push(element_change);
        } else {
            self.changes.push(element_change);
        }
    }

    // =========================================================================
    // Other
    // =========================================================================

    #[cfg(feature = "output_svg_element_changes")]
    pub fn drain_changes(&mut self) -> (Vec<SvgElementChange>, Vec<SvgElementChange>) {
        self.was_created_in_current_update_cycle = false;
        return (
            self.changes.drain(..).collect(),
            self.deferred_changes.drain(..).collect(),
        );
    }

    /// Destroys this SvgElement.
    /// This method only handles the destruction of the element itself.
    /// It is the responsibility of the caller to ensure that any references to this element are properly managed.
    #[cfg(feature = "output_svg_element_changes")]
    pub fn destroy(&mut self) {
        self.register_change(
            SvgElementChange::ElementDeleted(SvgElementDeletedChange {}),
            true,
        );
    }

    pub fn to_string(
        &self,
        bundle: &dyn SvgBundle,
        maybe_bundle_variant_query: Option<&Query<&SvgBundleVariant, Without<Root>>>,
    ) -> String {
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
        let bundle_elements = bundle.get_elements_map();
        for child in &self.children {
            match child.identifier {
                SvgElementChildIdentifier::InSvgBundleContext => {
                    if let Some(child_element) = bundle_elements.get(&child.id) {
                        result
                            .push_str(&child_element.to_string(bundle, maybe_bundle_variant_query));
                    }
                }
                SvgElementChildIdentifier::InWorldContext(entity) => {
                    if let Some(bundle_variant_query) = maybe_bundle_variant_query {
                        if let Ok(bundle_variant) = bundle_variant_query.get(entity) {
                            result.push_str(&bundle_variant.to_string(bundle_variant_query));
                        }
                    }
                }
            }
        }

        // Close SVG tag
        result.push_str(&format!("</{}>", self.tag.as_str()));

        return result;
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
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
pub enum SvgTag {
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

impl SvgTag {
    pub fn as_str(&self) -> &'static str {
        match self {
            SvgTag::Circle => "circle",
            SvgTag::Rect => "rect",
            SvgTag::Path => "path",
            SvgTag::Line => "line",
            SvgTag::Ellipse => "ellipse",
            SvgTag::Polygon => "polygon",
            SvgTag::Polyline => "polyline",
            SvgTag::Text => "text",
            SvgTag::Group => "g",
            SvgTag::Defs => "defs",
            SvgTag::ClipPath => "clipPath",
            SvgTag::Pattern => "pattern",
            SvgTag::Image => "image",
            SvgTag::LinearGradient => "linearGradient",
            SvgTag::RadialGradient => "radialGradient",
            SvgTag::Stop => "stop",
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SvgElementChild {
    pub id: SvgElementId,
    pub identifier: SvgElementChildIdentifier,
}

#[derive(Debug, Copy, Clone)]
pub enum SvgElementChildIdentifier {
    /// Child element is root element of SvgBundle.
    InWorldContext(Entity),
    /// Child element is child element of SvgBundle.
    InSvgBundleContext,
}
