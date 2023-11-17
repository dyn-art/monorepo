use bevy_ecs::entity::Entity;

use crate::core::modules::svg_render::mixin_change::MixinChange;

use self::base_svg_node::BaseSVGNode;

use super::SVGComposition;
use std::fmt::Debug;

pub mod base_svg_node;
pub mod frame_svg_node;
pub mod shape_svg_node;

#[derive(Debug)]
pub struct ElementReference {
    pub id: u32,
    pub index: usize,
}

pub trait SVGNode: Sync + Send + Debug {
    fn get_base(&self) -> &BaseSVGNode;
    fn get_base_mut(&mut self) -> &mut BaseSVGNode;
    fn to_string(&self, composition: &SVGComposition) -> String;
    fn apply_mixin_changes(&mut self, changes: &[MixinChange]) -> ();
    fn get_external_child_append_id(&self) -> Option<&ElementReference>;
}
