use dyn_comp_common::common::{Size, Viewport};
use dyn_comp_core::resources::composition::CompositionRes;
use glam::Vec2;

/// Transforms a point from the canvas coordinate system to the viewport coordinate system.
///
/// This function is essential for aligning cursor interactions on the canvas with
/// the SVG content, which is manipulated by the viewport. The canvas coordinates
/// are transformed to match the scale and position defined by the viewport.
pub fn transform_point_to_viewport(
    comp_res: &CompositionRes,
    point: &Vec2,
    apply_min_offset: bool,
) -> Vec2 {
    let CompositionRes {
        viewport: Viewport {
            physical_position,
            physical_size,
        },
        size: Size(size),
        ..
    } = comp_res;

    let normalized_x = point.x / size.x;
    let normalized_y = point.y / size.y;

    let normalized_point = Vec2::new(
        normalized_x * physical_size.x,
        normalized_y * physical_size.y,
    );

    return if apply_min_offset {
        normalized_point + Vec2::new(physical_position.x, physical_position.y)
    } else {
        normalized_point
    };
}

pub fn rotate_point(point: &Vec2, pivot: &Vec2, angle_rad: f32) -> Vec2 {
    Vec2::new(
        (point.x - pivot.x) * angle_rad.cos() - (point.y - pivot.y) * angle_rad.sin() + pivot.x,
        (point.x - pivot.x) * angle_rad.sin() + (point.y - pivot.y) * angle_rad.cos() + pivot.y,
    )
}
