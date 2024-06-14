use dyn_arb_bundles::properties::Viewport;
use dyn_arb_core::resources::canvas::ArtboardRes;
use glam::Vec2;

/// Transforms a point from the canvas coordinate system to the viewport coordinate system.
///
/// This function is essential for aligning cursor interactions on the canvas with
/// the SVG content, which is manipulated by the viewport. The canvas coordinates
/// are transformed to match the scale and position defined by the viewport.
pub fn transform_point_to_viewport(
    arb_res: &ArtboardRes,
    point: &Vec2,
    apply_min_offset: bool,
) -> Vec2 {
    let ArtboardRes {
        viewport: Viewport {
            physical_position,
            physical_size,
        },
        size,
        ..
    } = arb_res;

    let normalized_point = *point / size.to_vec2() * physical_size.to_vec2();

    return if apply_min_offset {
        normalized_point + *physical_position
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
