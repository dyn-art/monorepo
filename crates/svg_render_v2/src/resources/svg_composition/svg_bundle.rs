use std::collections::BTreeMap;
use std::fmt::Debug;

use bevy_ecs::entity::Entity;
use dyn_composition::core::utils::continuous_id::ContinuousId;

use crate::{
    events::output_event::ElementChangeEvent,
    resources::changed_entities::{ChangedEntity, ChangedEntityType},
};

use super::{svg_context::SVGContext, svg_element::SVGElement};

pub trait SVGBundle: Sync + Send + Debug {
    fn get_entity(&self) -> &Entity;
    fn get_type(&self) -> ChangedEntityType;
    fn append_child(&mut self, svg_bundle: &mut Box<dyn SVGBundle>) -> ();
    fn update(&mut self, changed_entity: ChangedEntity, cx: &mut SVGContext) -> ();
    fn get_child_elements(&self) -> BTreeMap<ContinuousId, &SVGElement>;
    fn get_child_elements_mut(&mut self) -> BTreeMap<ContinuousId, &mut SVGElement>;
    fn get_root_element(&self) -> &SVGElement;
    fn get_root_element_mut(&mut self) -> &mut SVGElement;
    fn drain_changes(&mut self) -> Vec<ElementChangeEvent>;
    fn to_string(&self, cx: &SVGContext) -> String;
}
