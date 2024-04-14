use crate::resources::tick::TickRes;
use bevy_ecs::{
    entity::Entity,
    query::{Changed, With, Without},
    system::{Commands, Query, Res},
    world::Ref,
};
use bevy_hierarchy::{Children, Parent};
use bevy_transform::components::Transform;
use dyn_comp_bundles::components::{
    marker::{StaleSize, StaleTransform},
    mixins::SizeMixin,
    nodes::GroupCompNode,
};
use dyn_utils::units::abs::Abs;

// TODO: The current implementation is temporary and not well designed

// TODO: Destory Group if no Children component (which is removed by Bevy when the last child was removed)

pub fn compute_group_children_size(
    tick_res: Res<TickRes>,
    query: Query<(Ref<SizeMixin>, &Children), (With<GroupCompNode>, Changed<SizeMixin>)>,
) {
    for (size_mixin, children) in query.iter() {
        // TODO
    }
}

pub fn compute_group_children_transform(
    tick_res: Res<TickRes>,
    query: Query<(Ref<Transform>, &Children), (With<GroupCompNode>, Changed<Transform>)>,
) {
    for (transform, children) in query.iter() {
        // TODO
    }
}

pub fn mark_group_transform_as_stale(
    mut commands: Commands,
    query: Query<&Parent, Changed<Transform>>,
    group_query: Query<Entity, Without<StaleTransform>>,
) {
    for parent in query.iter() {
        if let Ok(entity) = group_query.get(parent.get()) {
            commands.entity(entity).insert(StaleTransform);
        }
    }
}

pub fn mark_group_size_as_stale(
    mut commands: Commands,
    query: Query<&Parent, Changed<SizeMixin>>,
    group_query: Query<Entity, Without<StaleSize>>,
) {
    for parent in query.iter() {
        if let Ok(entity) = group_query.get(parent.get()) {
            commands.entity(entity).insert(StaleSize);
        }
    }
}

pub fn compute_group_size(
    mut commands: Commands,
    mut query: Query<(Entity, &mut SizeMixin, &Children), (With<GroupCompNode>, With<StaleSize>)>,
    size_mixin_query: Query<(&SizeMixin, &Transform), Without<GroupCompNode>>, // TODO: Group nodes can be child of group node
) {
    for (entity, mut size_mixin, children) in query.iter_mut() {
        let mut min_x = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_y = f32::NEG_INFINITY;

        for &child in children.iter() {
            if let Ok((SizeMixin(child_size), child_transform)) = size_mixin_query.get(child) {
                let left = child_transform.translation.x;
                let right = child_transform.translation.x + child_size.width();
                let top = child_transform.translation.y;
                let bottom = child_transform.translation.y + child_size.height();

                if left < min_x {
                    min_x = left;
                }
                if right > max_x {
                    max_x = right;
                }
                if top < min_y {
                    min_y = top;
                }
                if bottom > max_y {
                    max_y = bottom;
                }
            }
        }

        size_mixin.0.width = Abs::pt(max_x - min_x);
        size_mixin.0.height = Abs::pt(max_y - min_y);

        // Mark the groups size as no longer stale
        commands.entity(entity).remove::<StaleSize>();
    }
}

pub fn compute_group_transform(
    mut commands: Commands,
    mut query: Query<
        (Entity, &mut Transform, &Children),
        (With<GroupCompNode>, With<StaleTransform>),
    >,
    transform_query: Query<&Transform, Without<GroupCompNode>>, // TODO: Group nodes can be child of group node
) {
    for (entity, mut transform, children) in query.iter_mut() {
        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;

        for &child in children.iter() {
            if let Ok(child_transform) = transform_query.get(child) {
                if child_transform.translation.x < min_x {
                    min_x = child_transform.translation.x;
                }
                if child_transform.translation.y < min_y {
                    min_y = child_transform.translation.y;
                }
            }
        }

        transform.translation.x = min_x;
        transform.translation.y = min_y;

        // Mark the groups transform as no longer stale
        commands.entity(entity).remove::<StaleTransform>();
    }
}
