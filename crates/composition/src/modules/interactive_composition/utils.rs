use glam::Vec2;

use crate::modules::composition::resources::composition::CompositionRes;

/// Transforms a point from the canvas coordinate system to the view box coordinate system.
///
/// This function is essential for aligning cursor interactions on the canvas with
/// the SVG content, which is manipulated by the view box. The canvas coordinates
/// are transformed to match the scale and position defined by the view box.
pub fn transform_point_to_view_box(
    composition: &CompositionRes,
    point: &Vec2,
    apply_min_offset: bool,
) -> Vec2 {
    let CompositionRes {
        view_box,
        width,
        height,
        ..
    } = composition;

    let normalized_x = point.x / width;
    let normalized_y = point.y / height;

    let normalized_point = Vec2::new(
        normalized_x * view_box.width,
        normalized_y * view_box.height,
    );

    return if apply_min_offset {
        normalized_point + Vec2::new(view_box.min_x, view_box.min_y)
    } else {
        normalized_point
    };
}
