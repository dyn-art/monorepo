use std::collections::BTreeMap;
use std::fmt::Debug;

use bevy_ecs::entity::Entity;
use dyn_composition::utils::continuous_id::ContinuousId;

#[cfg(feature = "output-event")]
use crate::events::output_event::ElementChangeEvent;
use crate::resources::changed_entities::{ChangedEntity, ChangedEntityType};

use super::{svg_context::SVGContext, svg_element::SVGElement};

/// Represents a collection of SVG elements representing a single `Entity`.
pub trait SVGBundle: Sync + Send + Debug {
    /// Returns a reference to the associated `Entity`.
    fn get_entity(&self) -> &Entity;

    /// Retrieves the type of the changed entity.
    fn get_type(&self) -> ChangedEntityType;

    /// Updates the SVG bundle elements based on specified changes in the given context.
    fn update(&mut self, changed_entity: ChangedEntity, cx: &mut SVGContext);

    /// Retrieves child SVG elements in a sorted order, starting from the top-level element and
    /// proceeding hierarchically to its children & siblings.
    ///
    /// Returns a `BTreeMap` mapping `ContinuousId` to references of `SVGElement`,
    /// ensuring the elements are sorted from the highest in the hierarchy to the lowest
    /// while allowing easy querying for single elements.
    fn get_child_elements(&self) -> BTreeMap<ContinuousId, &SVGElement>;

    /// Similar to `get_child_elements`, but returns mutable references to the SVG elements.
    fn get_child_elements_mut(&mut self) -> BTreeMap<ContinuousId, &mut SVGElement>;

    /// Returns a reference to the root `SVGElement`.
    fn get_root_element(&self) -> &SVGElement;

    /// Returns a mutable reference to the root `SVGElement`.
    fn get_root_element_mut(&mut self) -> &mut SVGElement;

    /// Returns a vector of child entities (`Entity`) in the bundle, representing nested bundles.
    fn get_child_entities(&self) -> Vec<Entity>;

    /// Destroys the specified SVG bundle and its elements.
    /// This method only handles the destruction of the bundle and its elements itself.
    /// It is the responsibility of the caller to ensure that any references to this bundle are properly managed.
    fn destroy(&mut self, cx: &mut SVGContext);

    /// Converts the SVG bundle into its SVG string representation.
    fn to_string(&self, cx: &SVGContext) -> String;

    /// Drains and returns all changes caused by the last update/s.
    #[cfg(feature = "output-event")]
    fn drain_changes(&mut self) -> Vec<ElementChangeEvent> {
        let mut drained_changes: Vec<ElementChangeEvent> = Vec::new();

        // Drain changes from root element
        let root = self.get_root_element_mut();
        let changes = root.drain_changes();
        if !changes.is_empty() {
            drained_changes.push(ElementChangeEvent {
                id: root.get_id(),
                changes,
            });
        }

        // Drain changes from children
        for (_, child_element) in self.get_child_elements_mut() {
            let changes = child_element.drain_changes();
            if !changes.is_empty() {
                drained_changes.push(ElementChangeEvent {
                    id: child_element.get_id(),
                    changes,
                });
            }
        }

        return drained_changes;
    }
}
