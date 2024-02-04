use bevy_ecs::{
    entity::Entity,
    query::{Changed, Or},
    system::{Commands, Query},
};
use dyn_composition::modules::node::components::{
    mixins::{DimensionMixin, ImageContentMixin},
    types::{GradientPaint, GradientPaintVariant, ImagePaint, ImagePaintScaleMode},
};
use glam::{Mat3, Vec2};

use crate::components::{
    SVGGradientPaint, SVGGradientPaintVariant, SVGImagePaint, SVGImagePaintScaleMode,
};

// =============================================================================
// SVG Image Paint
// =============================================================================

pub fn map_to_svg_image_paint(
    mut commands: Commands,
    mut query: Query<
        (Entity, &DimensionMixin, &mut ImagePaint, &ImageContentMixin),
        Or<(
            Changed<DimensionMixin>,
            Changed<ImagePaint>,
            Changed<ImageContentMixin>,
        )>,
    >,
) {
    for (entity, dimension, mut paint, image_content) in query.iter_mut() {
        match &mut paint.scale_mode {
            ImagePaintScaleMode::Tile {
                rotation,
                scaling_factor,
                ..
            } => {
                // Insert or update the SVGImagePaint component for the entity
                commands.entity(entity).insert(SVGImagePaint {
                    scale_mode: SVGImagePaintScaleMode::Tile {
                        rotation: *rotation,
                        tile_width: image_content.width * *scaling_factor,
                        tile_height: image_content.height * *scaling_factor,
                    },
                });
            }
            ImagePaintScaleMode::Crop { transform, .. } => {
                let (new_image_width, new_image_height, new_image_transform) =
                    calculate_cropped_image_transform(
                        (dimension.width, dimension.height),
                        (image_content.width, image_content.height),
                        transform,
                    );

                // Insert or update the SVGImagePaint component for the entity
                commands.entity(entity).insert(SVGImagePaint {
                    scale_mode: SVGImagePaintScaleMode::Crop {
                        transform: new_image_transform,
                        image_width: new_image_width,
                        image_height: new_image_height,
                    },
                });
            }
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

// =============================================================================
// SVG Image Paint
// =============================================================================

pub fn map_to_svg_gradient_paint(
    mut commands: Commands,
    mut query: Query<
        (Entity, &DimensionMixin, &mut GradientPaint),
        Or<(Changed<DimensionMixin>, Changed<GradientPaint>)>,
    >,
) {
    for (entity, dimension, mut paint) in query.iter_mut() {
        match &mut paint.variant {
            GradientPaintVariant::Linear { transform } => {
                let (start, end) = extract_linear_gradient_params_from_transform(
                    dimension.width,
                    dimension.height,
                    transform,
                );

                // Insert or update the SVGGradientPaint component for the entity
                commands.entity(entity).insert(SVGGradientPaint {
                    variant: SVGGradientPaintVariant::Linear { start, end },
                });
            }
            GradientPaintVariant::Radial { transform } => {
                // TODO
            }
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
