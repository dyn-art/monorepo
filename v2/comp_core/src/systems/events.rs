use bevy_ecs::{
    event::EventReader,
    system::{Commands, Query},
};
use bevy_hierarchy::DespawnRecursiveExt;
use bevy_transform::components::Transform;
use dyn_comp_common::{
    events::{
        EntityDeletedInputEvent, EntityMovedInputEvent, EntitySetPositionInputEvent,
        EntitySetRotationInputEvent,
    },
    mixins::SizeMixin,
};
use glam::{EulerRot, Mat4, Quat, Vec3};

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
                -transform.rotation.to_euler(EulerRot::XYZ).2,
                pivot_point,
            );
            let rotation_transform_mat4 = rotate_around_point(
                reset_rotation_transform_mat4,
                rotation_deg.to_radians(),
                pivot_point,
            );
            *transform = Transform::from_matrix(rotation_transform_mat4);
        }
    }
}

// https://math.stackexchange.com/questions/2093314/rotation-matrix-of-rotation-around-a-point-other-than-the-origin
pub fn rotate_around_point(transform: Mat4, angle_rad: f32, pivot_point: Vec3) -> Mat4 {
    let translate_to_pivot = Mat4::from_translation(pivot_point);
    let translate_to_origin = Mat4::from_translation(-pivot_point);
    let rotation = Mat4::from_quat(Quat::from_rotation_z(angle_rad));

    // b: -mat4.x_axis.y,
    // c: -mat4.y_axis.x,
    let rotation_around_pivot = translate_to_pivot * rotation * translate_to_origin;

    return transform * rotation_around_pivot;
}
