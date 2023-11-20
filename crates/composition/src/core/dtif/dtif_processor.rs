use std::collections::HashMap;

use bevy_ecs::{entity::Entity, world::World};
use bevy_hierarchy::BuildWorldChildren;

use crate::core::modules::{
    composition::events::CoreInputEvent,
    node::components::{
        bundles::{FrameNodeBundle, GroupNodeBundle, RectangleNodeBundle},
        mixins::{ChildrenMixin, FillMixin},
    },
};

use super::{DTIFComposition, DTIFNode};

pub struct DTIFProcessor {
    eid_to_entity: HashMap<String, Entity>,
}

impl DTIFProcessor {
    pub fn new() -> Self {
        DTIFProcessor {
            eid_to_entity: HashMap::new(),
        }
    }

    /// Processes a single DTIF node and its children, updating internal mappings
    /// to track relationships between DTIF eids and ECS entity IDs
    pub fn process_node(
        &mut self,
        node_eid: &str,
        world: &mut World,
        dtif: &DTIFComposition,
    ) -> Option<Entity> {
        dtif.nodes.get(node_eid).map(|dtif_node| {
            let node_entity = self.spawn_node(world, dtif_node);
            self.eid_to_entity.insert(node_eid.to_string(), node_entity);

            self.process_fill(world, dtif, dtif_node, node_entity);
            self.process_children(world, dtif, dtif_node, node_entity);

            return node_entity;
        })
    }

    /// Processes the fill mixin of a DTIF node, if present
    fn process_fill(
        &self,
        world: &mut World,
        dtif: &DTIFComposition,
        dtif_node: &DTIFNode,
        node_entity: Entity,
    ) {
        if let DTIFNode::Frame(FrameNodeBundle { fill_mixin, .. })
        | DTIFNode::Rectangle(RectangleNodeBundle { fill_mixin, .. }) = dtif_node
        {
            // Process paints and collect their Bevy entity ids
            let new_paints: Vec<Entity> = fill_mixin
                .paints
                .iter()
                .filter_map(|paint_entity| {
                    let paint_eid = DTIFProcessor::entity_to_eid(paint_entity);
                    dtif.paints
                        .get(&paint_eid)
                        .map(|paint| world.spawn(paint.clone()).id())
                })
                .collect();

            // Establish Bevy parent-child relationships. Bevy's hierarchy system allows for
            // more optimized and feature-rich parent-child interactions within the ECS
            // https://bevy-cheatbook.github.io/fundamentals/hierarchy.html
            if !new_paints.is_empty() {
                world.entity_mut(node_entity).push_children(&new_paints);
            }

            // Now that Bevy's own parent-child relationship is established, we remove the
            // `FillMixin` as it was only a temporary measure to transition from the DTIF format
            world.entity_mut(node_entity).remove::<FillMixin>();
        }
    }

    /// Processes the children of a DTIF node, if any, by processing each child and establishing
    /// parent-child relationships in the ECS world
    fn process_children(
        &mut self,
        world: &mut World,
        dtif: &DTIFComposition,
        dtif_node: &DTIFNode,
        node_entity: Entity,
    ) {
        if let DTIFNode::Frame(FrameNodeBundle { children_mixin, .. })
        | DTIFNode::Group(GroupNodeBundle { children_mixin, .. }) = dtif_node
        {
            // Process child nodes and collect their Bevy entity ids
            let new_children: Vec<Entity> = children_mixin
                .0
                .iter()
                .filter_map(|child_entity| {
                    self.process_node(&DTIFProcessor::entity_to_eid(child_entity), world, dtif)
                })
                .collect();

            // Establish Bevy parent-child relationships. Bevy's hierarchy system allows for
            // more optimized and feature-rich parent-child interactions within the ECS
            // https://bevy-cheatbook.github.io/fundamentals/hierarchy.html
            if !new_children.is_empty() {
                world.entity_mut(node_entity).push_children(&new_children);
            }

            // Now that Bevy's own parent-child relationship is established, we remove the
            // `ChildrenMixin` as it was only a temporary measure to transition from the DTIF format
            world.entity_mut(node_entity).remove::<ChildrenMixin>();
        }
    }

    /// Spawns a DTIF node into the ECS world
    fn spawn_node(&self, world: &mut World, node: &DTIFNode) -> Entity {
        match node {
            DTIFNode::Frame(bundle) => world.spawn(bundle.clone()).id(),
            DTIFNode::Rectangle(bundle) => world.spawn(bundle.clone()).id(),
            DTIFNode::Group(bundle) => world.spawn(bundle.clone()).id(),
        }
    }

    /// Translate an entity id from the event to the actual entity
    pub fn translate_event_entity(&self, event_entity_id: &Entity) -> Option<Entity> {
        let eid = DTIFProcessor::entity_to_eid(event_entity_id);
        self.eid_to_entity.get(&eid).cloned()
    }

    /// Process and send the event to the ECS
    pub fn send_event_to_ecs(&self, world: &mut World, event: CoreInputEvent) {
        match event {
            CoreInputEvent::EntityMoved(mut event) => {
                if let Some(entity) = self.translate_event_entity(&event.entity) {
                    event.entity = entity;
                    world.send_event(event);
                }
            }
            CoreInputEvent::EntitySetPosition(mut event) => {
                if let Some(entity) = self.translate_event_entity(&event.entity) {
                    event.entity = entity;
                    world.send_event(event);
                }
            }
        }
    }

    /// Converts an Entity to a string we called "eid".
    ///
    /// Why?
    /// Due to an issue we have to work with a stringified Enitity in the Hashmap.
    /// https://github.com/serde-rs/serde/issues/1183
    #[inline]
    pub fn entity_to_eid(entity: &Entity) -> String {
        entity.to_bits().to_string()
    }
}
