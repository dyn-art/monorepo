use dyn_comp_bundles::components::mixins::{LeafLayoutMixin, ParentLayoutMixin, ToTaffyStyle};
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
        self.taffy_tree
            .new_leaf(style)
            .map_err(|e| LayoutError::TaffyError(e))
    }

    pub fn update_leaf(&mut self, node_id: NodeId, style: Style) -> bool {
        self.taffy_tree.set_style(node_id, style).is_ok()
    }

    pub fn update_children(
        &mut self,
        parent_id: NodeId,
        child_ids: &Vec<NodeId>,
    ) -> Result<(), LayoutError> {
        self.taffy_tree
            .set_children(parent_id, child_ids)
            .map_err(|e| LayoutError::TaffyError(e))
    }

    pub fn try_remove_children(&mut self, node_id: NodeId) {
        self.taffy_tree.set_children(node_id, &[]).unwrap();
    }

    pub fn compute_layouts(
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

    pub fn layout_mixins_to_style(
        parent_layout_mixin: Option<&ParentLayoutMixin>,
        leaf_layout_mixin: Option<&LeafLayoutMixin>,
    ) -> Style {
        let parent_layout_style = parent_layout_mixin
            .map(|pl| pl.to_style())
            .unwrap_or(Style::default());
        let leaf_layout_style = leaf_layout_mixin
            .map(|ll| ll.to_style())
            .unwrap_or(Style::default());

        return Style {
            align_self: parent_layout_style.align_self,
            justify_self: parent_layout_style.justify_self,
            ..Default::default()
        };
    }
}

#[derive(Debug)]
pub enum LayoutError {
    InvalidHierarchy,
    TaffyError(TaffyError),
}
