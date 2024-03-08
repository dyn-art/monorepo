pub mod node;
pub mod style;

use std::{collections::BTreeMap, fmt::Debug};

#[cfg(feature = "output_svg_element_changes")]
use super::svg_element::element_changes::SvgElementChanges;
use super::svg_element::{SvgElement, SvgElementId};

pub trait SvgBundle: Debug {
    /// Retrieves SVG elements in a sorted order, starting from the top-level element and
    /// proceeding hierarchically to its children & siblings.
    ///
    /// Returns a `BTreeMap` mapping `ContinuousId` to references of `SvgElement`,
    /// ensuring the elements are sorted from the highest in the hierarchy to the lowest
    /// while allowing easy querying for single elements.
    fn get_elements(&self) -> BTreeMap<SvgElementId, &SvgElement>;

    /// Similar to `get_child_elements`, but returns mutable references to the SVG elements.
    fn get_elements_mut(&mut self) -> BTreeMap<SvgElementId, &mut SvgElement>;

    /// Returns a reference to the root `SvgElement`.
    fn get_root_element(&self) -> &SvgElement;

    /// Returns a mutable reference to the root `SvgElement`.
    fn get_root_element_mut(&mut self) -> &mut SvgElement;

    #[cfg(feature = "output_svg_element_changes")]
    fn drain_changes(&mut self) -> Vec<SvgElementChanges> {
        let mut drained_changes: Vec<SvgElementChanges> = Vec::new();

        for (_, element) in self.get_elements_mut() {
            let changes = element.drain_changes();
            if !changes.is_empty() {
                drained_changes.push(SvgElementChanges {
                    id: element.get_id(),
                    changes,
                });
            }
        }

        return drained_changes;
    }
}
