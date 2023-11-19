use bevy_app::Plugin;
use bevy_ecs::world::World;

use crate::core::dtif::{dtif_processor::DTIFProcessor, DTIFComposition};

use self::{
    components::CompositionMixin,
    events::{EntityMoved, EntitySetPosition},
};

use super::node::components::types::Root;

pub mod components;
pub mod events;

pub struct CompositionPlugin {
    pub dtif: Option<DTIFComposition>,
}

impl Plugin for CompositionPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        // Register events
        app.add_event::<EntityMoved>();
        app.add_event::<EntitySetPosition>();

        // Load DTIF
        if let Some(dtif) = &self.dtif {
            insert_dtif(&mut app.world, dtif);
        }
    }
}

fn insert_dtif(world: &mut World, dtif: &DTIFComposition) {
    let root_node_eid = DTIFProcessor::entity_to_eid(&dtif.root_node_id);
    let mut dtif_processor = DTIFProcessor::new();

    // Spawn and process nodes recursively
    let root_node_entity = dtif_processor
        .process_node(&root_node_eid, world, dtif)
        .unwrap();
    world.entity_mut(root_node_entity).insert(Root);

    // Apply changes
    if let Some(changes) = &dtif.changes {
        for change in changes {
            dtif_processor.send_event_to_ecs(world, change.clone())
        }
    }

    // Spawn composition as entity (only one should exist).
    // Why entity? Because we see it as part of the "game" world.
    world.spawn(CompositionMixin {
        version: dtif.version.clone(),
        name: dtif.name.clone(),
        width: dtif.width,
        height: dtif.height,
        root_node: root_node_entity,
    });
}
