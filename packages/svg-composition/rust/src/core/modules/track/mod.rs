use bevy_app::{App, Plugin, Update};

use self::{resources::TrackedEntities, systems::track_changes};

pub mod resources;
mod systems;

pub struct TrackPlugin;

impl Plugin for TrackPlugin {
    fn build(&self, app: &mut App) {
        // Register resources
        app.init_resource::<TrackedEntities>();

        // Register systems
        app.add_systems(Update, track_changes);
    }
}
