use bevy_ecs::{
    entity::Entity,
    query::{Changed, Or, With, Without},
    system::{Commands, Query},
};
use bevy_hierarchy::{Children, Parent};
use glam::{Mat3, Vec2};

use crate::core::modules::node::components::{
    mixins::DimensionMixin,
    types::{
        ImageFillFitPaintTransform, ImagePaint, ImagePaintScaleMode, ImageTilePaintTransform, Node,
        Paint,
    },
};

// Note: To avoid Bevy's ECS conflict between mutable and immutable references of the same component
// (`DimensionMixin` in this case), we explicitly specify `Without` in the queries.
// This is necessary because Bevy ensures safe access to components, and having both mutable and
// immutable references to the same component type in different queries can lead to runtime errors.
// In our system, `With<Node>` and `With<Paint>` could potentially conflict, as they might coexist on the same entity.
// Adding `Without<Paint>` and `Without<Node>` to the respective queries resolves this conflict by ensuring
// that entities in one query cannot be present in the other, thereby upholding Rust's borrowing rules.
// https://discord.com/channels/691052431525675048/1199265475155202108
// https://github.com/bevyengine/bevy/blob/main/errors/B0002.md
pub fn update_paint_dimension_based_on_parent_node(
    mut commands: Commands,
    node_children_query: Query<
        (Entity, &DimensionMixin, &Children),
        (With<Node>, Without<Paint>, Changed<DimensionMixin>),
    >,
    mut paint_with_dimension_query: Query<
        (Entity, &Parent, &mut DimensionMixin),
        (With<Paint>, Without<Node>),
    >,
    paint_without_dimension_query: Query<
        (Entity, &Parent),
        (With<Paint>, Without<DimensionMixin>, Without<Node>),
    >,
) {
    for (node_entity, dimension, children) in node_children_query.iter() {
        // Update existing DimensionMixin for children with Paint and DimensionMixin
        for (paint_entity, parent, mut dimension_mixin) in paint_with_dimension_query.iter_mut() {
            if children.contains(&paint_entity) && parent.get() == node_entity {
                dimension_mixin.width = dimension.width;
                dimension_mixin.height = dimension.height;
            }
        }

        // Add DimensionMixin for children with Paint but without DimensionMixin
        for (paint_entity, parent) in paint_without_dimension_query.iter() {
            if children.contains(&paint_entity) && parent.get() == node_entity {
                commands.entity(paint_entity).insert(DimensionMixin {
                    width: dimension.width,
                    height: dimension.height,
                });
            }
        }
    }
}

pub fn update_image_paint_transform(
    mut query: Query<
        (&DimensionMixin, &mut ImagePaint),
        Or<(Changed<DimensionMixin>, Changed<ImagePaint>)>,
    >,
) {
    for (dimension, mut paint) in query.iter_mut() {
        match &mut paint.scale_mode {
            ImagePaintScaleMode::Fill { transform } | ImagePaintScaleMode::Fit { transform } => {
                match transform {
                    ImageFillFitPaintTransform::Simple { rotation } => {
                        let center_x = dimension.width / 2.0;
                        let center_y = dimension.height / 2.0;
                        let rotation_angle = rotation.to_radians();

                        let mut mat3_transform =
                            Mat3::from_translation(Vec2::new(-center_x, -center_y))
                                * Mat3::from_rotation_z(rotation_angle)
                                * Mat3::from_translation(Vec2::new(center_x, center_y));
                        mat3_transform.z_axis.x = dimension.width;
                        mat3_transform.z_axis.y = dimension.height;

                        // Translate to origin, rotate, translate back
                        *transform = ImageFillFitPaintTransform::Transform {
                            transform: mat3_transform,
                        };
                    }
                    ImageFillFitPaintTransform::Transform { transform } => {
                        transform.z_axis.x = dimension.width;
                        transform.z_axis.y = dimension.height;
                    }
                }
            }
            ImagePaintScaleMode::Tile { transform } => {
                match transform {
                    ImageTilePaintTransform::Simple {
                        rotation,
                        scaling_factor,
                    } => {
                        let center_x = dimension.width / 2.0;
                        let center_y = dimension.height / 2.0;

                        // Translate to origin, scale, rotate, translate back
                        *transform = ImageTilePaintTransform::Transform {
                            transform: Mat3::from_translation(Vec2::new(-center_x, -center_y))
                                * Mat3::from_scale(Vec2::splat(*scaling_factor))
                                * Mat3::from_rotation_z(*rotation)
                                * Mat3::from_translation(Vec2::new(center_x, center_y)),
                        };
                    }
                    ImageTilePaintTransform::Transform { transform } => {
                        transform.z_axis.x = dimension.width;
                        transform.z_axis.y = dimension.height;
                    }
                }
            }
            _ => {}
        }
    }
}
