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
        mixins::{
            BlendModeMixin, CornerRadiiMixin, ImageAssetMixin, OpacityMixin, SizeMixin,
            VisibilityMixin,
        },
        nodes::{
            CompNode, EllipseCompNode, FrameCompNode, PolygonCompNode, StarCompNode, TextCompNode,
        },
        paints::{GradientCompPaint, ImageCompPaint, SolidCompPaint},
    },
    events::{
        DeleteEntityInputEvent, FocusRootNodesInputEvent, MoveEntityInputEvent,
        UpdateCompositionSizeInputEvent, UpdateCompositionViewportInputEvent,
        UpdateEllipseNodeInputEvent, UpdateEntityBlendModeInputEvent,
        UpdateEntityCornerRadiiInputEvent, UpdateEntityOpacityInputEvent,
        UpdateEntityRotationInputEvent, UpdateEntitySizeInputEvent,
        UpdateEntityTransformInputEvent, UpdateEntityVisibilityInputEvent,
        UpdateFrameNodeInputEvent, UpdateGradientPaintInputEvent, UpdateImagePaintInputEvent,
        UpdatePolygonNodeInputEvent, UpdateSolidPaintInputEvent, UpdateStarNodeInputEvent,
        UpdateTextNodeInputEvent,
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
// Node
// =============================================================================

pub fn update_frame_node_input_system(
    mut event_reader: EventReader<UpdateFrameNodeInputEvent>,
    mut query: Query<&mut FrameCompNode>,
) {
    for UpdateFrameNodeInputEvent {
        entity,
        clip_content: maybe_clip_content,
    } in event_reader.read()
    {
        if let Ok(mut frame_comp_node) = query.get_mut(*entity) {
            if let Some(clip_content) = maybe_clip_content {
                frame_comp_node.clip_content = *clip_content;
            }
        }
    }
}

pub fn update_ellipse_node_input_system(
    mut event_reader: EventReader<UpdateEllipseNodeInputEvent>,
    mut query: Query<&mut EllipseCompNode>,
) {
    for UpdateEllipseNodeInputEvent {
        entity,
        starting_angle: maybe_starting_angle,
        ending_angle: maybe_ending_angle,
        inner_radius_ratio: maybe_inner_radius_ratio,
    } in event_reader.read()
    {
        if let Ok(mut ellipse_comp_node) = query.get_mut(*entity) {
            if let Some(starting_angle) = maybe_starting_angle {
                ellipse_comp_node.arc_data.starting_angle = *starting_angle;
            }
            if let Some(ending_angle) = maybe_ending_angle {
                ellipse_comp_node.arc_data.ending_angle = *ending_angle;
            }
            if let Some(inner_radius_ratio) = maybe_inner_radius_ratio {
                ellipse_comp_node.arc_data.inner_radius_ratio = *inner_radius_ratio;
            }
        }
    }
}

pub fn update_star_node_input_system(
    mut event_reader: EventReader<UpdateStarNodeInputEvent>,
    mut query: Query<&mut StarCompNode>,
) {
    for UpdateStarNodeInputEvent {
        entity,
        point_count: maybe_point_count,
        inner_radius_ratio: maybe_inner_radius_ratio,
    } in event_reader.read()
    {
        if let Ok(mut star_comp_node) = query.get_mut(*entity) {
            if let Some(point_count) = maybe_point_count {
                star_comp_node.point_count = *point_count;
            }
            if let Some(inner_radius_ratio) = maybe_inner_radius_ratio {
                star_comp_node.inner_radius_ratio = *inner_radius_ratio;
            }
        }
    }
}

pub fn update_polygon_node_input_system(
    mut event_reader: EventReader<UpdatePolygonNodeInputEvent>,
    mut query: Query<&mut PolygonCompNode>,
) {
    for UpdatePolygonNodeInputEvent {
        entity,
        point_count: maybe_point_count,
    } in event_reader.read()
    {
        if let Ok(mut polygon_comp_node) = query.get_mut(*entity) {
            if let Some(point_count) = maybe_point_count {
                polygon_comp_node.point_count = *point_count;
            }
        }
    }
}

pub fn update_text_node_input_system(
    mut event_reader: EventReader<UpdateTextNodeInputEvent>,
    mut query: Query<&mut TextCompNode>,
) {
    for UpdateTextNodeInputEvent {
        entity,
        text: maybe_text,
        attributes: maybe_attributes,
        line_wrap: maybe_line_wrap,
        horizontal_text_alignment: maybe_horizontal_text_alignment,
        vertical_text_alignment: maybe_vertical_text_alignment,
        sizing_mode: maybe_sizing_mode,
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
            if let Some(sizing_mode) = maybe_sizing_mode {
                text_comp_node.sizing_mode = *sizing_mode;
            }
        }
    }
}

// =============================================================================
// Paint
// =============================================================================

pub fn update_solid_paint_input_system(
    mut event_reader: EventReader<UpdateSolidPaintInputEvent>,
    mut query: Query<&mut SolidCompPaint>,
) {
    for UpdateSolidPaintInputEvent { entity, color } in event_reader.read() {
        if let Ok(mut solid_comp_paint) = query.get_mut(*entity) {
            solid_comp_paint.color = *color;
        }
    }
}

pub fn update_image_paint_input_system(
    mut event_reader: EventReader<UpdateImagePaintInputEvent>,
    mut query: Query<(&mut ImageCompPaint, &mut ImageAssetMixin)>,
) {
    for UpdateImagePaintInputEvent {
        entity,
        scale_mode: maybe_scale_mode,
        image_id: maybe_image_id,
    } in event_reader.read()
    {
        if let Ok((mut image_comp_paint, mut image_asset_mixin)) = query.get_mut(*entity) {
            if let Some(scale_mode) = maybe_scale_mode {
                image_comp_paint.scale_mode = *scale_mode;
            }
            if let Some(image_id) = maybe_image_id {
                image_asset_mixin.0 = Some(*image_id);
            }
        }
    }
}

pub fn update_gradient_paint_input_system(
    mut event_reader: EventReader<UpdateGradientPaintInputEvent>,
    mut query: Query<&mut GradientCompPaint>,
) {
    for UpdateGradientPaintInputEvent {
        entity,
        variant: maybe_variant,
        stops: maybe_stops,
    } in event_reader.read()
    {
        if let Ok(mut gradient_comp_paint) = query.get_mut(*entity) {
            if let Some(variant) = maybe_variant {
                gradient_comp_paint.variant = *variant;
            }
            if let Some(stops) = maybe_stops {
                gradient_comp_paint.stops = SmallVec::from_vec(stops.clone());
            }
        }
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

pub fn update_entity_transform_input_system(
    mut event_reader: EventReader<UpdateEntityTransformInputEvent>,
    mut query: Query<&mut Transform>,
) {
    for UpdateEntityTransformInputEvent {
        entity,
        x: maybe_x,
        y: maybe_y,
        rotation_deg: maybe_rotation_deg,
    } in event_reader.read()
    {
        if let Ok(mut transform) = query.get_mut(*entity) {
            if let Some(x) = maybe_x {
                transform.translation.x = *x;
            }
            if let Some(y) = maybe_y {
                transform.translation.y = *y;
            }
            if let Some(rotation_deg) = maybe_rotation_deg {
                transform.rotation = rotation_deg.to_quat();
            }
        }
    }
}

pub fn update_entity_size_input_system(
    mut event_reader: EventReader<UpdateEntitySizeInputEvent>,
    mut query: Query<&mut SizeMixin>,
) {
    for UpdateEntitySizeInputEvent { entity, size } in event_reader.read() {
        if let Ok(mut size_mixin) = query.get_mut(*entity) {
            size_mixin.0 = *size;
        }
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

pub fn update_entity_corner_radii_input_system(
    mut event_reader: EventReader<UpdateEntityCornerRadiiInputEvent>,
    mut query: Query<&mut CornerRadiiMixin>,
) {
    for UpdateEntityCornerRadiiInputEvent {
        entity,
        corner_radii,
    } in event_reader.read()
    {
        if let Ok(mut corner_radii_mixin) = query.get_mut(*entity) {
            corner_radii_mixin.0 = *corner_radii;
        }
    }
}

pub fn update_entity_blend_mode_input_system(
    mut event_reader: EventReader<UpdateEntityBlendModeInputEvent>,
    mut query: Query<&mut BlendModeMixin>,
) {
    for UpdateEntityBlendModeInputEvent { entity, blend_mode } in event_reader.read() {
        if let Ok(mut blend_mode_mixin) = query.get_mut(*entity) {
            blend_mode_mixin.0 = *blend_mode;
        }
    }
}

pub fn update_entity_opacity_input_system(
    mut event_reader: EventReader<UpdateEntityOpacityInputEvent>,
    mut query: Query<&mut OpacityMixin>,
) {
    for UpdateEntityOpacityInputEvent { entity, opacity } in event_reader.read() {
        if let Ok(mut opacity_mixin) = query.get_mut(*entity) {
            opacity_mixin.0 = *opacity;
        }
    }
}
