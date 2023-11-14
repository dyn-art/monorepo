use bevy_ecs::entity::Entity;

use crate::core::modules::svg_render::mixin_change::MixinChange;

use self::base_svg_node::BaseSVGNode;

use super::svg_composition::SVGComposition;
use std::fmt::Debug;

pub mod base_svg_node;
pub mod frame_svg_node;
pub mod shape_svg_node;

#[derive(Debug)]
pub struct ElementReference {
    id: u32,
    index: usize,
}

pub trait SVGNode: Sync + Send + Debug {
    fn get_base(&self) -> &BaseSVGNode;
    fn get_base_mut(&mut self) -> &mut BaseSVGNode;
    fn to_string(&self, composition: &SVGComposition) -> String;
    fn apply_mixin_changes(&mut self, changes: &[MixinChange]) -> ();
    fn append_external_child(&mut self, entity: Entity) -> ();
}
