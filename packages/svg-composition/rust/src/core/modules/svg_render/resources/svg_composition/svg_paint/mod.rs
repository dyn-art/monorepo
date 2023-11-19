use crate::core::modules::svg_render::resources::svg_composition::{
    svg_bundle::SVGBundle, svg_node::SVGNode, SVGComposition,
};

use std::fmt::Debug;

pub mod solid_svg_paint;

pub trait SVGPaint: SVGBundle + Sync + Send + Debug {
    fn to_string(&self, node: &dyn SVGNode, composition: &SVGComposition) -> String;
}
