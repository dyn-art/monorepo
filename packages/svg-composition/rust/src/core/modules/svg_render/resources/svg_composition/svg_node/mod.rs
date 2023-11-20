use crate::core::{
    events::output_event::RenderUpdateEvent,
    modules::svg_render::resources::changed_components::ChangedNode,
};

use super::{svg_bundle::SVGBundle, SVGComposition};
use std::fmt::Debug;

pub mod frame_svg_node;
pub mod shape_svg_node;

#[derive(Debug)]
pub struct ElementReference {
    pub id: u32,
    pub index: usize,
}

pub trait SVGNode: SVGBundle + Sync + Send + Debug {
    fn apply_node_change(&mut self, changed_node: &ChangedNode) -> ();
    fn get_external_child_append_id(&self) -> Option<&ElementReference>;
    fn get_paint_append_id(&self) -> Option<&ElementReference>;
    fn drain_updates(&mut self) -> Vec<RenderUpdateEvent>;
    fn to_string(&self, composition: &SVGComposition) -> String;
}
