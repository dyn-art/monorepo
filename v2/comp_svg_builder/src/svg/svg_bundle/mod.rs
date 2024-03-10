pub mod node;
pub mod style;

use self::{
    node::{frame::FrameNodeSvgBundle, shape::ShapeNodeSvgBundle},
    style::solid::SolidStyleSvgBundle,
};
use bevy_ecs::{component::Component, entity::Entity, query::Without, system::Query};
use dyn_comp_common::mixins::Root;
use smallvec::SmallVec;
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
    fn drain_changes(&mut self) -> (Vec<SvgElementChanges>, Vec<SvgElementChanges>) {
        let mut elements_changes: Vec<SvgElementChanges> = Vec::new();
        let mut deferred_elements_changes: Vec<SvgElementChanges> = Vec::new();

        for element in self.elements_iter_mut() {
            let (element_changes, deffered_element_changes) = element.drain_changes();
            if !element_changes.is_empty() {
                elements_changes.push(SvgElementChanges {
                    id: element.get_id(),
                    changes: element_changes,
                });
            }
            if !deffered_element_changes.is_empty() {
                deferred_elements_changes.push(SvgElementChanges {
                    id: element.get_id(),
                    changes: deffered_element_changes,
                });
            }
        }

        return (elements_changes, deferred_elements_changes);
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
    pub fn get_svg_bundle(&self) -> &dyn SvgBundle {
        match self {
            SvgBundleVariant::Frame(bundle) => bundle,
            SvgBundleVariant::Shape(bundle) => bundle,
            SvgBundleVariant::Solid(bundle) => bundle,
        }
    }

    pub fn get_svg_bundle_mut(&mut self) -> &mut dyn SvgBundle {
        match self {
            SvgBundleVariant::Frame(bundle) => bundle,
            SvgBundleVariant::Shape(bundle) => bundle,
            SvgBundleVariant::Solid(bundle) => bundle,
        }
    }

    pub fn get_style_entities(&self) -> Option<&SmallVec<[Entity; 2]>> {
        match self {
            SvgBundleVariant::Frame(bundle) => Some(&bundle.style_entities),
            SvgBundleVariant::Shape(bundle) => Some(&bundle.style_entities),
            _ => None,
        }
    }

    pub fn get_style_entities_mut(&mut self) -> Option<&mut SmallVec<[Entity; 2]>> {
        match self {
            SvgBundleVariant::Frame(bundle) => Some(&mut bundle.style_entities),
            SvgBundleVariant::Shape(bundle) => Some(&mut bundle.style_entities),
            _ => None,
        }
    }

    pub fn get_child_node_entities(&self) -> Option<&SmallVec<[Entity; 2]>> {
        match self {
            SvgBundleVariant::Frame(bundle) => Some(&bundle.child_node_entities),
            _ => None,
        }
    }

    pub fn get_child_node_entities_mut(&mut self) -> Option<&mut SmallVec<[Entity; 2]>> {
        match self {
            SvgBundleVariant::Frame(bundle) => Some(&mut bundle.child_node_entities),
            _ => None,
        }
    }

    pub fn get_root_element(&self) -> &SvgElement {
        match self {
            SvgBundleVariant::Frame(bundle) => &bundle.root_g,
            SvgBundleVariant::Shape(bundle) => &bundle.root_g,
            SvgBundleVariant::Solid(bundle) => &bundle.root_g,
        }
    }

    pub fn get_root_element_mut(&mut self) -> &mut SvgElement {
        match self {
            SvgBundleVariant::Frame(bundle) => &mut bundle.root_g,
            SvgBundleVariant::Shape(bundle) => &mut bundle.root_g,
            SvgBundleVariant::Solid(bundle) => &mut bundle.root_g,
        }
    }

    pub fn get_click_area_element(&self) -> Option<&SvgElement> {
        match self {
            SvgBundleVariant::Frame(bundle) => Some(&bundle.click_area_rect),
            SvgBundleVariant::Shape(bundle) => Some(&bundle.click_area_rect),
            _ => None,
        }
    }

    pub fn get_click_area_element_mut(&mut self) -> Option<&mut SvgElement> {
        match self {
            SvgBundleVariant::Frame(bundle) => Some(&mut bundle.click_area_rect),
            SvgBundleVariant::Shape(bundle) => Some(&mut bundle.click_area_rect),
            _ => None,
        }
    }

    pub fn get_children_wrapper_element(&self) -> Option<&SvgElement> {
        match self {
            SvgBundleVariant::Frame(bundle) => Some(&bundle.children_wrapper_g),
            _ => None,
        }
    }

    pub fn get_children_wrapper_element_mut(&mut self) -> Option<&mut SvgElement> {
        match self {
            SvgBundleVariant::Frame(bundle) => Some(&mut bundle.children_wrapper_g),
            _ => None,
        }
    }

    pub fn get_styles_wrapper_element(&self) -> Option<&SvgElement> {
        match self {
            SvgBundleVariant::Frame(bundle) => Some(&bundle.styles_wrapper_g),
            SvgBundleVariant::Shape(bundle) => Some(&bundle.styles_wrapper_g),
            _ => None,
        }
    }

    pub fn get_styles_wrapper_element_mut(&mut self) -> Option<&mut SvgElement> {
        match self {
            SvgBundleVariant::Frame(bundle) => Some(&mut bundle.styles_wrapper_g),
            SvgBundleVariant::Shape(bundle) => Some(&mut bundle.styles_wrapper_g),
            _ => None,
        }
    }

    pub fn to_string(
        &self,
        bundle_variant_query: &Query<&SvgBundleVariant, Without<Root>>,
    ) -> String {
        match self {
            SvgBundleVariant::Frame(bundle) => bundle
                .get_root_element()
                .to_string(bundle, Some(bundle_variant_query)),
            SvgBundleVariant::Shape(bundle) => bundle
                .get_root_element()
                .to_string(bundle, Some(bundle_variant_query)),
            SvgBundleVariant::Solid(bundle) => bundle.get_root_element().to_string(bundle, None),
        }
    }
}
