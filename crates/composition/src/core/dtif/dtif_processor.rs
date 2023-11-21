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
    /// Maps DTIF entity to actual spawned entity in ECS world.
    eid_to_entity: HashMap<String, Entity>,
}

impl DTIFProcessor {
    pub fn new() -> Self {
        DTIFProcessor {
            eid_to_entity: HashMap::new(),
        }
    }

    // =============================================================================
    // Node
    // =============================================================================

    /// Processes a single DTIF node and its children.
    pub fn process_node(
        &mut self,
        node_eid: String,
        world: &mut World,
        dtif: &DTIFComposition,
    ) -> Option<Entity> {
        dtif.nodes.get(&node_eid).map(|dtif_node| {
            // Spawn node
            let node_entity = self.spawn_node(world, dtif_node);
            self.eid_to_entity.insert(node_eid, node_entity);

            self.process_fill(node_entity, world, dtif, dtif_node);
            self.process_children(node_entity, world, dtif, dtif_node);

            return node_entity;
        })
    }

    /// Processes the fill mixin of a DTIF node, if present.
    fn process_fill(
        &mut self,
        node_entity: Entity,
        world: &mut World,
        dtif: &DTIFComposition,
        dtif_node: &DTIFNode,
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
                    return self.process_paint(paint_eid, world, dtif);
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

    /// Processes the children of a DTIF node, if any.
    fn process_children(
        &mut self,
        node_entity: Entity,
        world: &mut World,
        dtif: &DTIFComposition,
        dtif_node: &DTIFNode,
    ) {
        if let DTIFNode::Frame(FrameNodeBundle { children_mixin, .. })
        | DTIFNode::Group(GroupNodeBundle { children_mixin, .. }) = dtif_node
        {
            // Process child nodes and collect their Bevy entity ids
            let new_children: Vec<Entity> = children_mixin
                .0
                .iter()
                .filter_map(|child_entity| {
                    let child_eid = DTIFProcessor::entity_to_eid(child_entity);
                    return self.process_node(child_eid, world, dtif);
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

    /// Spawns a DTIF node into the ECS world.
    fn spawn_node(&self, world: &mut World, node: &DTIFNode) -> Entity {
        match node {
            DTIFNode::Frame(bundle) => world.spawn(bundle.clone()).id(),
            DTIFNode::Rectangle(bundle) => world.spawn(bundle.clone()).id(),
            DTIFNode::Group(bundle) => world.spawn(bundle.clone()).id(),
        }
    }

    // =============================================================================
    // Paint
    // =============================================================================

    /// Processes a single DTIF paint.
    pub fn process_paint(
        &mut self,
        paint_eid: String,
        world: &mut World,
        dtif: &DTIFComposition,
    ) -> Option<Entity> {
        dtif.paints.get(&paint_eid).map(|paint| {
            // Spawn paint
            let paint_entity = world.spawn(paint.clone()).id();
            self.eid_to_entity.insert(paint_eid, paint_entity);

            return paint_entity;
        })
    }

    // =============================================================================
    // Event
    // =============================================================================

    /// Processes and sends the event into the ECS world.
    pub fn send_event_into_world(&self, event: CoreInputEvent, world: &mut World) {
        match event {
            CoreInputEvent::EntityMoved(mut event) => {
                if let Some(entity) = self.find_entity(&event.entity) {
                    event.entity = entity;
                    world.send_event(event);
                }
            }
            CoreInputEvent::EntitySetPosition(mut event) => {
                if let Some(entity) = self.find_entity(&event.entity) {
                    event.entity = entity;
                    world.send_event(event);
                }
            }
        }
    }

    // =============================================================================
    // Helper
    // =============================================================================

    /// Tries to find the actual spawned entity for a DTIF entity.
    fn find_entity(&self, dtif_entity: &Entity) -> Option<Entity> {
        let eid = DTIFProcessor::entity_to_eid(dtif_entity);
        self.eid_to_entity.get(&eid).cloned()
    }

    /// Converts an entity to a String we call DTIF entity (eid).
    ///
    /// Why?
    /// Due to an issue we have to work with a stringified enitity in the hashmap.
    /// https://github.com/serde-rs/serde/issues/1183
    #[inline]
    pub fn entity_to_eid(entity: &Entity) -> String {
        entity.to_bits().to_string()
    }
}
