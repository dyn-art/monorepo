use crate::resources::{
    layout::{debug::create_taffy_to_entity_map, layout_tree::LayoutTree, LayoutRes},
    tick::TickRes,
};
use bevy_ecs::{
    change_detection::DetectChanges,
    entity::{Entity, EntityHashMap},
    query::{Added, Changed, Or, With, Without},
    system::{Commands, Query, Res, ResMut},
    world::Ref,
};
use bevy_hierarchy::{Children, Parent};
use bevy_transform::components::Transform;
use dyn_comp_bundles::components::{
    marker::StaleStaticLayout,
    mixins::{
        AbsoluteLayoutElementMixin, HierarchyLevel, SizeMixin, StaticLayoutElementMixin,
        StaticLayoutNodeId, StaticLayoutParentMixin,
    },
};
use dyn_utils::units::abs::Abs;
use glam::Vec3;
use std::collections::HashMap;

pub fn discover_new_static_layout_parents(
    mut commands: Commands,
    mut layout_res: ResMut<LayoutRes>,
    added_layout_parent_query: Query<
        (Entity, &Children),
        (Without<StaticLayoutNodeId>, Added<StaticLayoutParentMixin>),
    >,
    layout_mixin_query: Query<
        (
            Option<&StaticLayoutParentMixin>,
            Option<&StaticLayoutElementMixin>,
            &SizeMixin,
        ),
        (
            Without<StaticLayoutNodeId>,
            Or<(
                With<StaticLayoutParentMixin>,
                Without<AbsoluteLayoutElementMixin>,
            )>,
        ),
    >,
) {
    let mut inserted_nodes: EntityHashMap<taffy::NodeId> = EntityHashMap::default();

    for (entity, children) in added_layout_parent_query.iter() {
        if let Ok((
            maybe_static_layout_parent_mixin,
            maybe_static_layout_element_mixin,
            SizeMixin(size),
        )) = layout_mixin_query.get(entity)
        {
            // Insert the parent into the layout tree.
            // Check whether it was already inserted into the layout tree
            // because the parent could be a child from another parent.
            let node_id = match inserted_nodes.entry(entity) {
                hashbrown::hash_map::Entry::Occupied(entry) => *entry.get(),
                hashbrown::hash_map::Entry::Vacant(entry) => {
                    let new_node_id = layout_res
                        .tree
                        .new_leaf(LayoutTree::merge_layout_parent_with_element(
                            maybe_static_layout_parent_mixin.map(|mixin| &mixin.0),
                            maybe_static_layout_element_mixin.map(|mixin| &mixin.0),
                            &size,
                        ))
                        .unwrap();
                    commands
                        .entity(entity)
                        .insert(StaticLayoutNodeId(new_node_id));

                    entry.insert(new_node_id);

                    new_node_id
                }
            };

            let mut child_node_ids: Vec<taffy::NodeId> = Vec::with_capacity(children.len());

            // Insert the parent's children into the layout tree
            for child in children {
                if let Some((
                    maybe_child_static_layout_parent_mixin,
                    maybe_child_static_layout_elment_mixin,
                    SizeMixin(child_size),
                )) = layout_mixin_query.get(*child).ok()
                {
                    // Insert the child into the layout tree.
                    // Check whether it was already inserted into the layout tree
                    // because the child could be a parent of another child.
                    let child_node_id = match inserted_nodes.entry(*child) {
                        hashbrown::hash_map::Entry::Occupied(entry) => *entry.get(),
                        hashbrown::hash_map::Entry::Vacant(entry) => {
                            let new_node_id = layout_res
                                .tree
                                .new_leaf(LayoutTree::merge_layout_parent_with_element(
                                    maybe_child_static_layout_parent_mixin.map(|mixin| &mixin.0),
                                    maybe_child_static_layout_elment_mixin.map(|mixin| &mixin.0),
                                    child_size,
                                ))
                                .unwrap();
                            commands
                                .entity(*child)
                                .insert(StaticLayoutNodeId(new_node_id));

                            entry.insert(new_node_id);

                            new_node_id
                        }
                    };
                    child_node_ids.push(child_node_id);
                }
            }

            layout_res
                .tree
                .update_children(node_id, &child_node_ids)
                .unwrap();

            commands.entity(entity).insert(StaleStaticLayout);
        }
    }
}

// TODO: System to remove nodes and its children from the layout tree
// if the LayoutParentMixin got removed (e.g. "remove_layout_parents_from_layout_tree")

pub fn update_static_layout_parents_children(
    mut commands: Commands,
    mut layout_res: ResMut<LayoutRes>,
    changed_children_query: Query<
        (Entity, &StaticLayoutNodeId, &Children),
        (With<StaticLayoutNodeId>, Changed<Children>),
    >,
    layout_mixin_query: Query<
        (
            Option<&StaticLayoutParentMixin>,
            Option<&StaticLayoutElementMixin>,
            &SizeMixin,
        ),
        (
            Without<StaticLayoutNodeId>,
            Without<AbsoluteLayoutElementMixin>,
        ),
    >,
) {
    for (entity, StaticLayoutNodeId(node_id), children) in changed_children_query.iter() {
        let mut child_node_ids: Vec<taffy::NodeId> = Vec::with_capacity(children.len());

        for child in children {
            if let Some((
                maybe_child_static_layout_parent_mixin,
                maybe_child_static_layout_elment_mixin,
                SizeMixin(size),
            )) = layout_mixin_query.get(*child).ok()
            {
                let child_node_id = layout_res
                    .tree
                    .new_leaf(LayoutTree::merge_layout_parent_with_element(
                        maybe_child_static_layout_parent_mixin.map(|mixin| &mixin.0),
                        maybe_child_static_layout_elment_mixin.map(|mixin| &mixin.0),
                        size,
                    ))
                    .unwrap();
                child_node_ids.push(child_node_id);
                commands
                    .entity(*child)
                    .insert((StaticLayoutNodeId(child_node_id),));
            }
        }

        layout_res
            .tree
            .update_children(*node_id, &child_node_ids)
            .unwrap();

        commands.entity(entity).insert(StaleStaticLayout);
    }
}

pub fn mark_nodes_with_static_layout_change_as_stale(
    mut commands: Commands,
    mut layout_res: ResMut<LayoutRes>,
    tick_res: Res<TickRes>,
    query: Query<
        (
            Entity,
            &StaticLayoutNodeId,
            Option<&StaticLayoutParentMixin>,
            Option<&StaticLayoutElementMixin>,
            Ref<Transform>,
            Ref<SizeMixin>,
            Option<&Parent>,
        ),
        (
            With<StaticLayoutNodeId>,
            Without<StaleStaticLayout>,
            Or<(
                Changed<StaticLayoutParentMixin>,
                Changed<StaticLayoutElementMixin>,
                Changed<Transform>,
                Changed<SizeMixin>,
            )>,
        ),
    >,
) {
    for (
        entity,
        StaticLayoutNodeId(node_id),
        maybe_static_layout_parent_mixin,
        maybe_static_layout_element_mixin,
        transform,
        size_mixin,
        maybe_parent,
    ) in query.iter()
    {
        // Check if Transform or Size has changed in this update cycle or the last.
        // A change in the current cycle likely indicates a mutation from operations like Translation or Resizing.
        // A change in the last cycle suggests an update by a layout system,
        // whose changes should be ignored by this system.
        //
        // https://discord.com/channels/691052431525675048/1228316069207216130
        if transform.last_changed().get() > tick_res.first_in_cycle.get()
            || size_mixin.last_changed().get() > tick_res.first_in_cycle.get()
        {
            layout_res.tree.update_leaf(
                *node_id,
                LayoutTree::merge_layout_parent_with_element(
                    maybe_static_layout_parent_mixin.map(|mixin| &mixin.0),
                    maybe_static_layout_element_mixin.map(|mixin| &mixin.0),
                    &size_mixin.0,
                ),
            );

            commands
                .entity(maybe_parent.map(|p| p.get()).unwrap_or(entity))
                .insert(StaleStaticLayout);
        }
    }
}

pub fn update_static_layout(
    mut commands: Commands,
    mut layout_res: ResMut<LayoutRes>,
    parent_query: Query<
        (Entity, &HierarchyLevel),
        (Added<StaleStaticLayout>, With<StaticLayoutNodeId>),
    >,
    children_query: Query<&Children, With<StaticLayoutNodeId>>,
    mut to_update_nodes_query: Query<
        (&StaticLayoutNodeId, &mut SizeMixin, &mut Transform),
        With<StaticLayoutNodeId>,
    >,
    layout_node_id_query: Query<(Entity, &StaticLayoutNodeId)>,
) {
    if parent_query.is_empty() {
        return;
    }

    let mut to_recompute_parents: Vec<Entity> = Vec::new();
    let mut lowest_level = u8::MAX;

    // Identify the most top level parent nodes whose layout need to be recomputed
    for (entity, HierarchyLevel(hierarchy_level)) in parent_query.iter() {
        if *hierarchy_level < lowest_level {
            lowest_level = *hierarchy_level;
            to_recompute_parents.clear();
            to_recompute_parents.push(entity);
        } else if *hierarchy_level == lowest_level {
            to_recompute_parents.push(entity);
        }
        commands.entity(entity).remove::<StaleStaticLayout>();
    }

    // Recompute layout
    let taffy_to_entity = create_taffy_to_entity_map(&layout_node_id_query);
    for parent in to_recompute_parents {
        update_node_layout_recursive(
            parent,
            &mut to_update_nodes_query,
            &children_query,
            &mut layout_res,
            true,
            &taffy_to_entity,
        );
    }
}

fn update_node_layout_recursive(
    entity: Entity,
    to_update_nodes_query: &mut Query<
        (&StaticLayoutNodeId, &mut SizeMixin, &mut Transform),
        With<StaticLayoutNodeId>,
    >,
    children_query: &Query<&Children, With<StaticLayoutNodeId>>,
    layout_res: &mut ResMut<LayoutRes>,
    is_root: bool,
    taffy_to_entity: &HashMap<taffy::NodeId, Entity>,
) {
    if let Ok((StaticLayoutNodeId(node_id), mut size_mixin, mut transform)) =
        to_update_nodes_query.get_mut(entity)
    {
        if is_root {
            layout_res
                .tree
                .compute_layout(*node_id, size_mixin.0)
                .unwrap();
            layout_res.tree.print_branch(*node_id, &taffy_to_entity);
        }

        if let Ok(layout) = layout_res.tree.get_layout(*node_id) {
            size_mixin.0.width = Abs::pt(layout.size.width);
            size_mixin.0.height = Abs::pt(layout.size.height);

            log::info!("[update_node_layout_recursive] {:?}: {:?}", node_id, layout); // TODO: REMOVE

            // Don't update root transform because it will be 0
            // if it was used as the starting point for the layout compution
            if !is_root {
                // TODO: Taffy rounds all floating numbers to whole numbers
                // See: https://github.com/DioxusLabs/taffy/issues/77
                //
                // Thus we can't apply the calculated location e.g. when translating
                // because those roundings add up and it doesn't feel right in the UI
                if !transform.is_changed() {
                    transform.translation = Vec3::new(layout.location.x, layout.location.y, 0.0);
                }
            }
        }

        // Check for children and recursively update their layout
        if let Ok(children) = children_query.get(entity) {
            for &child in children.iter() {
                update_node_layout_recursive(
                    child,
                    to_update_nodes_query,
                    children_query,
                    layout_res,
                    false,
                    taffy_to_entity,
                );
            }
        }
    }
}
