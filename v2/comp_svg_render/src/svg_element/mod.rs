use std::collections::HashMap;

use crate::svg_element::continuous_id::ContinuousId;
use bevy_ecs::{component::Component, entity::Entity};

pub mod continuous_id;
pub mod element_change;

#[derive(Component, Debug, Clone)]
pub struct SVGElement {
    /// Unique identifier of the SVGElement
    id: SVGElementId,
    /// The type of SVG element (e.g., circle, rect).
    tag: &'static str,
    /// The attributes of the SVG element.
    attributes: HashMap<&'static str, String>,
    /// The style properties of the SVG element.
    styles: HashMap<&'static str, String>,
    z_index: ZIndex,
    /// Children of the SVG element in the SVG tree.
    children: Vec<SVGElementChildIdentifier>,
}

impl SVGElement {
    // TODO
}

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct SVGElementId(ContinuousId);

/// Represents the z-index of an SVG element within a flat ECS architecture.
///
/// The `ZIndex` struct is designed to facilitate the rendering order of SVG elements
/// without requiring a hierarchical tree structure. It combines an element's nesting level
/// (`indent_level`) with its relative position among siblings (`child_index`) to calculate
/// a composite z-index value. This approach allows for efficient rendering order determination
/// within an ECS, where elements are managed granularly and independently.
#[derive(Debug, Clone)]
pub struct ZIndex {
    /// Represents the depth of nesting of the SVG element. A higher value
    /// indicates a deeper level of nesting.
    pub indent_level: u16,
    /// The position of the element among its siblings within the same nesting level.
    /// The count starts from 0.
    pub child_index: u16,
}

impl ZIndex {
    /// Calculates the composite z-index for the SVG element.
    pub fn get_index(&self) -> u16 {
        self.indent_level + self.child_index
    }
}

#[derive(Debug, Clone)]
pub enum SVGElementChildIdentifier {
    /// Child element is root element of SVGNode.
    InWorldContext(Entity),
    /// Child element is child element of SVGNode.
    InSVGNodeContext(Entity, SVGElementId),
}
