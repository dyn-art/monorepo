use crate::events::WheeledOnCompInputEvent;
use bevy_ecs::{event::EventReader, system::ResMut};
use dyn_comp_bundles::properties::Viewport;
use dyn_comp_core::resources::composition::CompositionRes;
use dyn_utils::{properties::size::Size, units::abs::Abs};

pub fn handle_wheel_on_comp_event(
    mut event_reader: EventReader<WheeledOnCompInputEvent>,
    mut comp_res: ResMut<CompositionRes>,
) {
    for event in event_reader.read() {
        let CompositionRes {
            size,
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
            let relative_cursor =
                (*cursor_position / size.to_vec2()) * physical_size.to_vec2() + *physical_position;

            let new_physical_size = Size::new(
                Abs::pt(physical_size.width() * scale_factor),
                Abs::pt(physical_size.height() * scale_factor),
            );
            let new_physical_position =
                relative_cursor - (*cursor_position / size.to_vec2()) * new_physical_size.to_vec2();

            // Update the composition's viewport
            *physical_position = new_physical_position;
            *physical_size = new_physical_size;
        } else {
            *physical_position += *delta;
        }
    }
}
