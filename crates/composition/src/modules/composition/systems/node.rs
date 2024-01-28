use bevy_ecs::{
    bundle::Bundle,
    entity::Entity,
    event::EventReader,
    query::With,
    system::{Commands, Query},
};
use bevy_hierarchy::BuildChildren;

use crate::modules::{
    composition::events::{NodeCreated, NodeDeleted},
    node::components::{bundles::NodeBundle, types::Root},
};

pub fn handle_node_deleted(mut commands: Commands, mut event_reader: EventReader<NodeDeleted>) {
    for event in event_reader.read() {
        commands.entity(event.entity).despawn();
    }
}

// TODO: WIP - Not working yet
pub fn handle_node_created(
    mut commands: Commands,
    mut event_reader: EventReader<NodeCreated>,
    root_node_query: Query<Entity, With<Root>>,
) {
    let root_node_id = root_node_query.iter().next();

    for event in event_reader.read() {
        let NodeCreated {
            parent_entity,
            node,
        } = event;

        match node {
            NodeBundle::Rectangle(bundle) => {
                let paint_ids = bundle.fill_mixin.paint_ids.clone();
                let entity =
                    spawn_node(&mut commands, bundle.clone(), *parent_entity, root_node_id);
                if let Some(mut entity) = commands.get_entity(entity) {
                    entity.push_children(&paint_ids);
                }
            }
            // TODO:
            _ => {}
        }
    }
}

pub fn spawn_node<B: Bundle + std::fmt::Debug>(
    commands: &mut Commands,
    bundle: B,
    maybe_parent_id: Option<Entity>,
    root_node_id: Option<Entity>,
) -> Entity {
    let entity_id = commands.spawn::<B>(bundle).id();

    // If no parent id provided the root node will become the parent
    let maybe_parent_id = maybe_parent_id.or_else(|| root_node_id);

    // Establish potential parent child relation
    if let Some(parent_id) = maybe_parent_id {
        if let Some(mut entity) = commands.get_entity(parent_id) {
            entity.push_children(&[entity_id]);
        }
    }

    return entity_id;
}
