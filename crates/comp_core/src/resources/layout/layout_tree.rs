use bevy_ecs::entity::{Entity, EntityHashMap};
use bevy_hierarchy::Children;
use dyn_utils::properties::size::Size;
use taffy::{prelude::*, TaffyError};

pub struct LayoutTree {
    pub entity_to_taffy: EntityHashMap<NodeId>,
    pub taffy_tree: TaffyTree,
}

impl Default for LayoutTree {
    fn default() -> Self {
        Self::new()
    }
}

impl LayoutTree {
    pub fn new() -> Self {
        Self {
            entity_to_taffy: EntityHashMap::default(),
            taffy_tree: TaffyTree::new(),
        }
    }

    /// Retrieves the Taffy node associated with the given node entity and updates its style.
    /// If no associated Taffy node exists a new Taffy node is inserted into the Taffy layout.
    pub fn upsert_node(&mut self, entity: Entity, style: Style) {
        match self.entity_to_taffy.entry(entity) {
            hashbrown::hash_map::Entry::Occupied(entry) => {
                self.taffy_tree.set_style(*entry.get(), style).unwrap();
            }
            hashbrown::hash_map::Entry::Vacant(entry) => {
                let node_id = self.taffy_tree.new_leaf(style).unwrap();
                entry.insert(node_id);
            }
        }
    }

    /// Update the children of the Taffy node corresponding to the given [`Entity`].
    pub fn update_children(
        &mut self,
        entity: Entity,
        children: &Children,
    ) -> Result<(), LayoutError> {
        let mut children_ids = Vec::with_capacity(children.len());
        for child in children {
            if let Some(taffy_node) = self.entity_to_taffy.get(child) {
                children_ids.push(*taffy_node);
            }
        }

        let node_id = self
            .entity_to_taffy
            .get(&entity)
            .ok_or(LayoutError::InvalidHierarchy)?;
        return self
            .taffy_tree
            .set_children(*node_id, &children_ids)
            .map_err(|e| LayoutError::TaffyError(e));
    }

    /// Removes children from the entity's Taffy node if it exists. Does nothing otherwise.
    pub fn try_remove_children(&mut self, entity: Entity) {
        if let Some(node_id) = self.entity_to_taffy.get(&entity) {
            self.taffy_tree.set_children(*node_id, &[]).unwrap();
        }
    }

    /// Compute the layout for each Taffy node corresponding to the root node [`Entity`] in the layout.
    pub fn compute_layouts(
        &mut self,
        entity: Entity,
        available_space: Size,
    ) -> Result<(), LayoutError> {
        if let Some(node_id) = self.entity_to_taffy.get(&entity) {
            self.taffy_tree
                .compute_layout(
                    *node_id,
                    taffy::Size {
                        width: AvailableSpace::Definite(available_space.width()),
                        height: AvailableSpace::Definite(available_space.height()),
                    },
                )
                .map_err(|e| LayoutError::TaffyError(e))
        } else {
            Err(LayoutError::InvalidHierarchy)
        }
    }

    /// Get the layout geometry for the Taffy node corresponding to the node [`Entity`].
    /// Does not compute the layout geometry, `compute_layouts` should be run before using this function.
    pub fn get_layout(&self, entity: Entity) -> Result<&Layout, LayoutError> {
        if let Some(node_id) = self.entity_to_taffy.get(&entity) {
            self.taffy_tree
                .layout(*node_id)
                .map_err(|e| LayoutError::TaffyError(e))
        } else {
            Err(LayoutError::InvalidHierarchy)
        }
    }
}

#[derive(Debug)]
pub enum LayoutError {
    InvalidHierarchy,
    TaffyError(TaffyError),
}
