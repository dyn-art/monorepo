use std::collections::HashMap;

use bevy_ecs::{entity::Entity, world::World};
use serde::{Deserialize, Serialize};
use specta::Type;

use super::modules::{
    composition::events::CoreInputEvent,
    node::components::{
        bundles::{FrameNodeBundle, GroupNodeBundle, RectangleNodeBundle},
        mixins::{ChildrenMixin, ParentMixin},
    },
};

#[derive(Serialize, Deserialize, Debug, Type)]
pub struct DTIFComposition {
    pub version: String,
    pub name: String,
    pub width: f32,
    pub height: f32,
    #[serde(rename = "rootNodeId")]
    pub root_node_id: Entity,
    pub nodes: HashMap<String, DTIFNode>, // TODO: Entity as key when fixed: https://github.com/serde-rs/serde/issues/1183
    #[serde(default)]
    pub changes: Option<Vec<CoreInputEvent>>,
}

#[derive(Serialize, Deserialize, Debug, Type)]
#[serde(tag = "type")]
pub enum DTIFNode {
    Rectangle(RectangleNodeBundle),
    Frame(FrameNodeBundle),
    Group(GroupNodeBundle),
}

pub struct DTIFProcessor {
    eid_to_entity: HashMap<String, Entity>,
}

impl DTIFProcessor {
    pub fn new() -> Self {
        DTIFProcessor {
            eid_to_entity: HashMap::new(),
        }
    }

    // Process a single dtif node and its children
    // and update the internal mapping to keep track of ECS id and id used in DTIF
    pub fn process_node(
        &mut self,
        node_eid: &String,
        world: &mut World,
        dtif_nodes: &HashMap<String, DTIFNode>,
    ) -> Option<Entity> {
        if let Some(dtif_node) = dtif_nodes.get(node_eid) {
            let node_entity = self.spawn_node(world, dtif_node);
            self.eid_to_entity.insert(node_eid.clone(), node_entity);

            if let DTIFNode::Frame(FrameNodeBundle { children_mixin, .. })
            | DTIFNode::Group(GroupNodeBundle { children_mixin, .. }) = dtif_node
            {
                let new_children: Vec<Entity> = children_mixin
                    .0
                    .iter()
                    .filter_map(|child_entity| {
                        let child_eid = DTIFProcessor::entity_to_eid(child_entity);
                        let processed_child_entity =
                            self.process_node(&child_eid, world, dtif_nodes)?;

                        // Keep track of parent in children
                        // to easily know where to append it in some render appraoches (e.g. svg)
                        world
                            .entity_mut(processed_child_entity)
                            .insert(ParentMixin(node_entity.clone()));

                        return Some(processed_child_entity);
                    })
                    .collect();

                if !new_children.is_empty() {
                    world
                        .entity_mut(node_entity)
                        .insert(ChildrenMixin(new_children));
                }
            }

            Some(node_entity)
        } else {
            None
        }
    }

    // Spawn a dtif node in the ECS world
    fn spawn_node(&self, world: &mut World, node: &DTIFNode) -> Entity {
        match node {
            DTIFNode::Frame(bundle) => world.spawn(bundle.clone()).id(),
            DTIFNode::Rectangle(bundle) => world.spawn(bundle.clone()).id(),
            DTIFNode::Group(bundle) => world.spawn(bundle.clone()).id(),
        }
    }

    // Translate an entity id from the event to the actual entity.
    pub fn translate_event_entity(&self, event_entity_id: &Entity) -> Option<Entity> {
        let eid = DTIFProcessor::entity_to_eid(event_entity_id);
        self.eid_to_entity.get(&eid).cloned()
    }

    // Process and send the event to the ECS.
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

    // Due to a issue we have to work with a stringified Enitity in the Hashmap.
    // https://github.com/serde-rs/serde/issues/1183
    // This function basically converts an Entity to a string we called "eid".
    #[inline]
    pub fn entity_to_eid(entity: &Entity) -> String {
        entity.to_bits().to_string()
    }
}
