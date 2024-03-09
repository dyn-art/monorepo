pub mod node;
pub mod style;

use self::{
    node::{frame::FrameNodeSvgBundle, shape::ShapeNodeSvgBundle},
    style::solid::SolidStyleSvgBundle,
};
use bevy_ecs::{component::Component, entity::Entity, query::Without, system::Query};
use dyn_comp_common::mixins::Root;
use std::{collections::HashMap, fmt::Debug};

#[cfg(feature = "output_svg_element_changes")]
use super::svg_element::element_changes::SvgElementChanges;
use super::svg_element::{SvgElement, SvgElementId};

pub trait SvgBundle: Debug {
    fn elements_iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a SvgElement> + 'a>;

    fn elements_iter_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &'a mut SvgElement> + 'a>;

    fn get_elements_map(&self) -> HashMap<SvgElementId, &SvgElement> {
        self.elements_iter()
            .map(|element| (element.get_id(), element))
            .collect()
    }

    fn get_elements_map_mut(&mut self) -> HashMap<SvgElementId, &mut SvgElement> {
        self.elements_iter_mut()
            .map(|element| (element.get_id(), element))
            .collect()
    }

    /// Returns a reference to the root `SvgElement`.
    fn get_root_element(&self) -> &SvgElement;

    /// Returns a mutable reference to the root `SvgElement`.
    fn get_root_element_mut(&mut self) -> &mut SvgElement;

    fn get_entity(&self) -> &Entity;

    #[cfg(feature = "output_svg_element_changes")]
    fn drain_changes(&mut self) -> Vec<SvgElementChanges> {
        let mut drained_changes: Vec<SvgElementChanges> = Vec::new();

        for element in self.elements_iter_mut() {
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

// Variant enum of SvgBundle variants because Bevy doesn't allow to query for traits like SvgBundle
#[derive(Component, Debug, Clone)]
pub enum SvgBundleVariant {
    // Nodes
    Frame(FrameNodeSvgBundle),
    Shape(ShapeNodeSvgBundle),
    // Styles
    Solid(SolidStyleSvgBundle),
    // Gradient
    // Image
    // Drop Shadow
}

impl SvgBundleVariant {
    // TODO

    pub fn to_string(&self, bundle_query: &Query<&SvgBundleVariant, Without<Root>>) -> String {
        // match self {
        //     SvgBundleVariant::Frame(bundle) => bundle
        //         .get_root_element()
        //         .to_string(bundle, Some(bundle_query)),
        //     SvgBundleVariant::Shape(bundle) => bundle
        //         .get_root_element()
        //         .to_string(bundle, Some(bundle_query)),
        //     SvgBundleVariant::Solid(bundle) => bundle.get_root_element().to_string(bundle, None),
        // }
        String::from("hello")
    }
}
