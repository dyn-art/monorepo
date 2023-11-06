use std::collections::HashMap;

use bevy_ecs::{entity::Entity, world::World};
use serde::{Deserialize, Serialize};
use specta::Type;

use super::modules::node::components::{
    bundles::{FrameNodeBundle, GroupNodeBundle, RectangleNodeBundle},
    mixins::{ChildrenMixin, ParentMixin},
};

#[derive(Serialize, Deserialize, Debug, Type)]
pub struct DTIFComposition {
    pub version: String,
    pub name: String,
    pub width: f32,
    pub height: f32,
    pub root_node_id: Entity,
    pub nodes: HashMap<String, DTIFNode>, // TODO: Entity as key when fixed: https://github.com/serde-rs/serde/issues/1183
}

#[derive(Serialize, Deserialize, Debug, Type)]
pub enum DTIFNode {
    Rectangle(RectangleNodeBundle),
    Frame(FrameNodeBundle),
    Group(GroupNodeBundle),
}

fn spawn_node(world: &mut World, node: &DTIFNode) -> Entity {
    match node {
        DTIFNode::Frame(bundle) => world.spawn(bundle.clone()).id(),
        DTIFNode::Rectangle(bundle) => world.spawn(bundle.clone()).id(),
        DTIFNode::Group(bundle) => world.spawn(bundle.clone()).id(),
    }
}

// Due to a issue we have to work with a stringified Enitity in the Hashmap.
// https://github.com/serde-rs/serde/issues/1183
// This function basically converts an Entity to a string we called "eid".
pub fn entity_to_eid(entity: &Entity) -> String {
    entity.to_bits().to_string()
}

pub fn process_dtif_nodes(
    world: &mut World,
    dtif_nodes: &HashMap<String, DTIFNode>,
    node_eid: &String,
    eid_to_entity: &mut HashMap<String, Entity>,
) -> Option<Entity> {
    // If  node exists, spawn it and process its children
    if let Some(dtif_node) = dtif_nodes.get(node_eid) {
        // Spawn node
        let node_entity = spawn_node(world, dtif_node);
        eid_to_entity.insert(node_eid.clone(), node_entity);

        // Process children
        let mut new_children: Vec<Entity> = vec![];
        if let DTIFNode::Frame(FrameNodeBundle { children_mixin, .. })
        | DTIFNode::Group(GroupNodeBundle { children_mixin, .. }) = dtif_node
        {
            for child_entity in &children_mixin.children {
                let child_eid = entity_to_eid(child_entity);
                let processed_child_entity =
                    process_dtif_nodes(world, dtif_nodes, &child_eid, eid_to_entity).unwrap();
                new_children.push(processed_child_entity);

                // Keep track of parent in children
                // to easily know where to append it in some render appraoches (e.g. svg)
                world
                    .entity_mut(processed_child_entity)
                    .insert(ParentMixin {
                        parent: node_entity.clone(),
                    });
            }

            // Update parent with new children (override old ones)
            if !new_children.is_empty() {
                world.entity_mut(node_entity).insert(ChildrenMixin {
                    children: new_children,
                });
            }
        }

        return Some(node_entity);
    }

    return None;
}
