pub mod frame;
pub mod shape;

use crate::svg::svg_node::SvgNode;

use self::{frame::FrameSvgNode, shape::ShapeSvgNode};
use bevy_ecs::component::Component;

/// Explicit SvgNode variants as Bevy Query doesn't support querying by trait yet
#[derive(Component, Debug, Clone)]
pub enum SvgNodeVariant {
    Frame(FrameSvgNode),
    Shape(ShapeSvgNode),
}

impl SvgNodeVariant {
    pub fn get_svg_node(&self) -> &dyn SvgNode {
        match self {
            SvgNodeVariant::Frame(node) => node,
            SvgNodeVariant::Shape(node) => node,
        }
    }

    pub fn get_svg_node_mut(&mut self) -> &mut dyn SvgNode {
        match self {
            SvgNodeVariant::Frame(node) => node,
            SvgNodeVariant::Shape(node) => node,
        }
    }
}
