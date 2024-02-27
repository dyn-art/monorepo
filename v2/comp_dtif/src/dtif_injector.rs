use crate::{
    events::DtifInputEvent,
    nodes::{FrameNode, GroupNode, Node},
    paints::Paint,
    CompDtif, ToEcsBundleImpl,
};
use bevy_ecs::{
    entity::Entity,
    world::{EntityWorldMut, World},
};
use bevy_hierarchy::BuildWorldChildren;
use dyn_comp_types::{events::InputEvent, mixins::Root};
use std::collections::HashMap;

pub struct DtifInjector {
    /// Maps Ids of type String (sid) from the DTIF to actual spawned Bevy entities.
    sid_to_entity: HashMap<String, Entity>,
}

impl DtifInjector {
    pub fn new() -> Self {
        Self {
            sid_to_entity: HashMap::default(),
        }
    }

    pub fn drain_sid_to_entity(&mut self) -> HashMap<String, Entity> {
        self.sid_to_entity.drain().collect()
    }

    pub fn inject_from_root(&mut self, dtif: &CompDtif, world: &mut World) -> Option<Entity> {
        // Process paints (before nodes as nodes can reference paint sid's)
        self.process_paints(dtif, world);

        // Process nodes starting from the root
        let maybe_root_node_entity = self.process_node(dtif.root_node_id.clone(), dtif, world);

        if let Some(root_node_entity) = maybe_root_node_entity {
            world.entity_mut(root_node_entity).insert(Root);
            self.inject_input_events(&dtif.events, world);
        }

        return maybe_root_node_entity;
    }

    fn process_node(
        &mut self,
        node_sid: String,
        dtif: &CompDtif,
        world: &mut World,
    ) -> Option<Entity> {
        dtif.nodes.get(&node_sid).map(|node| {
            let node_entity = self.spawn_node(node, world).id();
            self.sid_to_entity.insert(node_sid, node_entity);

            self.process_node_children(node_entity, node, dtif, world);

            return node_entity;
        })
    }

    fn spawn_node<'a>(&self, node: &Node, world: &'a mut World) -> EntityWorldMut<'a> {
        match node {
            Node::Frame(node) => world.spawn(node.to_ecs_bundle(&self.sid_to_entity)),
            Node::Group(node) => world.spawn(node.to_ecs_bundle(&self.sid_to_entity)),
            Node::Rectangle(node) => world.spawn(node.to_ecs_bundle(&self.sid_to_entity)),
        }
    }

    fn process_node_children(
        &mut self,
        parent_entity: Entity,
        node: &Node,
        dtif: &CompDtif,
        world: &mut World,
    ) {
        if let Node::Frame(FrameNode { children, .. }) | Node::Group(GroupNode { children, .. }) =
            node
        {
            // Process child nodes and collect their Bevy entity ids
            let new_children: Vec<Entity> = children
                .iter()
                .filter_map(|child_sid| self.process_node(child_sid.clone(), dtif, world))
                .collect();

            // Establish Bevy parent-child relationships
            if !new_children.is_empty() {
                world.entity_mut(parent_entity).push_children(&new_children);
            }
        }
    }

    fn process_paints(&mut self, dtif: &CompDtif, world: &mut World) {
        dtif.paints.iter().for_each(|(id, paint)| {
            let paint_entity = self.spawn_paint(&paint, world).id();
            self.sid_to_entity.insert(id.clone(), paint_entity);
        });
    }

    fn spawn_paint<'a>(&self, paint: &Paint, world: &'a mut World) -> EntityWorldMut<'a> {
        match paint {
            Paint::Solid(paint) => world.spawn(paint.to_ecs_bundle(&self.sid_to_entity)),
        }
    }

    fn inject_input_events(&self, events: &Vec<DtifInputEvent>, world: &mut World) {
        events
            .iter()
            .cloned()
            .map(|event| event.to_comp_input_event(&self.sid_to_entity))
            .for_each(|maybe_event| {
                if let Some(event) = maybe_event {
                    event.send_into_ecs(world);
                }
            });
    }

    /// Converts an `Entity` to an Id of type String (sid) used to reference elements in DTIF.
    #[inline]
    pub fn entity_to_sid(entity: &Entity) -> String {
        entity.to_bits().to_string()
    }
}
