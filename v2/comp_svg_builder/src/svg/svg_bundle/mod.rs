pub mod frame_node;
pub mod shape_node;
pub mod solid_paint;

use self::{
    frame_node::FrameNodeSvgBundle, shape_node::ShapeNodeSvgBundle,
    solid_paint::SolidPaintSvgBundle,
};
use super::svg_element::{SvgElement, SvgElementId};
use bevy_ecs::{component::Component, query::Without, system::Query};
use dyn_comp_types::mixins::Root;
use std::{collections::BTreeMap, fmt::Debug};

#[cfg(feature = "output_svg_element_changes")]
use super::svg_element::element_changes::SvgElementChanges;

pub trait SvgBundle: Debug {
    /// Retrieves child SVG elements in a sorted order, starting from the top-level element and
    /// proceeding hierarchically to its children & siblings.
    ///
    /// Returns a `BTreeMap` mapping `ContinuousId` to references of `SvgElement`,
    /// ensuring the elements are sorted from the highest in the hierarchy to the lowest
    /// while allowing easy querying for single elements.
    fn get_child_elements(&self) -> BTreeMap<SvgElementId, &SvgElement>;

    /// Similar to `get_child_elements`, but returns mutable references to the Svg elements.
    fn get_child_elements_mut(&mut self) -> BTreeMap<SvgElementId, &mut SvgElement>;

    /// Returns a reference to the root `SvgElement`.
    fn get_root_element(&self) -> &SvgElement;

    /// Returns a mutable reference to the root `SvgElement`.
    fn get_root_element_mut(&mut self) -> &mut SvgElement;

    #[cfg(feature = "output_svg_element_changes")]
    fn drain_changes(&mut self) -> Vec<SvgElementChanges> {
        let mut drained_changes: Vec<SvgElementChanges> = Vec::new();

        // Drain changes from root element
        let root = self.get_root_element_mut();
        let changes = root.drain_changes();
        if !changes.is_empty() {
            drained_changes.push(SvgElementChanges {
                id: root.get_id(),
                changes,
            });
        }

        // Drain changes from children
        for (_, child_element) in self.get_child_elements_mut() {
            let changes = child_element.drain_changes();
            if !changes.is_empty() {
                drained_changes.push(SvgElementChanges {
                    id: child_element.get_id(),
                    changes,
                });
            }
        }

        return drained_changes;
    }
}

#[derive(Component, Debug, Clone)]
pub enum NodeSvgBundleVariant {
    Frame(FrameNodeSvgBundle),
    Shape(ShapeNodeSvgBundle),
}

impl NodeSvgBundleVariant {
    pub fn get_svg_bundle(&self) -> &dyn SvgBundle {
        match self {
            NodeSvgBundleVariant::Frame(bundle) => bundle,
            NodeSvgBundleVariant::Shape(bundle) => bundle,
        }
    }

    pub fn get_svg_bundle_mut(&mut self) -> &mut dyn SvgBundle {
        match self {
            NodeSvgBundleVariant::Frame(bundle) => bundle,
            NodeSvgBundleVariant::Shape(bundle) => bundle,
        }
    }

    pub fn to_string(&self, bundle_query: &Query<&NodeSvgBundleVariant, Without<Root>>) -> String {
        match self {
            NodeSvgBundleVariant::Frame(bundle) => bundle
                .get_root_element()
                .to_string(bundle, Some(bundle_query)),
            NodeSvgBundleVariant::Shape(bundle) => {
                bundle.get_root_element().to_string(bundle, None)
            }
        }
    }
}

#[derive(Component, Debug, Clone)]
pub enum PaintSvgBundleVariant {
    Solid(SolidPaintSvgBundle),
}

impl PaintSvgBundleVariant {
    pub fn get_svg_bundle(&self) -> &dyn SvgBundle {
        match self {
            PaintSvgBundleVariant::Solid(bundle) => bundle,
        }
    }

    pub fn get_svg_bundle_mut(&mut self) -> &mut dyn SvgBundle {
        match self {
            PaintSvgBundleVariant::Solid(bundle) => bundle,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            PaintSvgBundleVariant::Solid(bundle) => {
                bundle.get_root_element().to_string(bundle, None)
            }
        }
    }
}