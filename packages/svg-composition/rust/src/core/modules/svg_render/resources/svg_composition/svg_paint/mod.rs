use dyn_composition::core::modules::node::components::mixins::Paint;

use crate::core::{
    events::output_event::RenderUpdateEvent,
    modules::svg_render::resources::svg_composition::{
        svg_bundle::SVGBundle, svg_node::SVGNode, SVGComposition,
    },
};

use std::fmt::Debug;

pub mod solid_svg_paint;

pub trait SVGPaint: SVGBundle + Sync + Send + Debug {
    fn apply_paint_change(&mut self, paint: &Paint) -> ();
    fn drain_updates(&mut self) -> Vec<RenderUpdateEvent>;
    fn to_string(&self, node: &dyn SVGNode, composition: &SVGComposition) -> String;
}