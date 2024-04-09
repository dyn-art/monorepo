use crate::resources::composition::CompositionRes;
use bevy_ecs::{
    entity::Entity,
    event::EventReader,
    query::With,
    system::{Commands, Query, ResMut},
};
use bevy_hierarchy::{BuildChildren, Children, DespawnRecursiveExt};
use bevy_transform::components::Transform;
use dyn_comp_bundles::{
    components::{marker::Removed, mixins::SizeMixin},
    events::{
        CompositionResizedInputEvent, CompositionViewportChangedInputEvent,
        EntityDeletedInputEvent, EntityMovedInputEvent, EntitySetPositionInputEvent,
        EntitySetRotationInputEvent,
    },
    utils::transform_to_z_rotation_rad,
};
use dyn_utils::math::matrix::rotate_around_point;
use glam::Vec3;

pub fn composition_resized_input_system(
    mut comp_res: ResMut<CompositionRes>,
    mut event_reader: EventReader<CompositionResizedInputEvent>,
) {
    if let Some(event) = event_reader.read().last() {
        comp_res.size = event.size;
        comp_res.viewport.physical_size = event.size;
    }
}

pub fn composition_viewport_input_system(
    mut comp_res: ResMut<CompositionRes>,
    mut event_reader: EventReader<CompositionViewportChangedInputEvent>,
) {
    if let Some(event) = event_reader.read().last() {
        comp_res.viewport = event.viewport;
    }
}

// https://bevy-cheatbook.github.io/fundamentals/hierarchy.html#despawning-child-entities
// https://github.com/bevyengine/bevy/issues/5584
pub fn entity_deleted_input_system(
    mut commands: Commands,
    mut event_reader: EventReader<EntityDeletedInputEvent>,
    children_query: Query<&Children>,
) {
    for event in event_reader.read() {
        commands
            .entity(event.entity)
            .insert(Removed)
            .remove_parent();

        if let Ok(children) = children_query.get(event.entity) {
            for child in children.iter() {
                commands.entity(*child).insert(Removed);
            }
        }
    }
}

pub fn despawn_removed_entities_system(
    mut commands: Commands,
    query: Query<Entity, With<Removed>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn entity_moved_input_system(
    mut event_reader: EventReader<EntityMovedInputEvent>,
    mut query: Query<&mut Transform>,
) {
    for EntityMovedInputEvent { entity, dx, dy } in event_reader.read() {
        if let Ok(mut transform) = query.get_mut(*entity) {
            transform.translation += Vec3::new(*dx, *dy, 0.0);
        }
    }
}

pub fn entity_set_position_input_system(
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

pub fn entity_set_rotation_input_system(
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
