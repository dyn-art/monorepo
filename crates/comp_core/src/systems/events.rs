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
    components::{
        marker::{Removed, Root},
        mixins::SizeMixin,
        nodes::CompNode,
    },
    events::{
        CompositionResizedInputEvent, CompositionViewportChangedInputEvent,
        EntityDeletedInputEvent, EntityMovedInputEvent, EntitySetPositionInputEvent,
        EntitySetRotationInputEvent, FocusRootNodesInputEvent,
    },
    properties::Viewport,
    utils::transform_to_z_rotation_rad,
};
use dyn_utils::{math::matrix::rotate_around_point, properties::size::Size, units::abs::Abs};
use glam::{Vec2, Vec3};

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

pub fn focus_root_nodes_input_system(
    mut event_reader: EventReader<FocusRootNodesInputEvent>,
    mut comp_res: ResMut<CompositionRes>,
    query: Query<(&SizeMixin, &Transform), (With<Root>, With<CompNode>)>,
) {
    if event_reader.read().len() > 0 {
        let CompositionRes {
            viewport:
                Viewport {
                    physical_size: original_physical_size,
                    ..
                },
            ..
        } = comp_res.as_ref();

        let mut min = Vec2::new(f32::INFINITY, f32::INFINITY);
        let mut max = Vec2::new(f32::NEG_INFINITY, f32::NEG_INFINITY);

        // Calculate bounding box
        for (SizeMixin(size), transform) in query.iter() {
            let corners = [
                transform.translation.truncate(), // Top left
                transform.translation.truncate() + Vec2::new(size.width(), 0.0), // Top right
                transform.translation.truncate() + Vec2::new(0.0, size.height()), // Bottom left
                transform.translation.truncate() + Vec2::new(size.width(), size.height()), // Bottom right
            ];

            for corner in corners.iter() {
                min.x = min.x.min(corner.x);
                max.x = max.x.max(corner.x);
                min.y = min.y.min(corner.y);
                max.y = max.y.max(corner.y);
            }
        }

        let bounding_size = max - min;

        let padding_factor = 1.05;
        let adjusted_bounding_size = bounding_size * padding_factor;

        let aspect_ratio = original_physical_size.width / original_physical_size.height;
        let focused_aspect_ratio = adjusted_bounding_size.x / adjusted_bounding_size.y;

        // Adjust the new physical_size to maintain the original aspect ratio
        let new_physical_size = if focused_aspect_ratio > aspect_ratio {
            Size::new(
                Abs::pt(adjusted_bounding_size.x),
                Abs::pt(adjusted_bounding_size.x / aspect_ratio),
            )
        } else {
            Size::new(
                Abs::pt(adjusted_bounding_size.y * aspect_ratio),
                Abs::pt(adjusted_bounding_size.y),
            )
        };

        // Calculate the new physica position
        let center = min + bounding_size / 2.0;
        let new_physical_position = center - new_physical_size.to_vec2() / 2.0;

        comp_res.viewport.physical_position = new_physical_position;
        comp_res.viewport.physical_size = new_physical_size;
    }
}
