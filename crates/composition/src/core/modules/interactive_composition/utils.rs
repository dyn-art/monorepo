use glam::Vec2;

use crate::core::modules::composition::resources::composition::CompositionRes;

pub fn apply_view_box_offset(composition: &CompositionRes, value: &Vec2) -> Vec2 {
    let scale_x = composition.view_box.width / composition.width;
    let scale_y = composition.view_box.height / composition.height;

    Vec2::new(value.x * scale_x, value.y * scale_y)
}
