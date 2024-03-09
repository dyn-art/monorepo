pub mod frame;
pub mod shape;

use self::{frame::FrameNodeSvgBundle, shape::ShapeNodeSvgBundle};
use super::SvgBundle;
use crate::svg::svg_element::SvgElement;
use bevy_ecs::{component::Component, entity::Entity, query::Without, system::Query};
use dyn_comp_common::mixins::Root;
use smallvec::SmallVec;

#[derive(Debug, Clone)]
pub enum NodeSvgBundle {
    Frame(FrameNodeSvgBundle),
    Shape(ShapeNodeSvgBundle),
}

impl NodeSvgBundle {
    pub fn get_node_entity(&self) -> &Entity {
        match self {
            NodeSvgBundle::Frame(bundle) => &bundle.entity,
            NodeSvgBundle::Shape(bundle) => &bundle.entity,
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

    pub fn get_styles_wrapper_element_mut(&mut self) -> Option<&mut SvgElement> {
        match self {
            NodeSvgBundle::Frame(bundle) => Some(&mut bundle.styles_wrapper_g),
            NodeSvgBundle::Shape(bundle) => Some(&mut bundle.styles_wrapper_g),
            _ => None,
        }
    }

    pub fn get_styles_mut(&mut self) -> Option<&mut SmallVec<[Entity; 2]>> {
        match self {
            NodeSvgBundle::Frame(bundle) => Some(&mut bundle.styles),
            NodeSvgBundle::Shape(bundle) => Some(&mut bundle.styles),
            _ => None,
        }
    }

    pub fn get_styles(&self) -> Option<&SmallVec<[Entity; 2]>> {
        match self {
            NodeSvgBundle::Frame(bundle) => Some(&bundle.styles),
            NodeSvgBundle::Shape(bundle) => Some(&bundle.styles),
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
