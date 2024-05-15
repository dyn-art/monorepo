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
        mixins::{SizeMixin, VisibilityMixin},
        nodes::{CompNode, TextCompNode},
    },
    events::{
        DeleteEntityInputEvent, FocusRootNodesInputEvent, MoveEntityInputEvent,
        UpdateCompositionSizeInputEvent, UpdateCompositionViewportInputEvent,
        UpdateEntityPositionInputEvent, UpdateEntityRotationInputEvent, UpdateEntityTextInputEvent,
        UpdateEntityVisibilityInputEvent,
    },
    properties::Viewport,
    utils::transform_to_z_rotation_rad,
};
use dyn_utils::{math::matrix::rotate_around_point, properties::size::Size, units::abs::Abs};
use glam::{Vec2, Vec3};
use smallvec::SmallVec;

// =============================================================================
// Composition
// =============================================================================

pub fn update_composition_size_input_system(
    mut comp_res: ResMut<CompositionRes>,
    mut event_reader: EventReader<UpdateCompositionSizeInputEvent>,
) {
    if let Some(event) = event_reader.read().last() {
        comp_res.size = event.size;
        comp_res.viewport.physical_size = event.size;
    }
}

pub fn update_composition_viewport_input_system(
    mut comp_res: ResMut<CompositionRes>,
    mut event_reader: EventReader<UpdateCompositionViewportInputEvent>,
) {
    if let Some(event) = event_reader.read().last() {
        comp_res.viewport = event.viewport;
    }
}

// =============================================================================
// Noe
// =============================================================================

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

        let padding_factor = 1.0;
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

// =============================================================================
// Entity
// =============================================================================

// https://bevy-cheatbook.github.io/fundamentals/hierarchy.html#despawning-child-entities
// https://github.com/bevyengine/bevy/issues/5584
pub fn delete_entity_input_system(
    mut commands: Commands,
    mut event_reader: EventReader<DeleteEntityInputEvent>,
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

pub fn move_entity_input_system(
    mut event_reader: EventReader<MoveEntityInputEvent>,
    mut query: Query<&mut Transform>,
) {
    for MoveEntityInputEvent {
        entity,
        dx: maybe_dx,
        dy: maybe_dy,
    } in event_reader.read()
    {
        if let Ok(mut transform) = query.get_mut(*entity) {
            transform.translation +=
                Vec3::new(maybe_dx.unwrap_or(0.0), maybe_dy.unwrap_or(0.0), 0.0);
        }
    }
}

pub fn update_entity_position_input_system(
    mut event_reader: EventReader<UpdateEntityPositionInputEvent>,
    mut query: Query<&mut Transform>,
) {
    for UpdateEntityPositionInputEvent {
        entity,
        x: maybe_x,
        y: maybe_y,
    } in event_reader.read()
    {
        if let Ok(mut transform) = query.get_mut(*entity) {
            if let Some(x) = maybe_x {
                transform.translation.x = *x;
            }
            if let Some(y) = maybe_y {
                transform.translation.y = *y;
            }
        }
    }
}

pub fn update_entity_rotation_input_system(
    mut event_reader: EventReader<UpdateEntityRotationInputEvent>,
    mut query: Query<(&mut Transform, &SizeMixin)>,
) {
    for UpdateEntityRotationInputEvent {
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

pub fn update_entity_text_input_system(
    mut event_reader: EventReader<UpdateEntityTextInputEvent>,
    mut query: Query<&mut TextCompNode>,
) {
    for UpdateEntityTextInputEvent {
        entity,
        text: maybe_text,
        attributes: maybe_attributes,
        line_wrap: maybe_line_wrap,
        horizontal_text_alignment: maybe_horizontal_text_alignment,
        vertical_text_alignment: maybe_vertical_text_alignment,
    } in event_reader.read()
    {
        if let Ok(mut text_comp_node) = query.get_mut(*entity) {
            if let Some(text) = maybe_text {
                text_comp_node.text = text.clone();
            }
            if let Some(attributes) = maybe_attributes {
                text_comp_node.attributes = SmallVec::from_vec(attributes.clone());
            }
            if let Some(line_wrap) = maybe_line_wrap {
                text_comp_node.line_wrap = *line_wrap;
            }
            if let Some(horizontal_text_alignment) = maybe_horizontal_text_alignment {
                text_comp_node.horizontal_text_alignment = *horizontal_text_alignment;
            }
            if let Some(vertical_text_alignment) = maybe_vertical_text_alignment {
                text_comp_node.vertical_text_alignment = *vertical_text_alignment;
            }
        }
    }
}

pub fn update_entity_visibility_input_system(
    mut event_reader: EventReader<UpdateEntityVisibilityInputEvent>,
    mut query: Query<&mut VisibilityMixin>,
) {
    for UpdateEntityVisibilityInputEvent { entity, visible } in event_reader.read() {
        if let Ok(mut visibility_mixin) = query.get_mut(*entity) {
            visibility_mixin.0 = *visible;
        }
    }
}
