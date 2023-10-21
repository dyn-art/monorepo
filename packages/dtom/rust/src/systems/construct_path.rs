use bevy_ecs::{
    entity::Entity,
    query::{Changed, Or},
    system::{Commands, Query},
};
use glam::Vec2;

use crate::node::mixins::{Anchor, LayoutMixin, PathMixin, RectangleCornerMixin};

// =============================================================================
// Rectangle
// =============================================================================

fn create_corner_anchor(x: f32, y: f32, radius: i16, max_radius: f32) -> Anchor {
    let final_radius = f32::min(radius as f32, max_radius);

    let controls = if final_radius > 0.0 {
        Some((
            Vec2::new(x + final_radius, y),
            Vec2::new(x, y + final_radius),
        ))
    } else {
        None
    };

    return Anchor {
        position: Vec2::new(x, y),
        controls,
    };
}

pub fn construct_rectangle_path(
    mut query: Query<
        (Entity, &RectangleCornerMixin, &LayoutMixin),
        Or<(Changed<RectangleCornerMixin>, Changed<LayoutMixin>)>,
    >,
    mut commands: Commands,
) {
    for (entity, rect_corner, layout) in query.iter_mut() {
        // Calculate maximum possible radius
        let max_radius = (layout.width.min(layout.height) / 2) as f32;

        let mut vertices: Vec<Anchor> = Vec::new();

        // Top left corner
        vertices.push(create_corner_anchor(
            0.0,
            0.0,
            rect_corner.top_left_radius,
            max_radius,
        ));

        // Top right corner
        vertices.push(create_corner_anchor(
            layout.width as f32,
            0.0,
            rect_corner.top_right_radius,
            max_radius,
        ));

        // Bottom right corner
        vertices.push(create_corner_anchor(
            layout.width as f32,
            layout.height as f32,
            rect_corner.bottom_right_radius,
            max_radius,
        ));

        // Bottom left corner
        vertices.push(create_corner_anchor(
            0.0,
            layout.height as f32,
            rect_corner.bottom_left_radius,
            max_radius,
        ));

        // Create PathMixin with the constructed vertices
        let path_mixin = PathMixin { vertices };

        // Add the constructed PathMixin to the entity
        commands.entity(entity).insert(path_mixin);
    }
}
