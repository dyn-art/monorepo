use bevy_ecs::{event::EventReader, system::Query};
use glam::{Mat3, Vec2};
use log::info;

use crate::core::modules::node::components::mixins::RelativeTransformMixin;

use super::events::{
    CursorDownOnEntity, CursorEnteredComposition, CursorExitedComposition, CursorMovedOnComposition,
};

pub fn handle_cursor_down_on_entity_event(
    mut event_reader: EventReader<CursorDownOnEntity>,
    mut query: Query<&mut RelativeTransformMixin>,
) {
    for event in event_reader.read() {
        let CursorDownOnEntity { entity } = event;
        info!("handle_cursor_down_on_entity_event: {:#?}", entity);
        // TODO: also frame is moved as cursor event also includes frame
        if let Ok(mut mixin) = query.get_mut(*entity) {
            info!("in relative transform move? {:#?}", mixin);
            let translation = Mat3::from_translation(Vec2::new(50.0, 50.0));
            mixin.0 = mixin.0 * translation;
        }
    }
}

pub fn handle_cursor_moved_on_composition(mut event_reader: EventReader<CursorMovedOnComposition>) {
    for event in event_reader.read() {
        let CursorMovedOnComposition { position } = event;
        info!("handle_cursor_moved_on_composition: {:#?}", position);
    }
}

pub fn handle_cursor_entered_composition(mut event_reader: EventReader<CursorEnteredComposition>) {
    for event in event_reader.read() {
        info!("handle_cursor_entered_composition");
    }
}

pub fn handle_cursor_exited_composition(mut event_reader: EventReader<CursorExitedComposition>) {
    for event in event_reader.read() {
        info!("handle_cursor_exited_composition");
    }
}
