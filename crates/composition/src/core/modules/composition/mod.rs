use bevy_app::{Plugin, PostUpdate, PreUpdate};
use bevy_ecs::world::World;

use crate::core::dtif::{dtif_processor::DTIFProcessor, DTIFComposition};

use self::{
    events::{EntityMoved, EntitySetPosition},
    resources::{composition::CompositionRes, font_cache::FontCacheRes},
    systems::layout::{
        calculate_absolute_transform, handle_entity_moved, handle_entity_set_position,
    },
};

use super::node::components::types::Root;

pub mod events;
pub mod resources;
mod systems;

pub struct CompositionPlugin {
    pub dtif: Option<DTIFComposition>,
}

impl Plugin for CompositionPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        // Register events
        app.add_event::<EntityMoved>();
        app.add_event::<EntitySetPosition>();

        // Register resources
        app.world.init_resource::<FontCacheRes>();

        // Register systems
        app.add_systems(PreUpdate, (handle_entity_moved, handle_entity_set_position));
        app.add_systems(PostUpdate, calculate_absolute_transform);

        // Load DTIF
        if let Some(dtif) = &self.dtif {
            insert_dtif_into_world(&mut app.world, dtif);
        }
    }
}

fn insert_dtif_into_world(world: &mut World, dtif: &DTIFComposition) {
    let root_node_eid = DTIFProcessor::entity_to_eid(&dtif.root_node_id);
    let mut dtif_processor = DTIFProcessor::new();

    // Load fonts into cache
    if let Some(fonts) = &dtif.fonts {
        for font_with_content in fonts.clone() {
            if let Some(mut font_cache) = world.get_resource_mut::<FontCacheRes>() {
                font_cache.insert_with_hash(
                    font_with_content.hash,
                    font_with_content.font,
                    font_with_content.content,
                );
            }
        }
    }

    // Spawn and process nodes recursively
    let root_node_entity = dtif_processor
        .process_root(root_node_eid, world, dtif)
        .unwrap();
    world.entity_mut(root_node_entity).insert(Root);

    // Apply DTIF changes
    if let Some(changes) = &dtif.changes {
        for change in changes {
            dtif_processor.send_event_into_world(change.clone(), world)
        }
    }

    // Register composition resource
    world.insert_resource(CompositionRes {
        version: dtif.version.clone(),
        name: dtif.name.clone(),
        width: dtif.width,
        height: dtif.height,
        root_node: root_node_entity,
    });
}
