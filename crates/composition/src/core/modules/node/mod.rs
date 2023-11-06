use bevy_app::{App, Plugin, PostUpdate, Update};

use self::systems::{
    construct_path::construct_rectangle_path,
    layout::{handle_entity_moved_events, handle_entity_set_position_events},
};

pub mod components;
mod systems;

pub struct NodePlugin;

impl Plugin for NodePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_entity_moved_events,
                handle_entity_set_position_events,
            ),
        )
        .add_systems(PostUpdate, construct_rectangle_path);
    }
}
