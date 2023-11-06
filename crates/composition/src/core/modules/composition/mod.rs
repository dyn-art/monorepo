use std::collections::HashMap;

use bevy_app::Plugin;
use bevy_ecs::{entity::Entity, world::World};

use crate::core::dtif::{entity_to_eid, process_dtif_nodes, DTIFComposition};

use self::{
    components::{CompositionInteractionMixin, CompositionMixin},
    events::input_event::{
        CursorDownOnEntity, CursorEnteredComposition, CursorExitedComposition,
        CursorMovedOnComposition, EntityMoved, EntitySetPosition,
    },
};

pub mod components;
pub mod events;

pub struct CompositionPlugin {
    pub dtif: Option<DTIFComposition>,
}

impl Plugin for CompositionPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        // Register events
        app.add_event::<CursorMovedOnComposition>();
        app.add_event::<CursorEnteredComposition>();
        app.add_event::<CursorExitedComposition>();
        app.add_event::<CursorDownOnEntity>();
        app.add_event::<EntityMoved>();
        app.add_event::<EntitySetPosition>();

        // Load DTIF
        if let Some(dtif) = &self.dtif {
            insert_dtif(&mut app.world, &dtif);
            // TODO: clear storage DTIF takes
        }
    }
}

fn insert_dtif(world: &mut World, dtif: &DTIFComposition) {
    let root_node_eid = entity_to_eid(&dtif.root_node_id);
    let mut eid_to_entity_map: HashMap<String, Entity> = HashMap::new();

    // Spawn and process nodes recursively
    let root_node_entity =
        process_dtif_nodes(world, &dtif.nodes, &root_node_eid, &mut eid_to_entity_map).unwrap();

    // Spawn composition as entity (only one should exist).
    // Why entity? Because I see it as part of the "game" world,
    // and to spawn it with values passed from JS.
    world.spawn(CompositionMixin {
        version: dtif.version.clone(),
        name: dtif.name.clone(),
        width: dtif.width,
        height: dtif.height,
        root_node: root_node_entity,
    });
    world.spawn(CompositionInteractionMixin::default());
}
