use crate::resources::{
    layout::{layout_tree::LayoutTree, LayoutRes},
    tick::TickRes,
};
use bevy_ecs::{
    change_detection::DetectChanges,
    entity::{Entity, EntityHashSet},
    query::{Added, Changed, Or, With, Without},
    system::{Commands, ParamSet, Query, Res, ResMut},
    world::Ref,
};
use bevy_hierarchy::{Children, Parent};
use bevy_transform::components::Transform;
use dyn_comp_bundles::components::{
    marker::StaleLayout,
    mixins::{LayoutTreeNodeId, LeafLayoutMixin, ParentLayoutMixin, SizeMixin},
    nodes::FrameCompNode,
};
use dyn_utils::units::abs::Abs;
use glam::Vec3;

pub fn discover_nodes_for_layout_trees(
    mut commands: Commands,
    mut layout_res: ResMut<LayoutRes>,
    changed_frame_without_layout_query: Query<
        (Entity, &FrameCompNode, &Children),
        (Changed<FrameCompNode>, Without<LayoutTreeNodeId>),
    >,
    changed_children_query: Query<(Entity, &Children), (With<LayoutTreeNodeId>, Changed<Children>)>,
    layout_mixin_query: Query<
        (
            Option<&ParentLayoutMixin>,
            Option<&LeafLayoutMixin>,
            &Transform,
            &SizeMixin,
        ),
        Without<LayoutTreeNodeId>,
    >,
) {
    let mut synced_parent = EntityHashSet::default();

    for (entity, frame_comp_node, children) in changed_frame_without_layout_query.iter() {
        if frame_comp_node.layout {
            // Insert parent into layout tree
            if let Ok((parent_layout_mixin, leaf_layout_mixin, transform, SizeMixin(size))) =
                layout_mixin_query.get(entity)
            {
                let node_id = layout_res
                    .tree
                    .new_leaf(LayoutTree::node_mixins_to_style(
                        parent_layout_mixin,
                        leaf_layout_mixin,
                        transform,
                        size,
                    ))
                    .unwrap();
                synced_parent.insert(entity);
                commands
                    .entity(entity)
                    .insert((LayoutTreeNodeId(node_id), StaleLayout));

                // Insert children into layout tree
                for child in children {
                    if let Some((
                        maybe_parent_layout_mixin,
                        maybe_leaf_layout_mixin,
                        transform,
                        SizeMixin(size),
                    )) = layout_mixin_query.get(*child).ok()
                    {
                        let node_id = layout_res
                            .tree
                            .new_leaf(LayoutTree::node_mixins_to_style(
                                maybe_parent_layout_mixin,
                                maybe_leaf_layout_mixin,
                                transform,
                                size,
                            ))
                            .unwrap();
                        commands
                            .entity(entity)
                            .insert((LayoutTreeNodeId(node_id), StaleLayout));
                    }
                }
            }
        }
    }

    for (entity, children) in changed_children_query.iter() {
        if !synced_parent.contains(&entity) {
            for child in children {
                if let Some((
                    maybe_parent_layout_mixin,
                    maybe_leaf_layout_mixin,
                    transform,
                    SizeMixin(size),
                )) = layout_mixin_query.get(*child).ok()
                {
                    let node_id = layout_res
                        .tree
                        .new_leaf(LayoutTree::node_mixins_to_style(
                            maybe_parent_layout_mixin,
                            maybe_leaf_layout_mixin,
                            transform,
                            size,
                        ))
                        .unwrap();
                    commands
                        .entity(entity)
                        .insert((LayoutTreeNodeId(node_id), StaleLayout));
                }
            }
        }
    }

    // TODO: Remove removed children from layout tree

    // TODO: Add system to unregister layout
}

pub fn mark_nodes_with_layout_change_as_stale(
    mut commands: Commands,
    mut layout_res: ResMut<LayoutRes>,
    tick_res: Res<TickRes>,
    query: Query<
        (
            Entity,
            &LayoutTreeNodeId,
            Option<&ParentLayoutMixin>,
            Option<&LeafLayoutMixin>,
            Ref<Transform>,
            Ref<SizeMixin>,
        ),
        (
            With<LayoutTreeNodeId>,
            Without<StaleLayout>,
            Or<(
                Changed<ParentLayoutMixin>,
                Changed<LeafLayoutMixin>,
                Changed<Transform>,
                Changed<SizeMixin>,
            )>,
        ),
    >,
) {
    for (
        entity,
        LayoutTreeNodeId(node_id),
        maybe_parent_layout_mixin,
        maybe_leaf_layout_mixin,
        transform,
        size_mixin,
    ) in query.iter()
    {
        // Check if Transform or Size has changed in this update cycle or the last.
        // A change in the current cycle likely indicates a mutation from operations like Translation or Resizing.
        // A change in the last cycle suggests an update by a Constraint system,
        // whose changes should be ignored by this system.
        //
        // https://discord.com/channels/691052431525675048/1228316069207216130
        if transform.last_changed().get() > tick_res.first_in_cycle.get()
            || size_mixin.last_changed().get() > tick_res.first_in_cycle.get()
        {
            layout_res.tree.update_leaf(
                *node_id,
                LayoutTree::node_mixins_to_style(
                    maybe_parent_layout_mixin,
                    maybe_leaf_layout_mixin,
                    transform.as_ref(),
                    &size_mixin.as_ref().0,
                ),
            );
            commands.entity(entity).insert(StaleLayout);
        }
    }
}

pub fn update_layout(
    mut commands: Commands,
    mut layout_res: ResMut<LayoutRes>,
    mut queries: ParamSet<(
        Query<
            (Entity, &LayoutTreeNodeId, &SizeMixin, &Children),
            (Added<StaleLayout>, With<LayoutTreeNodeId>),
        >,
        Query<(&LayoutTreeNodeId, &SizeMixin), With<LayoutTreeNodeId>>,
        Query<(&LayoutTreeNodeId, &mut SizeMixin, &mut Transform), With<LayoutTreeNodeId>>,
    )>,
    leaf_query: Query<(Entity, &Parent), (Added<StaleLayout>, With<LayoutTreeNodeId>)>,
) {
    let mut to_update_entities: Vec<Entity> = Vec::new();

    for (entity, LayoutTreeNodeId(node_id), SizeMixin(size), children) in queries.p0().iter() {
        layout_res.tree.compute_layouts(*node_id, *size).unwrap();
        to_update_entities.push(entity);
        to_update_entities.extend(children.iter());
        commands.entity(entity).remove::<StaleLayout>();
    }

    for (entity, parent) in leaf_query.iter() {
        if let Ok((LayoutTreeNodeId(node_id), SizeMixin(size))) = queries.p1().get(parent.get()) {
            layout_res.tree.compute_layouts(*node_id, *size).unwrap();
            to_update_entities.push(parent.get());
            to_update_entities.push(entity);
            commands.entity(entity).remove::<StaleLayout>();
        }
    }

    for entity in to_update_entities {
        if let Ok((LayoutTreeNodeId(node_id), mut size_mixin, mut transform)) =
            queries.p2().get_mut(entity)
        {
            if let Ok(layout) = layout_res.tree.get_layout(*node_id) {
                log::info!("[update_layout] {:?}: {:?}", entity, layout); // TODO: REMOVE
                size_mixin.0.width = Abs::pt(layout.size.width);
                size_mixin.0.height = Abs::pt(layout.size.height);
                transform.translation = Vec3::new(layout.location.x, layout.location.y, 0.0);
            }
        }
    }
}
