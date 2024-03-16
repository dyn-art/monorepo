use bevy_ecs::{event::EventReader, system::ResMut};

use crate::modules::composition::{
    events::{CompositionResized, CompositionViewBoxChanged},
    resources::composition::CompositionRes,
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

pub fn handle_composition_view_box_changed(
    mut event_reader: EventReader<CompositionViewBoxChanged>,
    mut composition_res: ResMut<CompositionRes>,
) {
    for event in event_reader.read() {
        let CompositionViewBoxChanged { view_box } = event;
        composition_res.view_box = *view_box;
    }
}
