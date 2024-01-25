use dyn_composition::core::utils::continuous_id::ContinuousId;

use crate::resources::changed_components::ChangedNode;

use super::svg_bundle::SVGBundle;
use std::fmt::Debug;

pub mod frame_svg_node;
pub mod shape_svg_node;

#[derive(Debug)]
pub struct ElementReference {
    // pub id: ContinuousId,
    pub index: usize,
}

pub trait SVGNode: SVGBundle + Sync + Send + Debug {
    fn apply_node_change(
        &mut self,
        changed_node: &ChangedNode,
        id_generator: &mut ContinuousId,
    ) -> ();
    fn get_node_append_id(&self) -> Option<&ElementReference>;
    fn get_paint_append_id(&self) -> Option<&ElementReference>;
}
