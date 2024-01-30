use std::f32::consts::PI;

use bevy_ecs::{
    entity::Entity,
    query::{Changed, Or},
    system::{Commands, Query},
};
use glam::Vec2;

use crate::modules::node::components::{
    mixins::{Anchor, AnchorCommand, DimensionMixin, PathMixin},
    types::EllipseNode,
};

pub fn construct_ellipse_path(
    mut commands: Commands,
    query: Query<
        (Entity, &EllipseNode, &DimensionMixin),
        Or<(Changed<EllipseNode>, Changed<DimensionMixin>)>,
    >,
) {
    for (entity, ellipse, dimension) in query.iter() {
        let radius = Vec2::new(dimension.width / 2.0, dimension.height / 2.0);
        let center = radius;
        let start_angle = ellipse.arc_data.starting_angle;
        let end_angle = ellipse.arc_data.ending_angle;
        let inner_radius_ratio = ellipse.arc_data.inner_radius_ratio;
        let mut vertices = Vec::new();

        // Calculate start and end coordinates on the boundary of the outer ellipse
        let outer_start = Vec2::new(
            center.x + start_angle.cos() * radius.x,
            center.y + start_angle.sin() * radius.y,
        );
        let outer_end = Vec2::new(
            center.x + end_angle.cos() * radius.x,
            center.y + end_angle.sin() * radius.y,
        );

        // When creating a SVG path for a full circle with an inner radius (creating a "donut" shape),
        // the path starts and ends at the same point on the circle's boundary. However, when the
        // starting and ending angles are exactly the same, the SVG renderer doesn't "know" that it needs
        // to draw a full circle. Instead, it sees this as a zero-length path, and so it doesn't draw anything.
        //
        // By adding a small offset (epsilon) to the ending angle, we ensure that the starting and ending
        // points are slightly different. This "tricks" the SVG renderer into drawing a nearly complete
        // circle, which appears as a full circle due to the small size of the offset.
        //
        // The value of epsilon is chosen to be small enough that the resulting gap in the circle is not
        // visible to the naked eye (0.001 radians is approximately 0.057 degrees), but large enough to
        // ensure that the SVG renderer recognizes the path as a non-zero length path, and therefore
        // draws the expected shape.
        //
        // Note that this solution is a workaround to a known issue with SVG rendering, and may not be
        // necessary if the SVG rendering behavior is changed in future versions of SVG or the rendering engine.
        let epsilon = 0.001;

        // Calculate start and end coordinates on the boundary of the inner ellipse
        let mut inner_start = Vec2::new(center.x, center.y);
        let mut inner_end = Vec2::new(center.x, center.y);
        if inner_radius_ratio > 0.0 {
            inner_start = Vec2::new(
                center.x + (start_angle + epsilon).cos() * inner_radius_ratio * radius.x,
                center.y + (start_angle + epsilon).sin() * inner_radius_ratio * radius.y,
            );
            inner_end = Vec2::new(
                center.x + (end_angle + epsilon).cos() * inner_radius_ratio * radius.x,
                center.y + (end_angle + epsilon).sin() * inner_radius_ratio * radius.y,
            );
        }

        // If the ellipse slice is bigger than 180 degrees (half circle),
        // we have to tell SVG to take the longest route (by default it will take the shortest),
        // which is why the largeArcFlag is set to 1 for a > 180 degrees circle
        let large_arc_flag = start_angle < 0.0 || end_angle < 0.0 || end_angle - start_angle >= PI;

        // If the ellipse slice is a full circle (360 degrees),
        // we have to draw the arc in a clockwise direction for it to appear correct to the viewer,
        // which is why the sweepFlagOuter is set to 0 when it's a full circle
        let sweep_flag_outer =
            !(start_angle < 0.0 || end_angle < 0.0 || end_angle - start_angle >= PI * 2.0);

        // On the other hand, for the inner arc,
        // drawing in a counter-clockwise direction will ensure the hole
        // in the donut segment is rendered correctly,
        // which is why the sweepFlagInner is set to 1 for a full circle
        let sweep_flag_inner =
            start_angle < 0.0 || end_angle < 0.0 || end_angle - start_angle >= PI * 2.0;

        // Start at the center of the ellipse
        vertices.push(Anchor {
            position: inner_start,
            command: AnchorCommand::MoveTo,
        });

        // Draw line to starting point on the boundary of the ellipse
        vertices.push(Anchor {
            position: outer_start,
            command: AnchorCommand::LineTo,
        });

        // Draw the outer arc to the end point on the boundary of the ellipse
        vertices.push(Anchor {
            position: outer_end,
            command: AnchorCommand::ArcTo {
                radius,
                x_axis_rotation: 0.0,
                large_arc_flag,
                sweep_flag: sweep_flag_outer,
            },
        });

        // Draw back to the center (initial starting point) if no inner radius,
        // if inner radius draw to inner radius end
        vertices.push(Anchor {
            position: inner_end,
            command: AnchorCommand::LineTo,
        });

        // If inner radius draw inner radius to the inner start
        if inner_radius_ratio > 0.0 {
            vertices.push(Anchor {
                position: inner_start,
                command: AnchorCommand::ArcTo {
                    radius: radius * inner_radius_ratio,
                    x_axis_rotation: 0.0,
                    large_arc_flag,
                    sweep_flag: sweep_flag_inner,
                },
            });
        }

        // Close the path
        vertices.push(Anchor {
            position: inner_start,
            command: AnchorCommand::ClosePath,
        });

        // Insert or update the PathMixin component for the entity
        commands.entity(entity).insert(PathMixin { vertices });
    }
}
