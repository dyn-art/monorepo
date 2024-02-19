use std::collections::HashMap;

use dyn_comp_types::{
    bevy_ecs::{
        entity::Entity,
        world::{EntityWorldMut, World},
    },
    bevy_hierarchy::BuildWorldChildren,
};

use crate::{
    node::{FrameNode, GroupNode, Node, NodeImpl},
    DTIFComp,
};

pub struct DTIFInjector {
    /// Maps Ids of type String (sid) from the DTIF to actual spawned Bevy entities.
    sid_to_entity: HashMap<String, Entity>,
}

impl DTIFInjector {
    pub fn new() -> Self {
        Self {
            sid_to_entity: HashMap::default(),
        }
    }

    pub fn inject_from_root(&mut self, dtif: &DTIFComp, world: &mut World) -> Option<Entity> {
        self.process_node(dtif.root_node_id.clone(), dtif, world)
    }

    fn process_node(
        &mut self,
        node_sid: String,
        dtif: &DTIFComp,
        world: &mut World,
    ) -> Option<Entity> {
        dtif.nodes.get(&node_sid).map(|node| {
            let node_entity = self.spawn_node(node, world).id();
            self.sid_to_entity.insert(node_sid, node_entity);

            self.process_children(node_entity, node, dtif, world);

            return node_entity;
        })
    }

    fn spawn_node<'a>(&self, node: &Node, world: &'a mut World) -> EntityWorldMut<'a> {
        match node {
            Node::Frame(frame) => world.spawn(frame.to_ecs_bundle()),
            Node::Group(group) => world.spawn(group.to_ecs_bundle()),
            Node::Rectangle(rectangle) => world.spawn(rectangle.to_ecs_bundle()),
        }
    }

    fn process_children(
        &mut self,
        parent_entity: Entity,
        node: &Node,
        dtif: &DTIFComp,
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
}
