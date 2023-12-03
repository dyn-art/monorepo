use bevy_app::App;
use bevy_ecs::{bundle::Bundle, entity::Entity, query::With};
use bevy_hierarchy::BuildWorldChildren;
use dyn_bevy_render_skeleton::RenderPlugin;

use crate::core::modules::{
    composition::CompositionPlugin, interactive_composition::InteractiveCompositionPlugin,
};

use super::{
    dtif::DTIFComposition,
    events::input_event::InputEvent,
    modules::node::{
        components::{bundles::RectangleNodeBundle, types::Root},
        utils::logging::log_entity_components,
        NodePlugin,
    },
};

pub struct Composition {
    app: App,
}

impl Composition {
    pub fn new(dtif: Option<DTIFComposition>) -> Self {
        let mut app = App::new();

        // Register plugins
        app.add_plugins((
            RenderPlugin,
            NodePlugin,
            CompositionPlugin { dtif },
            InteractiveCompositionPlugin, // TODO: only include if "interaction" feature active?
        ));

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

    pub fn spawn_rectangle_node(
        &mut self,
        bundle: RectangleNodeBundle,
        maybe_parent_id: Option<Entity>,
    ) -> Entity {
        let paint_ids = bundle.fill_mixin.paints.clone();
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

    pub fn clear(&mut self) {
        self.app.world.clear_all();
    }

    #[cfg(feature = "trace")]
    pub fn log_entity_components(&self, entity: Entity) {
        log_entity_components(&self.app.world, entity);
    }
}
