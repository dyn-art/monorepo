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
    mut query: Query<(Entity, &Transform, Option<&mut ConstraintsOffset>), Changed<Transform>>,
) {
    for (entity, transform, maybe_offset) in query.iter_mut() {
        log::info!(
            "[apply_constraints_offset] {:?} -> {:?}/{:?} | {:?}",
            entity,
            transform.translation.truncate(),
            maybe_offset,
            transform
        );
        match maybe_offset {
            Some(mut offset) => {
                // offset.0 = transform.translation.truncate();
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
