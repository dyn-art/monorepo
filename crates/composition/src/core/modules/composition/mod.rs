use bevy_app::{Plugin, PreUpdate};
use bevy_ecs::world::World;

use crate::core::dtif::{dtif_processor::DTIFProcessor, DTIFComposition};

use self::{
    components::CompositionMixin,
    events::{EntityMoved, EntitySetPosition},
    systems::layout::{handle_entity_moved_events, handle_entity_set_position_events},
};

use super::node::components::types::Root;

pub mod components;
pub mod events;
mod systems;

pub struct CompositionPlugin {
    pub dtif: Option<DTIFComposition>,
}

impl Plugin for CompositionPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        // Register events
        app.add_event::<EntityMoved>();
        app.add_event::<EntitySetPosition>();

        // Register systems
        app.add_systems(
            PreUpdate,
            (
                handle_entity_moved_events,
                handle_entity_set_position_events,
            ),
        );

        // Load DTIF
        if let Some(dtif) = &self.dtif {
            insert_dtif_into_world(&mut app.world, dtif);
        }
    }
}

fn insert_dtif_into_world(world: &mut World, dtif: &DTIFComposition) {
    let root_node_eid = DTIFProcessor::entity_to_eid(&dtif.root_node_id);
    let mut dtif_processor = DTIFProcessor::new();

    // Spawn and process nodes recursively
    let root_node_entity = dtif_processor
        .process_node(root_node_eid, world, dtif)
        .unwrap();
    world.entity_mut(root_node_entity).insert(Root);

    // Apply DTIF changes
    if let Some(changes) = &dtif.changes {
        for change in changes {
            dtif_processor.send_event_into_world(change.clone(), world)
        }
    }

    // Spawn composition entity
    world.spawn(CompositionMixin {
        version: dtif.version.clone(),
        name: dtif.name.clone(),
        width: dtif.width,
        height: dtif.height,
        root_node: root_node_entity,
    });
}
