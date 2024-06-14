use super::debug::print_branch;
use bevy_ecs::entity::Entity;
use dyn_arb_bundles::components::mixins::{
    LayoutElementSizingMode, LayoutParentSizingMode, StaticLayoutElement, StaticLayoutParent,
};
use dyn_utils::properties::size::Size;
use std::collections::HashMap;
use taffy::{prelude::*, TaffyError};

pub struct LayoutTree {
    taffy_tree: TaffyTree,
}

impl Default for LayoutTree {
    fn default() -> Self {
        Self::new()
    }
}

impl LayoutTree {
    pub fn new() -> Self {
        Self {
            taffy_tree: TaffyTree::new(),
        }
    }

    pub fn new_leaf(&mut self, style: Style) -> Result<NodeId, LayoutError> {
        // log::info!("[new_leaf] {:?}: {:#?}", node_id, style); // TODO: REMOVE

        return self
            .taffy_tree
            .new_leaf(style)
            .map_err(|e| LayoutError::TaffyError(e));
    }

    pub fn update_leaf(&mut self, node_id: NodeId, style: Style) -> bool {
        // log::info!("[update_leaf] {:?}: {:#?}", node_id, style); // TODO: REMOVE
        self.taffy_tree.set_style(node_id, style).is_ok()
    }

    pub fn update_children(
        &mut self,
        parent_id: NodeId,
        child_ids: &Vec<NodeId>,
    ) -> Result<(), LayoutError> {
        // log::info!("[update_children] {:?}: {:?}", parent_id, child_ids); // TODO: REMOVE
        self.taffy_tree
            .set_children(parent_id, child_ids)
            .map_err(|e| LayoutError::TaffyError(e))
    }

    pub fn try_remove_children(&mut self, node_id: NodeId) {
        self.taffy_tree.set_children(node_id, &[]).unwrap();
    }

    pub fn remove(&mut self, node_id: NodeId) -> Result<taffy::NodeId, LayoutError> {
        self.taffy_tree
            .remove(node_id)
            .map_err(|e| LayoutError::TaffyError(e))
    }

    pub fn compute_layout(
        &mut self,
        node_id: NodeId,
        available_space: taffy::Size<taffy::AvailableSpace>,
    ) -> Result<(), LayoutError> {
        self.taffy_tree
            .compute_layout(node_id, available_space)
            .map_err(|e| LayoutError::TaffyError(e))
    }

    pub fn get_layout(&self, node_id: NodeId) -> Result<&Layout, LayoutError> {
        self.taffy_tree
            .layout(node_id)
            .map_err(|e| LayoutError::TaffyError(e))
    }

    pub fn merge_layout_parent_with_element(
        maybe_layout_parent: Option<&StaticLayoutParent>,
        maybe_static_layout_element: Option<&StaticLayoutElement>,
        size: &Size,
    ) -> Style {
        let mut style = Style::default();

        let mut taffy_size = taffy::Size::<taffy::Dimension> {
            width: Dimension::Length(size.width()),
            height: Dimension::Length(size.height()),
        };

        if let Some(static_layout_element) = maybe_static_layout_element {
            let layout_element_style = static_layout_element.to_style();

            style.display = Display::Block;
            style.align_self = layout_element_style.align_self;
            style.justify_self = layout_element_style.justify_self;
            style.margin = layout_element_style.margin;

            taffy_size.width = match static_layout_element.horizontal_sizing_mode {
                LayoutElementSizingMode::Fixed => Dimension::Length(size.width()),
                LayoutElementSizingMode::Fill => Dimension::Percent(100.0),
            };
            taffy_size.height = match static_layout_element.vertical_sizing_mode {
                LayoutElementSizingMode::Fixed => Dimension::Length(size.height()),
                LayoutElementSizingMode::Fill => Dimension::Percent(100.0),
            };
        }

        if let Some(static_layout_parent) = maybe_layout_parent {
            let layout_parent_style = static_layout_parent.to_style();

            style.display = Display::Flex;
            style.align_items = layout_parent_style.align_items;
            style.justify_content = layout_parent_style.justify_content;
            style.gap = layout_parent_style.gap;
            style.padding = layout_parent_style.padding;
            style.flex_direction = layout_parent_style.flex_direction;

            // TODO: Does this conflict with the layout element?
            taffy_size.width = match static_layout_parent.horizontal_sizing_mode {
                LayoutParentSizingMode::Fixed => Dimension::Length(size.width()),
                LayoutParentSizingMode::Hug => Dimension::Auto,
            };
            taffy_size.height = match static_layout_parent.vertical_sizing_mode {
                LayoutParentSizingMode::Fixed => Dimension::Length(size.height()),
                LayoutParentSizingMode::Hug => Dimension::Auto,
            };
        }

        style.size = taffy_size;

        return style;
    }

    pub fn print_branch(&self, root_node_id: NodeId, taffy_to_entity: &HashMap<NodeId, Entity>) {
        print_branch(&self.taffy_tree, root_node_id, taffy_to_entity);
    }
}

#[derive(Debug)]
pub enum LayoutError {
    InvalidHierarchy,
    TaffyError(TaffyError),
}
