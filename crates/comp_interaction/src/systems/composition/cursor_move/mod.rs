mod dragging;
mod inserting;
mod resizing;
mod rotating;
mod translating;

use self::{
    dragging::handle_dragging, inserting::handle_inserting, resizing::handle_resizing,
    rotating::handle_rotating, translating::handle_translating,
};
use crate::{
    components::Selected,
    events::CursorMovedOnCompInputEvent,
    resources::comp_interaction::{CompInteractionRes, InteractionMode},
};
use bevy_ecs::{
    entity::Entity,
    event::EventReader,
    query::With,
    system::{Commands, ParamSet, Query, ResMut},
};
use bevy_hierarchy::Parent;
use bevy_transform::components::{GlobalTransform, Transform};
use dyn_comp_bundles::components::{marker::Root, mixins::SizeMixin, nodes::CompNode};
use dyn_comp_core::resources::composition::CompositionRes;

pub fn cursor_moved_on_comp_input_system(
    mut commands: Commands,
    mut event_reader: EventReader<CursorMovedOnCompInputEvent>,
    mut comp_interaction_res: ResMut<CompInteractionRes>,
    mut comp_res: ResMut<CompositionRes>,
    // https://bevy-cheatbook.github.io/programming/paramset.html
    mut query_set: ParamSet<(
        // Translating
        Query<(&mut Transform, Option<&Parent>), With<Selected>>,
        // Resizing
        Query<(&mut Transform, &mut SizeMixin, Option<&Parent>), With<Selected>>,
        // Rotating
        Query<(&mut Transform, &GlobalTransform, &SizeMixin), With<Selected>>,
        // Inserting
        Query<(&mut Transform, &mut SizeMixin, Option<&Parent>)>,
    )>,
    global_transfrom_query: Query<&GlobalTransform>,
    root_node_query: Query<Entity, (With<CompNode>, With<Root>)>,
) {
    for event in event_reader.read() {
        match &mut comp_interaction_res.interaction_mode {
            InteractionMode::Translating { current, .. } => handle_translating(
                &comp_res,
                &mut query_set.p0(),
                &global_transfrom_query,
                event,
                current,
            ),
            InteractionMode::Resizing {
                corner,
                initial_bounds,
                ..
            } => handle_resizing(
                &comp_res,
                &mut query_set.p1(),
                &global_transfrom_query,
                event,
                *corner,
                initial_bounds,
            ),
            InteractionMode::Rotating {
                corner,
                initial_rotation_rad,
                rotation_deg,
            } => handle_rotating(
                &comp_res,
                &mut query_set.p2(),
                event,
                *corner,
                *initial_rotation_rad,
                rotation_deg,
            ),
            InteractionMode::Dragging { current } => handle_dragging(&mut comp_res, event, current),
            InteractionMode::Inserting {
                entity,
                origin,
                shape_variant,
            } => handle_inserting(
                &mut commands,
                &mut comp_res,
                &mut query_set.p3(),
                &root_node_query,
                &global_transfrom_query,
                event,
                entity,
                *shape_variant,
                origin,
            ),
            _ => {}
        }
    }
}
