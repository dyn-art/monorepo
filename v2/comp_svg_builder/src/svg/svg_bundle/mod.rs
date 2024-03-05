pub mod frame_node;
pub mod shape_node;
pub mod solid_fill;

use self::{
    frame_node::FrameNodeSvgBundle, shape_node::ShapeNodeSvgBundle, solid_fill::SolidFillSvgBundle,
};
use super::svg_element::{SvgElement, SvgElementId};
use bevy_ecs::{component::Component, entity::Entity, query::Without, system::Query};
use dyn_comp_types::mixins::Root;
use smallvec::SmallVec;
use std::{collections::BTreeMap, fmt::Debug};

#[cfg(feature = "output_svg_element_changes")]
use super::svg_element::element_changes::SvgElementChanges;

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

#[derive(Debug, Clone)]
pub enum NodeSvgBundle {
    Frame(FrameNodeSvgBundle),
    Shape(ShapeNodeSvgBundle),
}

impl NodeSvgBundle {
    pub fn get_node_entity(&self) -> &Entity {
        match self {
            NodeSvgBundle::Frame(bundle) => &bundle.node_entity,
            NodeSvgBundle::Shape(bundle) => &bundle.node_entity,
        }
    }

    pub fn get_svg_bundle(&self) -> &dyn SvgBundle {
        match self {
            NodeSvgBundle::Frame(bundle) => bundle,
            NodeSvgBundle::Shape(bundle) => bundle,
        }
    }

    pub fn get_svg_bundle_mut(&mut self) -> &mut dyn SvgBundle {
        match self {
            NodeSvgBundle::Frame(bundle) => bundle,
            NodeSvgBundle::Shape(bundle) => bundle,
        }
    }

    pub fn get_fills_wrapper_element_mut(&mut self) -> Option<&mut SvgElement> {
        match self {
            NodeSvgBundle::Frame(bundle) => Some(&mut bundle.fills_wrapper_g),
            NodeSvgBundle::Shape(bundle) => Some(&mut bundle.fills_wrapper_g),
            _ => None,
        }
    }

    pub fn get_fill_bundles_mut(&mut self) -> Option<&mut SmallVec<[FillSvgBundle; 2]>> {
        match self {
            NodeSvgBundle::Frame(bundle) => Some(&mut bundle.fill_bundles),
            NodeSvgBundle::Shape(bundle) => Some(&mut bundle.fill_bundles),
            _ => None,
        }
    }

    pub fn get_child_nodes_mut(&mut self) -> Option<&mut SmallVec<[Entity; 2]>> {
        match self {
            NodeSvgBundle::Frame(bundle) => Some(&mut bundle.child_nodes),
            _ => None,
        }
    }

    pub fn get_child_nodes(&self) -> Option<&SmallVec<[Entity; 2]>> {
        match self {
            NodeSvgBundle::Frame(bundle) => Some(&bundle.child_nodes),
            _ => None,
        }
    }

    pub fn get_children_wrapper_element_mut(&mut self) -> Option<&mut SvgElement> {
        match self {
            NodeSvgBundle::Frame(bundle) => Some(&mut bundle.children_wrapper_g),
            _ => None,
        }
    }

    pub fn to_string(&self, bundle_query: &Query<&NodeSvgBundleMixin, Without<Root>>) -> String {
        match self {
            NodeSvgBundle::Frame(bundle) => bundle
                .get_root_element()
                .to_string(bundle, Some(bundle_query)),
            NodeSvgBundle::Shape(bundle) => bundle.get_root_element().to_string(bundle, None),
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct NodeSvgBundleMixin(pub NodeSvgBundle);

#[derive(Debug, Clone)]
pub enum FillSvgBundle {
    Solid(SolidFillSvgBundle),
}

impl FillSvgBundle {
    pub fn get_paint_entity(&self) -> &Entity {
        match self {
            FillSvgBundle::Solid(bundle) => &bundle.paint_entity,
        }
    }

    pub fn get_svg_bundle(&self) -> &dyn SvgBundle {
        match self {
            FillSvgBundle::Solid(bundle) => bundle,
        }
    }

    pub fn get_svg_bundle_mut(&mut self) -> &mut dyn SvgBundle {
        match self {
            FillSvgBundle::Solid(bundle) => bundle,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            FillSvgBundle::Solid(bundle) => bundle.get_root_element().to_string(bundle, None),
        }
    }
}
