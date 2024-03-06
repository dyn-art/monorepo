pub mod frame;
pub mod shape;

use self::{frame::FrameNodeSvgBundle, shape::ShapeNodeSvgBundle};
use super::{fill::FillSvgBundle, stroke::StrokeSvgBundle, SvgBundle};
use crate::svg::svg_element::SvgElement;
use bevy_ecs::{component::Component, entity::Entity, query::Without, system::Query};
use dyn_comp_types::mixins::Root;
use smallvec::SmallVec;

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

    pub fn get_fill_wrapper_element_mut(&mut self) -> Option<&mut SvgElement> {
        match self {
            NodeSvgBundle::Frame(bundle) => Some(&mut bundle.fill_wrapper_g),
            NodeSvgBundle::Shape(bundle) => Some(&mut bundle.fill_wrapper_g),
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

    pub fn get_stroke_wrapper_element_mut(&mut self) -> Option<&mut SvgElement> {
        match self {
            NodeSvgBundle::Frame(bundle) => Some(&mut bundle.stroke_wrapper_g),
            NodeSvgBundle::Shape(bundle) => Some(&mut bundle.stroke_wrapper_g),
            _ => None,
        }
    }

    pub fn get_stroke_bundles_mut(&mut self) -> Option<&mut SmallVec<[StrokeSvgBundle; 2]>> {
        match self {
            NodeSvgBundle::Frame(bundle) => Some(&mut bundle.stroke_bundles),
            NodeSvgBundle::Shape(bundle) => Some(&mut bundle.stroke_bundles),
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
