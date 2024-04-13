use crate::resources::tick::TickRes;
use bevy_ecs::change_detection::DetectChanges;
use bevy_ecs::query::With;
use bevy_ecs::system::{ParamSet, Res};
use bevy_ecs::world::{Mut, Ref};
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
use dyn_utils::units::abs::Abs;

// TODO: Use taffy?
// Don't build any taffy tree by default, instead do it only for a Frame in Auto Layout,
// because for nodes that are children to a non Auto Layout parent it doesn't make sense
// https://github.com/bevyengine/bevy/blob/5caf085dacf74bf553a0428a5eb7f4574a9bb99c/crates/bevy_ui/src/layout/ui_surface.rs

pub fn apply_constraints_offset(
    mut commands: Commands,
    tick_res: Res<TickRes>,
    mut query: Query<
        (Entity, Ref<Transform>, Ref<SizeMixin>, &Parent),
        (Changed<Transform>, With<Parent>),
    >,
    size_mixin_query: Query<&SizeMixin>,
) {
    for (entity, transform, size_mixin, parent) in query.iter_mut() {
        // Check if Transform has changed in this update cycle or the last.
        // A change in the current cycle likely indicates a mutation from operations like Translation.
        // A change in the last cycle suggests an update by a Constraint system,
        // whose changes should be ignored by this system.
        //
        // https://discord.com/channels/691052431525675048/1228316069207216130
        if transform.last_changed().get() > tick_res.first_in_cycle.get() || size_mixin.last_changed().get() > tick_res.first_in_cycle.get()
        // If its the first update cycle
            || tick_res.first_in_cycle.get() < 100
        {
            if let Ok(SizeMixin(parent_size)) = size_mixin_query.get(parent.get()) {
                commands
                    .entity(entity)
                    .insert(ConstraintsLayoutMetricsMixin {
                        pos: transform.translation,
                        size: size_mixin.0,
                        parent_size: *parent_size,
                    });
            }
        }
    }
}

pub fn apply_constraints(
    mut query_set: ParamSet<(
        Query<(&Children, &SizeMixin), Changed<SizeMixin>>,
        Query<(
            &mut SizeMixin,
            &ConstraintsMixin,
            &mut Transform,
            &ConstraintsLayoutMetricsMixin,
        )>,
    )>,
) {
    let mut to_update_children = Vec::new();

    for (children, parent_size) in query_set.p0().iter() {
        for child in children.iter() {
            to_update_children.push((*child, parent_size.0))
        }
    }

    for (child, parent_size) in to_update_children.iter() {
        if let Ok((mut size_mixin, constraints_mixin, mut transform, layout_metric)) =
            query_set.p1().get_mut(*child)
        {
            apply_horizontal_constraint(
                &constraints_mixin.0,
                &mut transform,
                &mut size_mixin,
                &parent_size,
                layout_metric,
            );
            apply_vertical_constraint(
                &constraints_mixin.0,
                &mut transform,
                &mut size_mixin,
                &parent_size,
                layout_metric,
            );
        }
    }
}

fn apply_horizontal_constraint(
    constraints: &Constraints,
    child_transform: &mut Mut<Transform>,
    child_size_mixin: &mut Mut<SizeMixin>,
    parent_size: &Size,
    layout_metric: &ConstraintsLayoutMetricsMixin,
) {
    match constraints.horizontal {
        Constraint::Start => {
            child_transform.translation.x = layout_metric.pos.x;
        }
        Constraint::Center => {
            let center_offset_x = (layout_metric.parent_size.width() - layout_metric.size.width())
                / 2.0
                - layout_metric.pos.x;
            child_transform.translation.x =
                layout_metric.pos.x + parent_size.width() - layout_metric.parent_size.width();
            let current_center_x = (parent_size.width() - child_size_mixin.0.width()) / 2.0;
            child_transform.translation.x = current_center_x - center_offset_x;
        }
        Constraint::End => {
            child_transform.translation.x =
                layout_metric.pos.x + parent_size.width() - layout_metric.parent_size.width();
        }
        Constraint::Stretch => {
            // TODO
        }
        Constraint::Scale => {
            let left = layout_metric.pos.x;
            let right =
                layout_metric.pos.x + parent_size.width() - layout_metric.parent_size.width();
            child_transform.translation.x = left;
            child_size_mixin.0.width = layout_metric.size.width + Abs::pt(right - left);
        }
    }
}

fn apply_vertical_constraint(
    constraints: &Constraints,
    child_transform: &mut Mut<Transform>,
    child_size_mixin: &mut Mut<SizeMixin>,
    parent_size: &Size,
    layout_metric: &ConstraintsLayoutMetricsMixin,
) {
    match constraints.vertical {
        Constraint::Start => {
            child_transform.translation.y = layout_metric.pos.y;
        }
        Constraint::Center => {
            let center_offset_y =
                (layout_metric.parent_size.height() - layout_metric.size.height()) / 2.0
                    - layout_metric.pos.y;
            child_transform.translation.y =
                layout_metric.pos.y + parent_size.height() - layout_metric.parent_size.height();
            let current_center_y = (parent_size.height() - child_size_mixin.0.height()) / 2.0;
            child_transform.translation.y = current_center_y - center_offset_y;
        }
        Constraint::End => {
            child_transform.translation.y =
                parent_size.height() - layout_metric.parent_size.height() + layout_metric.pos.y;
        }
        Constraint::Stretch => {
            // TODO
        }
        Constraint::Scale => {
            let top = layout_metric.pos.y;
            let bottom =
                layout_metric.pos.y + parent_size.height() - layout_metric.parent_size.height();
            child_transform.translation.y = top;
            child_size_mixin.0.height = layout_metric.size.height + Abs::pt(bottom - top);
        }
    }
}
