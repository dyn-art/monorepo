use bevy_ecs::{
    event::EventReader,
    system::{Commands, Query},
};
use bevy_hierarchy::DespawnRecursiveExt;
use bevy_transform::components::Transform;
use dyn_comp_bundles::{
    components::mixins::SizeMixin,
    events::{
        EntityDeletedInputEvent, EntityMovedInputEvent, EntitySetPositionInputEvent,
        EntitySetRotationInputEvent,
    },
    math::transform_to_z_rotation_rad,
};
use dyn_utils::math::matrix::rotate_around_point;
use glam::Vec3;

// https://bevy-cheatbook.github.io/fundamentals/hierarchy.html#despawning-child-entities
// https://github.com/bevyengine/bevy/issues/5584
pub fn handle_entity_deleted_event(
    mut commands: Commands,
    mut event_reader: EventReader<EntityDeletedInputEvent>,
) {
    for event in event_reader.read() {
        commands.entity(event.entity).despawn_recursive();
    }
}

pub fn handle_entity_moved_event(
    mut event_reader: EventReader<EntityMovedInputEvent>,
    mut query: Query<&mut Transform>,
) {
    for EntityMovedInputEvent { entity, dx, dy } in event_reader.read() {
        if let Ok(mut transform) = query.get_mut(*entity) {
            transform.translation += Vec3::new(*dx, *dy, 0.0);
        }
    }
}

pub fn handle_entity_set_position_event(
    mut event_reader: EventReader<EntitySetPositionInputEvent>,
    mut query: Query<&mut Transform>,
) {
    for EntitySetPositionInputEvent { entity, x, y } in event_reader.read() {
        if let Ok(mut transform) = query.get_mut(*entity) {
            transform.translation.x = *x;
            transform.translation.y = *y;
        }
    }
}

pub fn handle_entity_set_rotation_event(
    mut event_reader: EventReader<EntitySetRotationInputEvent>,
    mut query: Query<(&mut Transform, &SizeMixin)>,
) {
    for EntitySetRotationInputEvent {
        entity,
        rotation_deg,
    } in event_reader.read()
    {
        if let Ok((mut transform, SizeMixin(size))) = query.get_mut(*entity) {
            let pivot_point = Vec3::new(size.width() / 2.0, size.height() / 2.0, 0.0);
            let reset_rotation_transform_mat4 = rotate_around_point(
                transform.compute_matrix(),
                -transform_to_z_rotation_rad(&transform),
                pivot_point,
            );
            let rotation_transform_mat4 = rotate_around_point(
                reset_rotation_transform_mat4,
                rotation_deg.to_rad(),
                pivot_point,
            );
            *transform = Transform::from_matrix(rotation_transform_mat4);
        }
    }
}
