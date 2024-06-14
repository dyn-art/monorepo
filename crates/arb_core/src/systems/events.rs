use crate::resources::{canvas::ArtboardRes, referencer::ReferencerRes};
use bevy_ecs::{
    entity::Entity,
    event::{EventReader, EventWriter},
    query::With,
    system::{Commands, Query, Res, ResMut, SystemState},
    world::World,
};
use bevy_hierarchy::{BuildChildren, Children};
use bevy_transform::components::Transform;
use dyn_arb_asset::resources::AssetsRes;
use dyn_arb_bundles::{
    components::{
        marker::{Removed, Root},
        mixins::{
            BlendModeMixin, CornerRadiiMixin, ImageAssetMixin, OpacityMixin, PaintChildMixin,
            PaintParentMixin, SizeMixin, StyleChildrenMixin, StyleParentMixin, VisibilityMixin,
        },
        nodes::{
            ArbNode, EllipseArbNode, FrameArbNode, PolygonArbNode, StarArbNode, TextArbNode,
        },
        paints::{ArbPaint, GradientArbPaint, ImageArbPaint, SolidArbPaint},
        styles::{DropShadowArbStyle, FillArbStyle, StrokeArbStyle},
    },
    events::{
        CreateAssetInputEvent, CreateNodeInputEvent, CreatePaintInputEvent, DeleteEntityInputEvent,
        FocusRootNodesInputEvent, MoveEntityInputEvent, UpdateArtboardSizeInputEvent,
        UpdateArtboardViewportInputEvent, UpdateDropShadowStyleInputEvent,
        UpdateEllipseNodeInputEvent, UpdateEntityBlendModeInputEvent,
        UpdateEntityChildrenInputEvent, UpdateEntityCornerRadiiInputEvent,
        UpdateEntityOpacityInputEvent, UpdateEntityRotationInputEvent, UpdateEntitySizeInputEvent,
        UpdateEntityTransformInputEvent, UpdateEntityVisibilityInputEvent,
        UpdateFillStyleInputEvent, UpdateFrameNodeInputEvent, UpdateGradientPaintInputEvent,
        UpdateImagePaintInputEvent, UpdatePolygonNodeInputEvent, UpdateSolidPaintInputEvent,
        UpdateStarNodeInputEvent, UpdateStorkeStyleInputEvent, UpdateTextNodeInputEvent,
    },
    properties::Viewport,
    reference_id::ReferenceIdOrEntity,
    utils::transform_to_z_rotation_rad,
    Node, Paint, Style,
};
use dyn_utils::{math::matrix::rotate_around_point, properties::size::Size, units::abs::Abs};
use glam::{Vec2, Vec3};
use smallvec::SmallVec;

// =============================================================================
// Artboard
// =============================================================================

pub fn update_canvas_size_input_system(
    mut arb_res: ResMut<ArtboardRes>,
    mut event_reader: EventReader<UpdateArtboardSizeInputEvent>,
) {
    if let Some(event) = event_reader.read().last() {
        arb_res.size = event.size;
        arb_res.viewport.physical_size = event.size;
    }
}

pub fn update_canvas_viewport_input_system(
    mut arb_res: ResMut<ArtboardRes>,
    mut event_reader: EventReader<UpdateArtboardViewportInputEvent>,
) {
    if let Some(event) = event_reader.read().last() {
        arb_res.viewport = event.viewport;
    }
}

pub fn focus_root_nodes_input_system(
    mut event_reader: EventReader<FocusRootNodesInputEvent>,
    mut arb_res: ResMut<ArtboardRes>,
    query: Query<(&SizeMixin, &Transform), (With<Root>, With<ArbNode>)>,
) {
    if event_reader.read().len() > 0 {
        let ArtboardRes {
            viewport:
                Viewport {
                    physical_size: original_physical_size,
                    ..
                },
            ..
        } = arb_res.as_ref();

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

        arb_res.viewport.physical_position = new_physical_position;
        arb_res.viewport.physical_size = new_physical_size;
    }
}

// =============================================================================
// Node
// =============================================================================

// TODO: Improve
pub fn create_node_input_system(
    mut commands: Commands,
    mut referencer_res: ResMut<ReferencerRes>,
    mut update_children_event_writer: EventWriter<UpdateEntityChildrenInputEvent>,
    mut event_reader: EventReader<CreateNodeInputEvent>,
    mut paint_parent_query: Query<&mut PaintParentMixin>,
) {
    for CreateNodeInputEvent { node } in event_reader.read() {
        // Spawn node
        let node_entity_commands = match node {
            Node::Frame(n) => n.spawn(&mut commands),
            Node::Rectangle(n) => n.spawn(&mut commands),
            Node::Ellipse(n) => n.spawn(&mut commands),
            Node::Star(n) => n.spawn(&mut commands),
            Node::Polygon(n) => n.spawn(&mut commands),
            Node::Text(n) => n.spawn(&mut commands),
            Node::Vector(n) => n.spawn(&mut commands),
        };
        let node_entity = node_entity_commands.id();

        let maybe_children = match node {
            Node::Frame(node) => Some(&node.children),
            _ => None,
        };

        // Handle children
        if let Some(children) = maybe_children {
            update_children_event_writer.send(UpdateEntityChildrenInputEvent {
                id: ReferenceIdOrEntity::Entity {
                    entity: node_entity,
                },
                children: children.clone(),
            });
        }

        let maybe_styles = match node {
            Node::Frame(n) => Some(&n.styles),
            Node::Rectangle(n) => Some(&n.styles),
            Node::Ellipse(n) => Some(&n.styles),
            Node::Star(n) => Some(&n.styles),
            Node::Polygon(n) => Some(&n.styles),
            Node::Text(n) => Some(&n.styles),
            Node::Vector(n) => Some(&n.styles),
            _ => None,
        };

        // Handle styles
        let mut style_children_mixin = StyleChildrenMixin(SmallVec::new());
        if let Some(styles) = maybe_styles {
            for style in styles {
                let maybe_paint_entity = match style {
                    Style::Fill(s) => s
                        .paint_id
                        .get_entity(referencer_res.get_reference_id_to_entity_map()),
                    Style::Stroke(s) => s
                        .paint_id
                        .get_entity(referencer_res.get_reference_id_to_entity_map()),
                    _ => None,
                };

                // Spawn style
                let mut style_entity_commands = match style {
                    Style::Fill(s) => s.spawn(&mut commands),
                    Style::Stroke(s) => s.spawn(&mut commands),
                    Style::DropShadow(s) => s.spawn(&mut commands),
                };
                let style_entity = style_entity_commands.id();

                // Establish parent child relation between node and style (1)
                style_entity_commands.insert(StyleParentMixin(node_entity));
                style_children_mixin.0.push(style_entity);

                // Establish parent child relation between style and paint
                if let Some(paint_entity) = maybe_paint_entity {
                    if let Ok(mut paint_parent_mixin) = paint_parent_query.get_mut(paint_entity) {
                        paint_parent_mixin.0.push(style_entity);
                        style_entity_commands.insert(PaintChildMixin(paint_entity));
                    }
                }

                // Reference style entity
                let maybe_style_id = match style {
                    Style::DropShadow(s) => s.id.clone(),
                    Style::Fill(s) => s.id.clone(),
                    Style::Stroke(s) => s.id.clone(),
                };
                if let Some(style_id) = maybe_style_id {
                    referencer_res.reference_entity(style_id, style_entity);
                }
            }
        }

        // Establish parent child relation between node and style (2)
        commands.entity(node_entity).insert(style_children_mixin);

        // Reference node entity
        let maybe_node_id = match node {
            Node::Frame(n) => n.id.clone(),
            Node::Rectangle(n) => n.id.clone(),
            Node::Ellipse(n) => n.id.clone(),
            Node::Star(n) => n.id.clone(),
            Node::Polygon(n) => n.id.clone(),
            Node::Text(n) => n.id.clone(),
            Node::Vector(n) => n.id.clone(),
        };
        if let Some(node_id) = maybe_node_id {
            referencer_res.reference_entity(node_id, node_entity);
        }
    }
}

pub fn update_frame_node_input_system(
    referencer_res: Res<ReferencerRes>,
    mut event_reader: EventReader<UpdateFrameNodeInputEvent>,
    mut query: Query<&mut FrameArbNode>,
) {
    for UpdateFrameNodeInputEvent {
        id,
        clip_content: maybe_clip_content,
    } in event_reader.read()
    {
        if let Some(entity) = id.get_entity(referencer_res.get_reference_id_to_entity_map()) {
            if let Ok(mut frame_arb_node) = query.get_mut(entity) {
                if let Some(clip_content) = maybe_clip_content {
                    frame_arb_node.clip_content = *clip_content;
                }
            }
        }
    }
}

pub fn update_ellipse_node_input_system(
    referencer_res: Res<ReferencerRes>,
    mut event_reader: EventReader<UpdateEllipseNodeInputEvent>,
    mut query: Query<&mut EllipseArbNode>,
) {
    for UpdateEllipseNodeInputEvent {
        id,
        starting_angle: maybe_starting_angle,
        ending_angle: maybe_ending_angle,
        inner_radius_ratio: maybe_inner_radius_ratio,
    } in event_reader.read()
    {
        if let Some(entity) = id.get_entity(referencer_res.get_reference_id_to_entity_map()) {
            if let Ok(mut ellipse_arb_node) = query.get_mut(entity) {
                if let Some(starting_angle) = maybe_starting_angle {
                    ellipse_arb_node.arc_data.starting_angle = *starting_angle;
                }
                if let Some(ending_angle) = maybe_ending_angle {
                    ellipse_arb_node.arc_data.ending_angle = *ending_angle;
                }
                if let Some(inner_radius_ratio) = maybe_inner_radius_ratio {
                    ellipse_arb_node.arc_data.inner_radius_ratio = *inner_radius_ratio;
                }
            }
        }
    }
}

pub fn update_star_node_input_system(
    referencer_res: Res<ReferencerRes>,
    mut event_reader: EventReader<UpdateStarNodeInputEvent>,
    mut query: Query<&mut StarArbNode>,
) {
    for UpdateStarNodeInputEvent {
        id,
        point_count: maybe_point_count,
        inner_radius_ratio: maybe_inner_radius_ratio,
    } in event_reader.read()
    {
        if let Some(entity) = id.get_entity(referencer_res.get_reference_id_to_entity_map()) {
            if let Ok(mut star_arb_node) = query.get_mut(entity) {
                if let Some(point_count) = maybe_point_count {
                    star_arb_node.point_count = *point_count;
                }
                if let Some(inner_radius_ratio) = maybe_inner_radius_ratio {
                    star_arb_node.inner_radius_ratio = *inner_radius_ratio;
                }
            }
        }
    }
}

pub fn update_polygon_node_input_system(
    referencer_res: Res<ReferencerRes>,
    mut event_reader: EventReader<UpdatePolygonNodeInputEvent>,
    mut query: Query<&mut PolygonArbNode>,
) {
    for UpdatePolygonNodeInputEvent {
        id,
        point_count: maybe_point_count,
    } in event_reader.read()
    {
        if let Some(entity) = id.get_entity(referencer_res.get_reference_id_to_entity_map()) {
            if let Ok(mut polygon_arb_node) = query.get_mut(entity) {
                if let Some(point_count) = maybe_point_count {
                    polygon_arb_node.point_count = *point_count;
                }
            }
        }
    }
}

pub fn update_text_node_input_system(
    referencer_res: Res<ReferencerRes>,
    mut event_reader: EventReader<UpdateTextNodeInputEvent>,
    mut query: Query<&mut TextArbNode>,
) {
    for UpdateTextNodeInputEvent {
        id,
        text: maybe_text,
        attributes: maybe_attributes,
        line_wrap: maybe_line_wrap,
        horizontal_text_alignment: maybe_horizontal_text_alignment,
        vertical_text_alignment: maybe_vertical_text_alignment,
        sizing_mode: maybe_sizing_mode,
    } in event_reader.read()
    {
        if let Some(entity) = id.get_entity(referencer_res.get_reference_id_to_entity_map()) {
            if let Ok(mut text_arb_node) = query.get_mut(entity) {
                if let Some(text) = maybe_text {
                    text_arb_node.text = text.clone();
                }
                if let Some(attributes) = maybe_attributes {
                    text_arb_node.attributes = SmallVec::from_vec(attributes.clone());
                }
                if let Some(line_wrap) = maybe_line_wrap {
                    text_arb_node.line_wrap = *line_wrap;
                }
                if let Some(horizontal_text_alignment) = maybe_horizontal_text_alignment {
                    text_arb_node.horizontal_text_alignment = *horizontal_text_alignment;
                }
                if let Some(vertical_text_alignment) = maybe_vertical_text_alignment {
                    text_arb_node.vertical_text_alignment = *vertical_text_alignment;
                }
                if let Some(sizing_mode) = maybe_sizing_mode {
                    text_arb_node.sizing_mode = *sizing_mode;
                }
            }
        }
    }
}

// =============================================================================
// Style
// =============================================================================

pub fn update_fill_style_input_system(
    referencer_res: Res<ReferencerRes>,
    mut event_reader: EventReader<UpdateFillStyleInputEvent>,
    mut style_query: Query<&mut PaintChildMixin, With<FillArbStyle>>,
    mut paint_query: Query<&mut PaintParentMixin, With<ArbPaint>>,
) {
    for UpdateFillStyleInputEvent {
        id,
        paint_id: maybe_paint_id,
    } in event_reader.read()
    {
        if let Some(entity) = id.get_entity(referencer_res.get_reference_id_to_entity_map()) {
            if let Ok(mut paint_child_mixin) = style_query.get_mut(entity) {
                if let Some(paint_entity) = maybe_paint_id.as_ref().and_then(|paint_id| {
                    paint_id.get_entity(referencer_res.get_reference_id_to_entity_map())
                }) {
                    // Dissolve old style-to-paint relationship
                    if let Ok(mut old_paint_parent_mixin) = paint_query.get_mut(paint_child_mixin.0)
                    {
                        old_paint_parent_mixin.0.retain(|e| *e != entity);
                    }

                    // Establish new style-to-paint relationship
                    if let Ok(mut new_paint_parent_mixin) = paint_query.get_mut(paint_entity) {
                        new_paint_parent_mixin.0.push(entity);
                        paint_child_mixin.0 = paint_entity;
                    }
                }
            }
        }
    }
}

pub fn update_storke_style_input_system(
    referencer_res: Res<ReferencerRes>,
    mut event_reader: EventReader<UpdateStorkeStyleInputEvent>,
    mut style_query: Query<(&mut StrokeArbStyle, &mut PaintChildMixin), With<StrokeArbStyle>>,
    mut paint_query: Query<&mut PaintParentMixin, With<ArbPaint>>,
) {
    for UpdateStorkeStyleInputEvent {
        id,
        paint_id: maybe_paint_id,
        width: maybe_width,
    } in event_reader.read()
    {
        if let Some(entity) = id.get_entity(referencer_res.get_reference_id_to_entity_map()) {
            if let Ok((mut stroke_arb_style, mut paint_child_mixin)) = style_query.get_mut(entity) {
                if let Some(paint_entity) = maybe_paint_id.as_ref().and_then(|paint_id| {
                    paint_id.get_entity(referencer_res.get_reference_id_to_entity_map())
                }) {
                    // Dissolve old style-to-paint relationship
                    if let Ok(mut old_paint_parent_mixin) = paint_query.get_mut(paint_child_mixin.0)
                    {
                        old_paint_parent_mixin.0.retain(|e| *e != entity);
                    }

                    // Establish new style-to-paint relationship
                    if let Ok(mut new_paint_parent_mixin) = paint_query.get_mut(paint_entity) {
                        paint_child_mixin.0 = paint_entity;
                        new_paint_parent_mixin.0.push(entity);
                    }
                }
                if let Some(width) = maybe_width {
                    stroke_arb_style.stroke.width = width.to_pt();
                }
            }
        }
    }
}

pub fn update_drop_shadow_style_input_system(
    referencer_res: Res<ReferencerRes>,
    mut event_reader: EventReader<UpdateDropShadowStyleInputEvent>,
    mut query: Query<&mut DropShadowArbStyle, With<StrokeArbStyle>>,
) {
    for UpdateDropShadowStyleInputEvent {
        id,
        color: maybe_color,
        position: maybe_position,
        spread: maybe_spread,
        blur: maybe_blur,
    } in event_reader.read()
    {
        if let Some(entity) = id.get_entity(referencer_res.get_reference_id_to_entity_map()) {
            if let Ok(mut drop_shadow_arb_style) = query.get_mut(entity) {
                if let Some(color) = maybe_color {
                    drop_shadow_arb_style.color = *color;
                }
                if let Some(position) = maybe_position {
                    drop_shadow_arb_style.position = *position;
                }
                if let Some(spread) = maybe_spread {
                    drop_shadow_arb_style.spread = *spread;
                }
                if let Some(blur) = maybe_blur {
                    drop_shadow_arb_style.blur = *blur;
                }
            }
        }
    }
}

// =============================================================================
// Paint
// =============================================================================

pub fn create_paint_input_system(
    mut commands: Commands,
    mut referencer_res: ResMut<ReferencerRes>,
    mut delete_entity_event_writer: EventWriter<DeleteEntityInputEvent>,
    mut event_reader: EventReader<CreatePaintInputEvent>,
) {
    for CreatePaintInputEvent { paint } in event_reader.read() {
        let maybe_paint_id = match paint {
            Paint::Solid(p) => p.id.clone(),
            Paint::Gradient(p) => p.id.clone(),
            Paint::Image(p) => p.id.clone(),
        };

        // TODO: Should we do this?
        // Remove old paint
        if let Some(prev_paint_entity) = maybe_paint_id
            .as_ref()
            .and_then(|paint_id| referencer_res.get_entity(paint_id))
            .copied()
        {
            delete_entity_event_writer.send(DeleteEntityInputEvent {
                id: ReferenceIdOrEntity::entity(prev_paint_entity),
            });
        }

        // Spawn paint
        let mut paint_entity_commands = match paint {
            Paint::Solid(p) => p.spawn(&mut commands),
            Paint::Gradient(p) => p.spawn(&mut commands),
            Paint::Image(p) => p.spawn(
                &mut commands,
                p.image_id
                    .get_image_id(referencer_res.get_reference_id_to_asset_id_map()),
            ),
        };
        let paint_entity = paint_entity_commands.id();

        paint_entity_commands.insert(PaintParentMixin(SmallVec::new()));

        // Reference paint entity
        if let Some(paint_id) = maybe_paint_id {
            referencer_res.reference_entity(paint_id, paint_entity);
        }
    }
}

pub fn update_solid_paint_input_system(
    referencer_res: Res<ReferencerRes>,
    mut event_reader: EventReader<UpdateSolidPaintInputEvent>,
    mut query: Query<&mut SolidArbPaint>,
) {
    for UpdateSolidPaintInputEvent { id, color } in event_reader.read() {
        if let Some(entity) = id.get_entity(referencer_res.get_reference_id_to_entity_map()) {
            if let Ok(mut solid_arb_paint) = query.get_mut(entity) {
                solid_arb_paint.color = *color;
            }
        }
    }
}

pub fn update_image_paint_input_system(
    referencer_res: Res<ReferencerRes>,
    mut event_reader: EventReader<UpdateImagePaintInputEvent>,
    mut query: Query<(&mut ImageArbPaint, &mut ImageAssetMixin)>,
) {
    for UpdateImagePaintInputEvent {
        id,
        scale_mode: maybe_scale_mode,
        image_id: maybe_image_id,
    } in event_reader.read()
    {
        if let Some(entity) = id.get_entity(referencer_res.get_reference_id_to_entity_map()) {
            if let Ok((mut image_arb_paint, mut image_asset_mixin)) = query.get_mut(entity) {
                if let Some(scale_mode) = maybe_scale_mode {
                    image_arb_paint.scale_mode = *scale_mode;
                }
                if let Some(image_id) = maybe_image_id {
                    image_asset_mixin.0 = Some(*image_id);
                }
            }
        }
    }
}

pub fn update_gradient_paint_input_system(
    referencer_res: Res<ReferencerRes>,
    mut event_reader: EventReader<UpdateGradientPaintInputEvent>,
    mut query: Query<&mut GradientArbPaint>,
) {
    for UpdateGradientPaintInputEvent {
        id,
        variant: maybe_variant,
        stops: maybe_stops,
    } in event_reader.read()
    {
        if let Some(entity) = id.get_entity(referencer_res.get_reference_id_to_entity_map()) {
            if let Ok(mut gradient_arb_paint) = query.get_mut(entity) {
                if let Some(variant) = maybe_variant {
                    gradient_arb_paint.variant = *variant;
                }
                if let Some(stops) = maybe_stops {
                    gradient_arb_paint.stops = SmallVec::from_vec(stops.clone());
                }
            }
        }
    }
}

// =============================================================================
// Asset
// =============================================================================

pub fn create_asset_input_system(
    mut referencer_res: ResMut<ReferencerRes>,
    mut assets_res: ResMut<AssetsRes>,
    mut event_reader: EventReader<CreateAssetInputEvent>,
) {
    for CreateAssetInputEvent { asset } in event_reader.read() {
        let maybe_asset_id = assets_res.insert_asset(asset.clone().into_asset().1);

        if let Some(id) = asset.id.clone() {
            if let Some(asset_id) = maybe_asset_id {
                referencer_res.reference_asset_id(id, asset_id)
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
    referencer_res: Res<ReferencerRes>,
    mut event_reader: EventReader<DeleteEntityInputEvent>,
    children_query: Query<&Children>,
) {
    for DeleteEntityInputEvent { id } in event_reader.read() {
        if let Some(entity) = id.get_entity(referencer_res.get_reference_id_to_entity_map()) {
            commands.entity(entity).insert(Removed).remove_parent();

            if let Ok(children) = children_query.get(entity) {
                for child in children.iter() {
                    commands.entity(*child).insert(Removed);
                }
            }
        }
    }
}

pub fn update_entity_transform_input_system(
    referencer_res: Res<ReferencerRes>,
    mut event_reader: EventReader<UpdateEntityTransformInputEvent>,
    mut query: Query<&mut Transform>,
) {
    for UpdateEntityTransformInputEvent {
        id,
        x: maybe_x,
        y: maybe_y,
        rotation_deg: maybe_rotation_deg,
    } in event_reader.read()
    {
        if let Some(entity) = id.get_entity(referencer_res.get_reference_id_to_entity_map()) {
            if let Ok(mut transform) = query.get_mut(entity) {
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
}

pub fn update_entity_size_input_system(
    referencer_res: Res<ReferencerRes>,
    mut event_reader: EventReader<UpdateEntitySizeInputEvent>,
    mut query: Query<&mut SizeMixin>,
) {
    for UpdateEntitySizeInputEvent { id, size } in event_reader.read() {
        if let Some(entity) = id.get_entity(referencer_res.get_reference_id_to_entity_map()) {
            if let Ok(mut size_mixin) = query.get_mut(entity) {
                size_mixin.0 = *size;
            }
        }
    }
}

pub fn move_entity_input_system(
    referencer_res: Res<ReferencerRes>,
    mut event_reader: EventReader<MoveEntityInputEvent>,
    mut query: Query<&mut Transform>,
) {
    for MoveEntityInputEvent {
        id,
        dx: maybe_dx,
        dy: maybe_dy,
    } in event_reader.read()
    {
        if let Some(entity) = id.get_entity(referencer_res.get_reference_id_to_entity_map()) {
            if let Ok(mut transform) = query.get_mut(entity) {
                transform.translation +=
                    Vec3::new(maybe_dx.unwrap_or(0.0), maybe_dy.unwrap_or(0.0), 0.0);
            }
        }
    }
}

pub fn update_entity_rotation_input_system(
    referencer_res: Res<ReferencerRes>,
    mut event_reader: EventReader<UpdateEntityRotationInputEvent>,
    mut query: Query<(&mut Transform, &SizeMixin)>,
) {
    for UpdateEntityRotationInputEvent { id, rotation_deg } in event_reader.read() {
        if let Some(entity) = id.get_entity(referencer_res.get_reference_id_to_entity_map()) {
            if let Ok((mut transform, SizeMixin(size))) = query.get_mut(entity) {
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
}

pub fn update_entity_visibility_input_system(
    referencer_res: Res<ReferencerRes>,
    mut event_reader: EventReader<UpdateEntityVisibilityInputEvent>,
    mut query: Query<&mut VisibilityMixin>,
) {
    for UpdateEntityVisibilityInputEvent { id, visible } in event_reader.read() {
        if let Some(entity) = id.get_entity(referencer_res.get_reference_id_to_entity_map()) {
            if let Ok(mut visibility_mixin) = query.get_mut(entity) {
                visibility_mixin.0 = *visible;
            }
        }
    }
}

pub fn update_entity_corner_radii_input_system(
    referencer_res: Res<ReferencerRes>,
    mut event_reader: EventReader<UpdateEntityCornerRadiiInputEvent>,
    mut query: Query<&mut CornerRadiiMixin>,
) {
    for UpdateEntityCornerRadiiInputEvent { id, corner_radii } in event_reader.read() {
        if let Some(entity) = id.get_entity(referencer_res.get_reference_id_to_entity_map()) {
            if let Ok(mut corner_radii_mixin) = query.get_mut(entity) {
                corner_radii_mixin.0 = *corner_radii;
            }
        }
    }
}

pub fn update_entity_blend_mode_input_system(
    referencer_res: Res<ReferencerRes>,
    mut event_reader: EventReader<UpdateEntityBlendModeInputEvent>,
    mut query: Query<&mut BlendModeMixin>,
) {
    for UpdateEntityBlendModeInputEvent { id, blend_mode } in event_reader.read() {
        if let Some(entity) = id.get_entity(referencer_res.get_reference_id_to_entity_map()) {
            if let Ok(mut blend_mode_mixin) = query.get_mut(entity) {
                blend_mode_mixin.0 = *blend_mode;
            }
        }
    }
}

pub fn update_entity_opacity_input_system(
    referencer_res: Res<ReferencerRes>,
    mut event_reader: EventReader<UpdateEntityOpacityInputEvent>,
    mut query: Query<&mut OpacityMixin>,
    paint_query: Query<&PaintParentMixin, With<ArbPaint>>,
) {
    for UpdateEntityOpacityInputEvent { id, opacity } in event_reader.read() {
        if let Some(entity) = id.get_entity(referencer_res.get_reference_id_to_entity_map()) {
            if let Ok(mut opacity_mixin) = query.get_mut(entity) {
                opacity_mixin.0 = *opacity;
            } else {
                // TODO: Should Paint be able to update opacity of Style?
                if let Ok(PaintParentMixin(paint_parent)) = paint_query.get(entity) {
                    for parent in paint_parent {
                        if let Ok(mut opacity_mixin) = query.get_mut(*parent) {
                            opacity_mixin.0 = *opacity;
                        }
                    }
                }
            }
        }
    }
}

pub fn update_entity_children_input_system(
    mut commands: Commands,
    referencer_res: Res<ReferencerRes>,
    mut event_reader: EventReader<UpdateEntityChildrenInputEvent>,
) {
    for UpdateEntityChildrenInputEvent {
        id,
        children: new_children,
    } in event_reader.read()
    {
        if let Some(entity) = id.get_entity(referencer_res.get_reference_id_to_entity_map()) {
            if let Some(mut entity_commands) = commands.get_entity(entity) {
                let new_child_entities: Vec<Entity> = new_children
                    .iter()
                    .flat_map(|child_id| {
                        child_id.get_entity(referencer_res.get_reference_id_to_entity_map())
                    })
                    .collect();

                entity_commands.clear_children();
                entity_commands.push_children(&new_child_entities);
            }
        }
    }
}

// =============================================================================
// Script
// =============================================================================

#[cfg(feature = "lua_scripts")]
pub fn register_lua_script_input_system(
    mut lua_res: ResMut<crate::resources::lua::LuaRes>,
    mut event_reader: EventReader<dyn_arb_bundles::events::RegisterLuaScriptInputEvent>,
) {
    use dyn_arb_bundles::events::RegisterLuaScriptInputEvent;

    for RegisterLuaScriptInputEvent { script } in event_reader.read() {
        let (id, script) = script.clone().into_lua_script();
        lua_res.register_script(id, script)
    }
}

// https://bevy-cheatbook.github.io/programming/exclusive.html
#[cfg(feature = "lua_scripts")]
pub fn execute_lua_script_input_system(
    world: &mut World,
    system_state: &mut SystemState<(
        Res<crate::resources::lua::LuaRes>,
        EventReader<dyn_arb_bundles::events::ExecuteLuaScriptInputEvent>,
    )>,
) {
    use crate::resources::lua::{arb_table::FrozenWorld, LuaRes};
    use dyn_arb_bundles::events::ExecuteLuaScriptInputEvent;
    use dyn_arb_lua::freeze::Frozen;
    use piccolo::{Lua, StashedExecutor};

    Frozen::in_scope(world, |frozen_world: FrozenWorld| {
        let mut to_execute_lua: Vec<(Lua, StashedExecutor)> = Vec::new();

        frozen_world.clone().with_mut(|inner_world| {
            let (lua_res, mut event_reader) = system_state.get_mut(inner_world);
            for ExecuteLuaScriptInputEvent { id, args_map } in event_reader.read() {
                match lua_res.setup_lua(id, frozen_world.clone(), args_map.clone()) {
                    Ok((lua, executor)) => to_execute_lua.push((lua, executor)),
                    Err(_) => {
                        // TODO: Handle error
                    }
                };
            }
        });

        for (mut lua, executor) in to_execute_lua {
            // TODO: Handle error
            let _ = LuaRes::execute_lua(&mut lua, &executor);
        }
    });
}
