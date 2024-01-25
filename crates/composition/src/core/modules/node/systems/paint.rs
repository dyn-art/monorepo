use bevy_ecs::{
    entity::Entity,
    query::{Changed, Or, With, Without},
    system::{Commands, Query},
};
use bevy_hierarchy::{Children, Parent};
use glam::{Mat3, Vec2};

use crate::core::modules::node::components::{
    mixins::{DimensionMixin, ImageContentMixin},
    types::{
        GradientPaint, GradientPaintVariant, ImageCropPaintTransform, ImagePaint,
        ImagePaintScaleMode, ImageTilePaintTransform, LinearGradientPaintTransform, Node, Paint,
        RadialGradientPaintTransform,
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
        (&DimensionMixin, &mut ImagePaint, &ImageContentMixin),
        Or<(
            Changed<DimensionMixin>,
            Changed<ImagePaint>,
            Changed<ImageContentMixin>,
        )>,
    >,
) {
    for (dimension, mut paint, image_content) in query.iter_mut() {
        match &mut paint.scale_mode {
            ImagePaintScaleMode::Tile { transform } => match transform {
                ImageTilePaintTransform::Basic {
                    rotation,
                    scaling_factor,
                } => {
                    *transform = ImageTilePaintTransform::Internal {
                        rotation: *rotation,
                        tile_width: image_content.width * *scaling_factor,
                        tile_height: image_content.height * *scaling_factor,
                    };
                }
                _ => {}
            },
            ImagePaintScaleMode::Crop {
                transform: outer_transform,
            } => match outer_transform {
                ImageCropPaintTransform::Basic { transform } => {
                    let (new_image_width, new_image_height, new_image_transform) =
                        calculate_cropped_image_transform(
                            (dimension.width, dimension.height),
                            (image_content.width, image_content.height),
                            transform,
                        );

                    *outer_transform = ImageCropPaintTransform::Internal {
                        applied_transform: new_image_transform,
                        image_width: new_image_width,
                        image_height: new_image_height,
                        crop_transform: *transform,
                    };
                }
                ImageCropPaintTransform::Internal {
                    crop_transform,
                    image_width,
                    image_height,
                    applied_transform: transform,
                } => {
                    let (new_image_width, new_image_height, new_image_transform) =
                        calculate_cropped_image_transform(
                            (dimension.width, dimension.height),
                            (image_content.width, image_content.height),
                            crop_transform,
                        );

                    *image_width = new_image_width;
                    *image_height = new_image_height;
                    *transform = new_image_transform;
                }
            },
            _ => {}
        }
    }
}

fn calculate_cropped_image_transform(
    container_dimensions: (f32, f32),
    image_content: (f32, f32),
    transform: &Mat3,
) -> (f32, f32, Mat3) {
    let (container_width, container_height) = container_dimensions;
    let (image_width, image_height) = image_content;

    // Calculate aspect ratios for container and image
    let container_ratio = container_width / container_height;
    let image_ratio = image_width / image_height;

    // Determine new image dimensions based on aspect ratio comparison
    let (adjusted_image_width, adjusted_image_height) = if image_ratio > container_ratio {
        (container_height * image_ratio, container_height)
    } else {
        (container_width, container_width / image_ratio)
    };

    // Calculate scale adjustment ratios
    let x_ratio = container_width / adjusted_image_width;
    let y_ratio = container_height / adjusted_image_height;

    // Extract scale components from the matrix and adjust them
    let scale_x = transform.x_axis.x;
    let scale_y = transform.y_axis.y;
    let adjusted_scale_x = (1.0 / scale_x) * x_ratio;
    let adjusted_scale_y = (1.0 / scale_y) * y_ratio;

    // Calculate adjusted translation.
    let tx = -adjusted_image_width * transform.z_axis.x * adjusted_scale_x;
    let ty = -adjusted_image_height * transform.z_axis.y * adjusted_scale_y;

    // Construct the adjusted transformation matrix
    let adjusted_transform = Mat3::from_scale_angle_translation(
        Vec2::new(adjusted_scale_x, adjusted_scale_y),
        0.0,
        Vec2::new(tx, ty),
    );

    return (
        adjusted_image_width,
        adjusted_image_height,
        adjusted_transform,
    );
}

pub fn update_gradient_paint_transform(
    mut query: Query<
        (&DimensionMixin, &mut GradientPaint),
        Or<(Changed<DimensionMixin>, Changed<GradientPaint>)>,
    >,
) {
    for (dimension, mut paint) in query.iter_mut() {
        match &mut paint.variant {
            GradientPaintVariant::Linear {
                transform: outer_transform,
            } => match outer_transform {
                LinearGradientPaintTransform::Basic { transform } => {
                    let (start, end) = extract_linear_gradient_params_from_transform(
                        dimension.width,
                        dimension.height,
                        transform,
                    );
                    *outer_transform = LinearGradientPaintTransform::Internal { start, end };
                }
                _ => {}
            },
            GradientPaintVariant::Radial { transform } => match transform {
                RadialGradientPaintTransform::Basic { transform } => {
                    // TODO
                }
                _ => {}
            },
        }
    }
}

/// Helper function to extract the x and y positions of the start and end of the linear gradient
/// (scale is not important here).
///
/// Credits:
/// https://github.com/figma-plugin-helper-functions/figma-plugin-helpers/tree/master
fn extract_linear_gradient_params_from_transform(
    shape_width: f32,
    shape_height: f32,
    transform: &Mat3,
) -> (Vec2, Vec2) {
    let mx_inv = transform.inverse();
    let start_end = [Vec2::new(0.0, 0.5), Vec2::new(1.0, 0.5)].map(|p| mx_inv.transform_point2(p));

    (
        Vec2::new(start_end[0].x * shape_width, start_end[0].y * shape_height),
        Vec2::new(start_end[1].x * shape_width, start_end[1].y * shape_height),
    )
}
