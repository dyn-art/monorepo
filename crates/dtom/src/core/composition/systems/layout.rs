use bevy_ecs::{event::EventReader, system::Query};
use glam::{Mat3, Vec2};

use crate::core::composition::{
    events::input_event::{EntityMoved, EntitySetPosition},
    nodes::mixins::LayoutMixin,
};

pub fn handle_entity_moved_events(
    mut event_reader: EventReader<EntityMoved>,
    mut query: Query<&mut LayoutMixin>,
) {
    for event in event_reader.read() {
        let EntityMoved { entity, dx, dy } = event;
        if let Ok(mut mixin) = query.get_mut(*entity) {
            let translation = Mat3::from_translation(Vec2::new(*dx, *dy));
            mixin.relative_transform = mixin.relative_transform * translation;
        }
    }
}

pub fn handle_entity_set_position_events(
    mut event_reader: EventReader<EntitySetPosition>,
    mut query: Query<&mut LayoutMixin>,
) {
    for event in event_reader.read() {
        let EntitySetPosition { entity, x, y } = event;
        if let Ok(mut mixin) = query.get_mut(*entity) {
            mixin.relative_transform.col_mut(2).x = *x;
            mixin.relative_transform.col_mut(2).y = *y;
        }
    }
}
