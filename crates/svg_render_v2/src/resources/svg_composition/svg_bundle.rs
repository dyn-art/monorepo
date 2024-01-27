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
    // fn append_child(&mut self, svg_bundle: &mut Box<dyn SVGBundle>) -> ();
    fn update(&mut self, changed_entity: ChangedEntity, cx: &mut SVGContext) -> ();
    fn get_child_elements(&self) -> BTreeMap<ContinuousId, &SVGElement>;
    fn get_child_elements_mut(&mut self) -> BTreeMap<ContinuousId, &mut SVGElement>;
    fn get_root_element(&self) -> &SVGElement;
    fn get_root_element_mut(&mut self) -> &mut SVGElement;
    fn to_string(&self, cx: &SVGContext) -> String;

    fn drain_changes(&mut self) -> Vec<ElementChangeEvent> {
        let mut drained_updates: Vec<ElementChangeEvent> = Vec::new();

        // Drain updates from root element
        let root = self.get_root_element_mut();
        let changes = root.drain_changes();
        if !changes.is_empty() {
            drained_updates.push(ElementChangeEvent {
                id: root.get_id(),
                changes,
            });
        }

        // Drain updates from children
        for (_, child_element) in self.get_child_elements_mut() {
            let changes = child_element.drain_changes();
            if !changes.is_empty() {
                drained_updates.push(ElementChangeEvent {
                    id: child_element.get_id(),
                    changes,
                });
            }
        }

        return drained_updates;
    }
}
