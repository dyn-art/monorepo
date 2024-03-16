use bevy_ecs::{
    event::EventReader,
    query::With,
    system::{ParamSet, Query, ResMut},
};

use crate::modules::{
    composition::resources::composition::CompositionRes,
    interactive_composition::{
        events::CursorMovedOnComposition,
        resources::{InteractionMode, InteractiveCompositionRes},
    },
    node::components::{
        mixins::{AbsoluteTransformMixin, DimensionMixin, RelativeTransformMixin},
        states::Selected,
    },
};

use self::{
    dragging::handle_dragging, resizing::handle_resizing, rotating::handle_rotating,
    translating::handle_translating,
};

mod dragging;
mod resizing;
mod rotating;
mod translating;

pub fn handle_cursor_moved_on_composition(
    mut event_reader: EventReader<CursorMovedOnComposition>,
    mut interactive_composition: ResMut<InteractiveCompositionRes>,
    mut composition: ResMut<CompositionRes>,
    // https://bevy-cheatbook.github.io/programming/paramset.html
    mut selected_nodes_query: ParamSet<(
        // Translating
        Query<&mut RelativeTransformMixin, With<Selected>>,
        // Resizing
        Query<(&mut RelativeTransformMixin, &mut DimensionMixin), With<Selected>>,
        // Rotating
        Query<
            (
                &mut RelativeTransformMixin,
                &AbsoluteTransformMixin,
                &mut DimensionMixin,
            ),
            With<Selected>,
        >,
    )>,
) {
    for event in event_reader.read() {
        match &mut interactive_composition.interaction_mode {
            InteractionMode::Translating { current, .. } => {
                handle_translating(&composition, &mut selected_nodes_query.p0(), event, current)
            }
            InteractionMode::Resizing {
                corner,
                initial_bounds,
                ..
            } => handle_resizing(
                &composition,
                &mut selected_nodes_query.p1(),
                event,
                *corner,
                initial_bounds,
            ),
            InteractionMode::Rotating {
                corner,
                initial_rotation_in_radians,
                rotation_in_degrees,
            } => handle_rotating(
                &composition,
                &mut selected_nodes_query.p2(),
                event,
                *corner,
                *initial_rotation_in_radians,
                rotation_in_degrees,
            ),
            InteractionMode::Dragging { current } => {
                handle_dragging(&mut composition, event, current)
            }
            _ => {}
        }
    }
}
