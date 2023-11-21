use bevy_app::{App, Plugin, PostUpdate};

use self::systems::construct_path::construct_rectangle_path;

pub mod components;
mod systems;

pub struct NodePlugin;

impl Plugin for NodePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, construct_rectangle_path);
    }
}
