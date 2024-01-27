use bevy_app::{Plugin, PreUpdate};
use bevy_ecs::world::World;

use crate::core::dtif::{dtif_processor::DTIFProcessor, DTIFComposition};

use self::{
    events::{
        CompositionResized, CompositionViewBoxChanged, EntityMoved, EntitySetPosition, NodeCreated,
        NodeDeleted,
    },
    resources::{
        composition::{CompositionRes, ViewBox},
        font_cache::{font::FontContent, FontCacheRes},
    },
    systems::{
        composition::{handle_composition_resized, handle_composition_view_box_changed},
        entity::{handle_entity_moved, handle_entity_set_position},
        node::handle_node_deleted,
    },
};

use super::node::components::types::Root;

pub mod events;
pub mod resources;
mod systems;

pub struct CompositionPlugin {
    pub dtif: DTIFComposition,
}

impl Plugin for CompositionPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        // Register events
        app.add_event::<CompositionResized>();
        app.add_event::<CompositionViewBoxChanged>();
        app.add_event::<EntityMoved>();
        app.add_event::<EntitySetPosition>();
        app.add_event::<NodeCreated>();
        app.add_event::<NodeDeleted>();

        // Register resources
        app.world.init_resource::<FontCacheRes>();

        // Register systems
        app.add_systems(
            PreUpdate,
            (
                handle_composition_resized,
                handle_composition_view_box_changed,
                handle_entity_moved,
                handle_entity_set_position,
                // handle_node_deleted, // TODO
                // handle_node_created, // TODO: damaged
            ),
        );
        #[cfg(feature = "interactive")]
        app.add_systems(
            bevy_app::PostUpdate,
            systems::entity::interactive::calculate_absolute_transform,
        );

        // Load DTIF
        insert_dtif_into_world(&mut app.world, &self.dtif);
    }
}

fn insert_dtif_into_world(world: &mut World, dtif: &DTIFComposition) {
    let root_node_eid = DTIFProcessor::entity_to_eid(&dtif.root_node_id);
    let mut dtif_processor = DTIFProcessor::new();

    // Load fonts into cache
    if let Some(fonts) = &dtif.fonts {
        for (id, font) in fonts.clone().into_iter() {
            if let Some(mut font_cache) = world.get_resource_mut::<FontCacheRes>() {
                match font.content {
                    FontContent::Binary { content } => {
                        font_cache.insert(id.parse().unwrap(), font.metadata, content);
                    }
                    FontContent::Url { url } => {
                        // TODO: Add URL resolve functionality once Bevy supports async plugin creation,
                        // or somehow allow blocking in a Tokio environment work ("Cannot drop a runtime in a context where blocking is not allowed...")
                        // https://github.com/bevyengine/bevy/discussions/3239
                        #[cfg(feature = "resolve-url")]
                        log::error!("URL resolve feature not implemented yet!");

                        #[cfg(not(feature = "resolve-url"))]
                        log::warn!("URL font loading not supported in this build. Use 'resolve-url' feature to activate this functionality.");
                    }
                }
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
        view_box: dtif.view_box.unwrap_or(ViewBox {
            width: dtif.width,
            height: dtif.height,
            min_y: 0.0,
            min_x: 0.0,
        }),
    });
}
