use bevy_ecs::prelude::*;
use svg_element::SVGElement;

pub mod svg_element;
pub mod svg_node;

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
