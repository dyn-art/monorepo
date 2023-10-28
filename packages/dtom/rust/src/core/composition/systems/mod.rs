use bevy_ecs::{event::EventReader, system::Query};
use glam::{Mat3, Vec2};
use log::info;

use crate::core::node::mixins::LayoutMixin;

use super::events::EntityMoved;

pub mod construct_path;

pub fn handle_entity_moved_events(
    mut event_reader: EventReader<EntityMoved>,
    mut query: Query<&mut LayoutMixin>,
) {
    for event in event_reader.iter() {
        let EntityMoved { entity, dx, dy } = event;
        if let Ok(mut mixin) = query.get_mut(*entity) {
            // Create a translation matrix
            let translation = Mat3::from_translation(Vec2::new(*dx, *dy));

            // Combine it with the existing relative_transform
            mixin.relative_transform = mixin.relative_transform * translation;
        }
    }
}
