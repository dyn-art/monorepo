use std::collections::HashMap;

use bevy_ecs::prelude::*;

trait SVGNode: Component + Clone {}

#[derive(Component, Debug, Clone)]
pub struct SVGElement {
    // TODO: ContinuousId or based on intendLevel & childIndex?
    id: u32,
    tag: &'static str,
    attributes: HashMap<&'static str, String>,
    styles: HashMap<&'static str, String>,
    /// How deeply the SVGElement is nested.
    /// Acts like a stack index to determine which SVGElement should be rendered when.
    indent_level: u16,
    /// Position of the element in its parents child.
    /// Also used to determine which SVGElement should be rendered when.
    child_index: u16,

    children: Vec<SVGElementChild>,
}

#[derive(Debug, Clone)]
pub struct SVGElementChild {
    pub id: u32, // SVGElement Id
    pub identifier: SVGElementChildIdentifier,
}

#[derive(Debug, Clone)]
pub enum SVGElementChildIdentifier {
    /// Child element is root element of SVGNode.
    InWorldContext(Entity),
    /// Child element is child element of SVGNode.
    InSVGNodeContext(Entity),
}

#[derive(Component, Debug, Clone)]
pub struct FrameSVGNode {
    root: SVGElement,
    defs: SVGElement,

    // Content elements
    content_clip_path: SVGElement,
    content_clipped_rect: SVGElement,
    content_wrapper_g: SVGElement,

    // Children elements
    children_wrapper_g: SVGElement,

    // Fill elements
    fill_clip_path: SVGElement,
    fill_clipped_path: SVGElement,
    fill_wrapper_g: SVGElement,
    // Children
    // TODO: Handled by Bevy-Hirachy as the SVGNode will be part of the Entity it represents
    //  paint_children: Vec<Entity>,
    //  node_children: Vec<Entity>,
}
