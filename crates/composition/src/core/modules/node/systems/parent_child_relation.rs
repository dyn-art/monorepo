use bevy_ecs::{
    entity::Entity,
    query::{Changed, With, Without},
    system::{Commands, Query},
};
use bevy_hierarchy::{Children, Parent};

use crate::core::modules::node::components::{
    mixins::DimensionMixin,
    types::{Node, Paint},
};

// TODO: Issue with Mutable Access - https://discord.com/channels/691052431525675048/1199265475155202108
pub fn update_paint_dimension_based_on_parent_node(
    mut commands: Commands,
    node_children_query: Query<
        (Entity, &DimensionMixin, &Children),
        (With<Node>, Changed<DimensionMixin>),
    >,
    // mut paint_with_dimension_query: Query<(Entity, &Parent, &mut DimensionMixin), With<Paint>>,
    paint_without_dimension_query: Query<(Entity, &Parent), (With<Paint>, Without<DimensionMixin>)>,
) {
    for (node_entity, dimension, children) in node_children_query.iter() {
        // Update existing DimensionMixin for children with Paint and DimensionMixin
        // for (paint_entity, parent, mut dimension_mixin) in paint_with_dimension_query.iter_mut() {
        //     if children.contains(&paint_entity) && parent.get() == node_entity {
        //         dimension_mixin.width = dimension.width;
        //         dimension_mixin.height = dimension.height;
        //     }
        // }

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
