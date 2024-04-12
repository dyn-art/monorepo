use crate::resources::tick::TickRes;
use bevy_ecs::change_detection::DetectChanges;
use bevy_ecs::system::{Res, SystemChangeTick};
use bevy_ecs::world::Ref;
use bevy_ecs::{
    entity::Entity,
    query::Changed,
    system::{Commands, Query},
};
use bevy_hierarchy::Children;
use bevy_transform::components::Transform;
use dyn_comp_bundles::components::mixins::{
    Constraint, Constraints, ConstraintsMixin, ConstraintsOffset, SizeMixin,
};
use dyn_utils::properties::size::Size;

// TODO: Use taffy?
// Don't build any taffy tree by default, instead do it only for a Frame in Auto Layout,
// because for nodes that are children to a non Auto Layout parent it doesn't make sense
// https://github.com/bevyengine/bevy/blob/5caf085dacf74bf553a0428a5eb7f4574a9bb99c/crates/bevy_ui/src/layout/ui_surface.rs

pub fn apply_constraints_offset(
    mut commands: Commands,
    system_change_tick: SystemChangeTick,
    tick_res: Res<TickRes>,
    mut query: Query<(Entity, Ref<Transform>, Option<&mut ConstraintsOffset>), Changed<Transform>>,
) {
    for (entity, transform, maybe_offset) in query.iter_mut() {
        log::info!(
            "[apply_constraints_offset] {:?}: {:?}",
            entity,
            if transform.last_changed().get() >= system_change_tick.last_run().get() {
                (transform.last_changed().get() - system_change_tick.last_run().get()).to_string()
            } else {
                String::from("NaN")
            }
        );
        log::info!(
            "[apply_constraints_offset] {}/{}",
            tick_res.first_in_cycle.get(),
            transform.last_changed().get()
        );

        match maybe_offset {
            // TODO: Solve with storing first system Tick
            //
            // The Component Tick is in my case likely higher
            // due to potential mutable references (e.g., dereferencing Mut<> as &mut).
            // Significant changes, such as those to Transform,
            // produce noticeably larger differences, making them detectable.
            //
            // https://discord.com/channels/691052431525675048/1228316069207216130
            //
            // Component Tick | Last System Tick | This System Tick
            // 21268 | 21267 | 21355 // Updated in last cycle (after this system)
            // 21356 | 21355 | 21444 // Updated in last cycle (after this system)
            // 29457 | 29376 | 29466 // Updated in current cycle (before this system)
            // 29547 | 29466 | 29556 // Updated in current cycle (before this system)
            Some(mut offset) => {
                if transform.last_changed().get() >= system_change_tick.last_run().get()
                    && transform.last_changed().get() - system_change_tick.last_run().get() > 10
                {
                    log::info!(
                        "[apply_constraints_offset] Update Offset: {:?}",
                        transform.translation.truncate()
                    );
                    offset.0 = transform.translation.truncate();
                }
            }
            None => {
                let initial_offset = transform.translation.truncate();
                commands
                    .entity(entity)
                    .insert(ConstraintsOffset(initial_offset));
            }
        }
    }
}

pub fn apply_constraints(
    query: Query<(&Children, &SizeMixin), Changed<SizeMixin>>,
    mut child_query: Query<(
        &SizeMixin,
        &ConstraintsMixin,
        &mut Transform,
        &ConstraintsOffset,
    )>,
) {
    for (children, parent_size) in query.iter() {
        for &child in children.iter() {
            if let Ok((size_mixin, constraints_mixin, mut transform, offset)) =
                child_query.get_mut(child)
            {
                apply_horizontal_constraint(
                    &constraints_mixin.0,
                    &mut transform,
                    &parent_size.0,
                    &size_mixin.0,
                    offset.0.x,
                );
                apply_vertical_constraint(
                    &constraints_mixin.0,
                    &mut transform,
                    &parent_size.0,
                    &size_mixin.0,
                    offset.0.y,
                );
            }
        }
    }
}

fn apply_horizontal_constraint(
    constraints: &Constraints,
    transform: &mut Transform,
    parent_size: &Size,
    child_size: &Size,
    initial_offset_x: f32,
) {
    match constraints.horizontal {
        Constraint::Start => transform.translation.x = initial_offset_x,
        Constraint::Center => {
            transform.translation.x =
                (parent_size.width() - child_size.width()) / 2.0 + initial_offset_x
        }
        Constraint::End => {
            transform.translation.x = parent_size.width() - child_size.width() - initial_offset_x
        }
        // TODO
        Constraint::Stretch => {}
        Constraint::Scale => {
            let scale = parent_size.width() / child_size.width();
            transform.translation.x = initial_offset_x * scale;
        }
    }
}

fn apply_vertical_constraint(
    constraints: &Constraints,
    transform: &mut Transform,
    parent_size: &Size,
    child_size: &Size,
    initial_offset_y: f32,
) {
    match constraints.vertical {
        Constraint::Start => transform.translation.y = initial_offset_y,
        Constraint::Center => {
            transform.translation.y =
                (parent_size.height() - child_size.height()) / 2.0 + initial_offset_y
        }
        Constraint::End => {
            transform.translation.y = parent_size.height() - child_size.height() - initial_offset_y
        }
        // TODO
        Constraint::Stretch => {}
        Constraint::Scale => {
            let scale = parent_size.height() / child_size.height();
            transform.translation.y = initial_offset_y * scale;
        }
    }
}
