use crate::resources::tick::TickRes;
use bevy_ecs::change_detection::DetectChanges;
use bevy_ecs::query::With;
use bevy_ecs::system::Res;
use bevy_ecs::world::Ref;
use bevy_ecs::{
    entity::Entity,
    query::Changed,
    system::{Commands, Query},
};
use bevy_hierarchy::{Children, Parent};
use bevy_transform::components::Transform;
use dyn_comp_bundles::components::mixins::{
    Constraint, Constraints, ConstraintsLayoutMetricsMixin, ConstraintsMixin, SizeMixin,
};
use dyn_utils::properties::size::Size;

// TODO: Use taffy?
// Don't build any taffy tree by default, instead do it only for a Frame in Auto Layout,
// because for nodes that are children to a non Auto Layout parent it doesn't make sense
// https://github.com/bevyengine/bevy/blob/5caf085dacf74bf553a0428a5eb7f4574a9bb99c/crates/bevy_ui/src/layout/ui_surface.rs

pub fn apply_constraints_offset(
    mut commands: Commands,
    tick_res: Res<TickRes>,
    mut query: Query<(Entity, Ref<Transform>, &Parent), (Changed<Transform>, With<Parent>)>,
    size_mixin: Query<&SizeMixin>,
) {
    for (entity, transform, parent) in query.iter_mut() {
        log::info!(
            "[apply_constraints_offset] {:?}: {:?}",
            entity,
            if transform.last_changed().get() > tick_res.first_in_cycle.get() {
                (transform.last_changed().get() - tick_res.first_in_cycle.get()).to_string()
            } else {
                String::from("NaN")
            }
        );
        log::info!(
            "[apply_constraints_offset] {:?}: {}/{}",
            entity,
            tick_res.first_in_cycle.get(),
            transform.last_changed().get()
        );

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
        if transform.last_changed().get() > tick_res.first_in_cycle.get()
        // TODO: Remove once figured out where Transform is mutated before this system (although it wasn't explicitly changed)
                && transform.last_changed().get() - tick_res.first_in_cycle.get() > 10
        {
            if let Ok(SizeMixin(parent_size)) = size_mixin.get(parent.get()) {
                log::info!("[apply_constraints_offset] {:?}", entity);
                commands
                    .entity(entity)
                    .insert(ConstraintsLayoutMetricsMixin {
                        pos: transform.translation,
                        parent_size: *parent_size,
                    });
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
        &ConstraintsLayoutMetricsMixin,
    )>,
) {
    for (children, parent_size) in query.iter() {
        for &child in children.iter() {
            if let Ok((size_mixin, constraints_mixin, mut transform, layout_metric)) =
                child_query.get_mut(child)
            {
                log::info!(
                    "[apply_constraints] {:?}: {:?} | New Size: {:?}",
                    child,
                    layout_metric,
                    size_mixin
                );
                apply_horizontal_constraint(
                    &constraints_mixin.0,
                    &mut transform,
                    &parent_size.0,
                    &size_mixin.0,
                    layout_metric,
                );
                apply_vertical_constraint(
                    &constraints_mixin.0,
                    &mut transform,
                    &parent_size.0,
                    &size_mixin.0,
                    layout_metric,
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
    layout_metric: &ConstraintsLayoutMetricsMixin,
) {
    match constraints.horizontal {
        Constraint::Start => {
            transform.translation.x = layout_metric.pos.x;
        }
        Constraint::Center => {
            transform.translation.x =
                (parent_size.width() - child_size.width()) / 2.0 + layout_metric.pos.x;
        }
        Constraint::End => {
            transform.translation.x =
                layout_metric.pos.x + parent_size.width() - layout_metric.parent_size.width();
        }
        Constraint::Stretch => {
            // TODO
        }
        Constraint::Scale => {
            let scale = parent_size.width() / child_size.width();
            transform.translation.x = layout_metric.pos.x * scale;
        }
    }
}

fn apply_vertical_constraint(
    constraints: &Constraints,
    transform: &mut Transform,
    parent_size: &Size,
    child_size: &Size,
    layout_metric: &ConstraintsLayoutMetricsMixin,
) {
    match constraints.vertical {
        Constraint::Start => {
            transform.translation.y = layout_metric.pos.y;
        }
        Constraint::Center => {
            transform.translation.y =
                (parent_size.height() - child_size.height()) / 2.0 + layout_metric.pos.y;
        }
        Constraint::End => {
            transform.translation.y =
                parent_size.height() - layout_metric.parent_size.height() + layout_metric.pos.y;
        }
        Constraint::Stretch => {
            // TODO
        }
        Constraint::Scale => {
            let scale = parent_size.height() / child_size.height();
            transform.translation.y = layout_metric.pos.y * scale;
        }
    }
}
