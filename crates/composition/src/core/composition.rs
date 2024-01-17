use bevy_app::App;
use bevy_ecs::{bundle::Bundle, entity::Entity, query::With};
use bevy_hierarchy::BuildWorldChildren;
use dyn_bevy_render_skeleton::RenderPlugin;

use crate::core::modules::composition::CompositionPlugin;

use super::{
    dtif::DTIFComposition,
    events::input_event::InputEvent,
    modules::{
        composition::resources::composition::CompositionRes,
        node::{
            components::{bundles::RectangleNodeBundle, types::Root},
            NodePlugin,
        },
    },
};

pub struct Composition {
    app: App,
}

impl Composition {
    pub fn new(dtif: DTIFComposition) -> Self {
        let mut app = App::new();
        let width = dtif.width;
        let height = dtif.height;

        // Register plugins
        app.add_plugins((RenderPlugin, NodePlugin, CompositionPlugin { dtif }));
        #[cfg(feature = "interactive")]
        app.add_plugins(
            super::modules::interactive_composition::InteractiveCompositionPlugin { width, height },
        );

        return Self { app };
    }

    // =========================================================================
    // Getter & Setter
    // =========================================================================

    pub fn get_app(&self) -> &App {
        &self.app
    }

    pub fn get_app_mut(&mut self) -> &mut App {
        &mut self.app
    }

    // =========================================================================
    // Lifecycle
    // =========================================================================

    pub fn update(&mut self) {
        self.app.update();
    }

    // =========================================================================
    // Spawn
    // =========================================================================

    // TODO: Try to solve with 'NodeCreated' event as its basically the same logic,
    //  but ofc it can't directly return the created Entity
    //  -> Solve with callback id where the Entity is sent back under the callback id or so

    pub fn spawn_rectangle_node(
        &mut self,
        bundle: RectangleNodeBundle,
        maybe_parent_id: Option<Entity>,
    ) -> Entity {
        let paint_ids = bundle.fill_mixin.paint_ids.clone();
        let entity_id = self.spawn_node(bundle, maybe_parent_id);

        // TODO
        if let Some(mut entity) = self.app.world.get_entity_mut(entity_id) {
            entity.push_children(&paint_ids);
        }

        // TOOD: Set absolute position

        return entity_id;
    }

    pub fn spawn_node<B: Bundle + std::fmt::Debug>(
        &mut self,
        bundle: B,
        maybe_parent_id: Option<Entity>,
    ) -> Entity {
        let entity_id = self.app.world.spawn::<B>(bundle).id();

        // If no parent id provided the root node will become the parent
        let maybe_parent_id = maybe_parent_id.or_else(|| {
            self.app
                .world
                .query_filtered::<Entity, With<Root>>()
                .iter(&self.app.world)
                .next()
        });

        // Establish potential parent child relation
        if let Some(parent_id) = maybe_parent_id {
            if let Some(mut entity) = self.app.world.get_entity_mut(parent_id) {
                entity.push_children(&[entity_id]);
            }
        }

        return entity_id;
    }

    // =========================================================================
    // Events
    // =========================================================================

    pub fn register_events<T: InputEvent>(&mut self, events: Vec<T>) {
        for event in events {
            self.register_event(event);
        }
    }

    pub fn register_event<T: InputEvent>(&mut self, event: T) {
        event.send_to_ecs(&mut self.app.world);
    }

    // =========================================================================
    // Other
    // =========================================================================

    pub fn set_size(&mut self, width: f32, height: f32) {
        let maybe_composition_res = self.app.world.get_resource_mut::<CompositionRes>();
        if let Some(mut composition_res) = maybe_composition_res {
            composition_res.width = width;
            composition_res.height = height;
        }
    }

    pub fn clear(&mut self) {
        self.app.world.clear_all();
    }
}
