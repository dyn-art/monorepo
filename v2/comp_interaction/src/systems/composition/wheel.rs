use crate::events::WheeledOnCompInputEvent;
use bevy_ecs::{event::EventReader, system::ResMut};
use dyn_comp_common::common::{Size, Viewport};
use dyn_comp_core::resources::composition::CompositionRes;
use glam::Vec2;

pub fn handle_wheel_on_comp_event(
    mut event_reader: EventReader<WheeledOnCompInputEvent>,
    mut comp_res: ResMut<CompositionRes>,
) {
    for event in event_reader.read() {
        #[cfg(feature = "tracing")]
        log::info!("[handle_wheel_on_comp_event] {:?}", event);

        let CompositionRes {
            size: Size(size),
            viewport:
                Viewport {
                    physical_position,
                    physical_size,
                },
            ..
        } = comp_res.as_mut();

        let WheeledOnCompInputEvent {
            position: cursor_position,
            delta,
            ctrl_key_pressed,
            meta_key_pressed,
        } = &event;
        let zoom_factor = 0.7;

        if *ctrl_key_pressed || *meta_key_pressed {
            let scale_factor = if delta.y < 0.0 {
                1.0 / zoom_factor
            } else {
                zoom_factor
            };

            // Calculate relative cursor position within the composition
            let relative_cursor = (*cursor_position / Vec2::new(size.x, size.x))
                * Vec2::new(physical_size.x, physical_size.y)
                + Vec2::new(physical_position.x, physical_position.y);

            let new_size = Vec2::new(size.x, size.y) * scale_factor;
            let new_physical_position =
                relative_cursor - (*cursor_position / Vec2::new(size.x, size.y)) * new_size;

            // Update the composition's viewport
            physical_position.x = new_physical_position.x;
            physical_position.y = new_physical_position.y;
            size.x = new_size.x;
            size.y = new_size.y;
        } else {
            physical_position.x += delta.x;
            physical_position.y += delta.y;
        }
    }
}
