use bevy_transform::components::Transform;
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

    pub fn node_mixins_to_style(
        parent_layout_mixin: Option<&ParentLayoutMixin>,
        leaf_layout_mixin: Option<&LeafLayoutMixin>,
        transform: &Transform,
        size: &Size,
    ) -> Style {
        let parent_layout_style = parent_layout_mixin
            .map(|pl| pl.to_style())
            .unwrap_or(Style::default());
        let leaf_layout_style = leaf_layout_mixin
            .map(|ll| ll.to_style())
            .unwrap_or(Style::default());

        // Default margins are Auto
        let mut margin_left = LengthPercentageAuto::Auto;
        let mut margin_right = LengthPercentageAuto::Auto;
        let mut margin_top = LengthPercentageAuto::Auto;
        let mut margin_bottom = LengthPercentageAuto::Auto;

        // Adjust margins based on justify_self and align_self
        match leaf_layout_style.justify_self {
            Some(AlignItems::Start) | Some(AlignItems::FlexStart) => {
                margin_left = LengthPercentageAuto::Length(transform.translation.x);
            }
            Some(AlignItems::End) | Some(AlignItems::FlexEnd) => {
                margin_right = LengthPercentageAuto::Length(transform.translation.x);
            }
            Some(AlignItems::Center) => {
                margin_left = LengthPercentageAuto::Length(transform.translation.x / 2.0);
                margin_right = LengthPercentageAuto::Length(transform.translation.x / 2.0);
            }
            _ => {}
        }

        match leaf_layout_style.align_self {
            Some(AlignItems::Start) | Some(AlignItems::FlexStart) => {
                margin_top = LengthPercentageAuto::Length(transform.translation.y);
            }
            Some(AlignItems::End) | Some(AlignItems::FlexEnd) => {
                margin_bottom = LengthPercentageAuto::Length(transform.translation.y);
            }
            Some(AlignItems::Center) => {
                margin_top = LengthPercentageAuto::Length(transform.translation.y / 2.0);
                margin_bottom = LengthPercentageAuto::Length(transform.translation.y / 2.0);
            }
            _ => {}
        }

        // Constructing the final Style object
        Style {
            align_self: leaf_layout_style.align_self,
            justify_self: leaf_layout_style.justify_self,
            margin: Rect {
                left: margin_left,
                right: margin_right,
                top: margin_top,
                bottom: margin_bottom,
            },
            size: taffy::Size {
                width: Dimension::Length(size.width()),
                height: Dimension::Length(size.height()),
            },
            ..Default::default()
        }
    }
}

#[derive(Debug)]
pub enum LayoutError {
    InvalidHierarchy,
    TaffyError(TaffyError),
}
