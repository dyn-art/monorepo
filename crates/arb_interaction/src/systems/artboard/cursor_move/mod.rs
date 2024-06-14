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
    events::CursorMovedOnArbInputEvent,
    resources::arb_interaction::{ArbInteractionRes, InteractionMode},
};
use bevy_ecs::{
    entity::Entity,
    event::EventReader,
    query::With,
    system::{Commands, ParamSet, Query, ResMut},
};
use bevy_hierarchy::Parent;
use bevy_transform::components::{GlobalTransform, Transform};
use dyn_arb_bundles::components::{
    marker::Root,
    mixins::{SizeMixin, StaticLayoutElementMixin, StaticLayoutParentMixin},
    nodes::{ArbNode, TextArbNode},
};
use dyn_arb_core::resources::canvas::ArtboardRes;

pub fn cursor_moved_on_arb_input_system(
    mut commands: Commands,
    mut event_reader: EventReader<CursorMovedOnArbInputEvent>,
    mut arb_interaction_res: ResMut<ArbInteractionRes>,
    mut arb_res: ResMut<ArtboardRes>,
    // https://bevy-cheatbook.github.io/programming/paramset.html
    mut query_set: ParamSet<(
        // Translating
        Query<(&mut Transform, Option<&Parent>), With<Selected>>,
        // Resizing
        Query<
            (
                &mut Transform,
                &mut SizeMixin,
                Option<&Parent>,
                Option<&mut StaticLayoutParentMixin>,
                Option<&mut StaticLayoutElementMixin>,
                Option<&mut TextArbNode>,
            ),
            With<Selected>,
        >,
        // Rotating
        Query<(&mut Transform, &GlobalTransform, &SizeMixin), With<Selected>>,
        // Inserting
        Query<(&mut Transform, &mut SizeMixin, Option<&Parent>)>,
    )>,
    global_transfrom_query: Query<&GlobalTransform>,
    root_node_query: Query<Entity, (With<ArbNode>, With<Root>)>,
) {
    for event in event_reader.read() {
        match &mut arb_interaction_res.interaction_mode {
            InteractionMode::Translating { current, .. } => handle_translating(
                &arb_res,
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
                &arb_res,
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
                &arb_res,
                &mut query_set.p2(),
                event,
                *corner,
                *initial_rotation_rad,
                rotation_deg,
            ),
            InteractionMode::Dragging { current } => handle_dragging(&mut arb_res, event, current),
            InteractionMode::Inserting {
                entity,
                origin,
                shape_variant,
            } => handle_inserting(
                &mut commands,
                &mut arb_res,
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
