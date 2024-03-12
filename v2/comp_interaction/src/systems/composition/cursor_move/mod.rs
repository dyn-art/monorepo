mod dragging;
mod resizing;
mod rotating;
mod translating;

use self::{
    dragging::handle_dragging, resizing::handle_resizing, rotating::handle_rotating,
    translating::handle_translating,
};
use crate::{
    components::Selected,
    events::CursorMovedOnCompInputEvent,
    resources::comp_interaction::{CompInteractionRes, InteractionMode},
};
use bevy_ecs::{
    event::EventReader,
    query::With,
    system::{ParamSet, Query, ResMut},
};
use bevy_transform::components::{GlobalTransform, Transform};
use dyn_comp_common::mixins::SizeMixin;
use dyn_comp_core::resources::composition::CompositionRes;

pub fn handle_cursor_moved_on_comp_event(
    mut event_reader: EventReader<CursorMovedOnCompInputEvent>,
    mut comp_interaction_res: ResMut<CompInteractionRes>,
    mut comp_res: ResMut<CompositionRes>,
    // https://bevy-cheatbook.github.io/programming/paramset.html
    mut query_set: ParamSet<(
        // Translating
        Query<&mut Transform, With<Selected>>,
        // Resizing
        Query<(&mut Transform, &mut SizeMixin), With<Selected>>,
        // Rotating
        Query<(&mut Transform, &GlobalTransform, &SizeMixin), With<Selected>>,
    )>,
) {
    for event in event_reader.read() {
        match &mut comp_interaction_res.interaction_mode {
            InteractionMode::Translating { current, .. } => {
                handle_translating(&comp_res, &mut query_set.p0(), event, current)
            }
            InteractionMode::Resizing {
                corner,
                initial_bounds,
                ..
            } => handle_resizing(
                &comp_res,
                &mut query_set.p1(),
                event,
                *corner,
                initial_bounds,
            ),
            InteractionMode::Rotating {
                corner,
                initial_rotation_in_radians,
                rotation_in_degrees,
            } => handle_rotating(
                &comp_res,
                &mut query_set.p2(),
                event,
                *corner,
                *initial_rotation_in_radians,
                rotation_in_degrees,
            ),
            InteractionMode::Dragging { current } => handle_dragging(&mut comp_res, event, current),
            _ => {}
        }
    }
}
