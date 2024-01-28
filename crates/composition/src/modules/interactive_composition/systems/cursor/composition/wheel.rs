use bevy_ecs::{event::EventReader, system::ResMut};
use glam::Vec2;

use crate::modules::{
    composition::resources::composition::CompositionRes,
    interactive_composition::events::WheeledOnComposition,
};

pub fn handle_wheel_on_composition(
    mut event_reader: EventReader<WheeledOnComposition>,
    mut composition: ResMut<CompositionRes>,
) {
    for event in event_reader.read() {
        #[cfg(feature = "tracing")]
        log::info!("handle_wheel_on_composition: {:#?}", event);

        let WheeledOnComposition {
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
            let relative_cursor = (*cursor_position
                / Vec2::new(composition.width, composition.height))
                * Vec2::new(composition.view_box.width, composition.view_box.height)
                + Vec2::new(composition.view_box.min_x, composition.view_box.min_y);

            let new_dimensions =
                Vec2::new(composition.view_box.width, composition.view_box.height) * scale_factor;
            let new_min = relative_cursor
                - (*cursor_position / Vec2::new(composition.width, composition.height))
                    * new_dimensions;

            // Update the composition's view box
            composition.view_box.min_x = new_min.x;
            composition.view_box.min_y = new_min.y;
            composition.view_box.width = new_dimensions.x;
            composition.view_box.height = new_dimensions.y;
        } else {
            composition.view_box.min_x += delta.x;
            composition.view_box.min_y += delta.y;
        }
    }
}
