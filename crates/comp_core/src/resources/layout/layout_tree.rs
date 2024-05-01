use bevy_ecs::entity::Entity;
use bevy_transform::components::Transform;
use dyn_comp_bundles::components::mixins::{LayoutElement, LayoutParent};
use dyn_utils::properties::size::Size;
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
        let node_id = self
            .taffy_tree
            .new_leaf(style.clone())
            .map_err(|e| LayoutError::TaffyError(e));

        log::info!("[new_leaf] {:?}", node_id); // TODO: REMOVE

        node_id
    }

    pub fn update_leaf(&mut self, node_id: NodeId, style: Style) -> bool {
        log::info!("[update_leaf] {:?}", node_id); // TODO: REMOVE
        self.taffy_tree.set_style(node_id, style).is_ok()
    }

    pub fn update_children(
        &mut self,
        parent_id: NodeId,
        child_ids: &Vec<NodeId>,
    ) -> Result<(), LayoutError> {
        log::info!("[update_children] {:?}: {:?}", parent_id, child_ids); // TODO: REMOVE
        self.taffy_tree
            .set_children(parent_id, child_ids)
            .map_err(|e| LayoutError::TaffyError(e))
    }

    pub fn try_remove_children(&mut self, node_id: NodeId) {
        self.taffy_tree.set_children(node_id, &[]).unwrap();
    }

    pub fn compute_layout(
        &mut self,
        node_id: NodeId,
        available_space: Size,
    ) -> Result<(), LayoutError> {
        self.taffy_tree
            .compute_layout(
                node_id,
                taffy::Size {
                    width: AvailableSpace::Definite(available_space.width()),
                    height: AvailableSpace::Definite(available_space.height()),
                },
            )
            .map_err(|e| LayoutError::TaffyError(e))
    }

    pub fn get_layout(&self, node_id: NodeId) -> Result<&Layout, LayoutError> {
        self.taffy_tree
            .layout(node_id)
            .map_err(|e| LayoutError::TaffyError(e))
    }

    pub fn merge_layout_parent_with_element(
        entity: Entity, // TODO: REMOVE
        maybe_layout_parent: Option<&LayoutParent>,
        maybe_layout_element: Option<&LayoutElement>,
        transform: &Transform,
        size: &Size,
        parent_size: Option<&Size>,
    ) -> Style {
        let mut style = Style::default();

        if let Some(layout_element) = maybe_layout_element {
            let layout_element_style =
                layout_element.to_style(entity, transform, size, parent_size);

            style.position = layout_element_style.position;
            style.inset = layout_element_style.inset;
            style.size = layout_element_style.size;
        }

        if let Some(layout_parent) = maybe_layout_parent {
            let layout_parent_style = layout_parent.to_style();

            // TODO:
        }

        return style;
    }
}

#[derive(Debug)]
pub enum LayoutError {
    InvalidHierarchy,
    TaffyError(TaffyError),
}
