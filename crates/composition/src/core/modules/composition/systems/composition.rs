use bevy_ecs::{event::EventReader, system::ResMut};

use crate::core::modules::composition::{
    events::CompositionResized, resources::composition::CompositionRes,
};

pub fn handle_composition_resized(
    mut event_reader: EventReader<CompositionResized>,
    mut composition_res: ResMut<CompositionRes>,
) {
    for event in event_reader.read() {
        let CompositionResized { width, height } = event;
        composition_res.width = *width;
        composition_res.height = *height;
    }
}
