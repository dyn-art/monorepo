mod debug;

use crate::resources::{
    layout::{layout_tree::LayoutTree, LayoutRes},
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
    marker::StaleLayout,
    mixins::{HierarchyLevel, LayoutElementMixin, LayoutNodeId, LayoutParentMixin, SizeMixin},
};
use dyn_utils::units::abs::Abs;
use glam::Vec3;

// <div class="flex h-screen items-center justify-center bg-gray-100">
//   <div class="absolute flex bg-blue-500 p-10 text-white" style="top: 50px; left: 50px; width: 300px; height: 300px;">
//     Absolute Parent Container
//     <div class="absolute bg-red-500 p-5 text-white" style="bottom: 20px; right: 20px; width: 100px; height: 100px;">Child Container</div>
//     <div class="bg-green-500 p-5">Not Absolute Container</div>
//   </div>
//   <div class="bg-green-500 p-5">Not Absolute Container</div>
// </div>

pub fn add_new_layout_parents_to_layout_tree(
    mut commands: Commands,
    mut layout_res: ResMut<LayoutRes>,
    added_layout_parent_query: Query<
        (Entity, &Children, Option<&Parent>),
        (Without<LayoutNodeId>, Added<LayoutParentMixin>),
    >,
    layout_mixin_query: Query<
        (
            Option<&LayoutParentMixin>,
            Option<&LayoutElementMixin>,
            &Transform,
            &SizeMixin,
        ),
        Without<LayoutNodeId>,
    >,
    size_mixin_query: Query<&SizeMixin>,
) {
    let mut inserted_nodes: EntityHashMap<taffy::NodeId> = EntityHashMap::default();

    for (entity, children, maybe_parent) in added_layout_parent_query.iter() {
        if let Ok((
            maybe_layout_parent_mixin,
            maybe_layout_element_mixin,
            transform,
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
                            entity,
                            maybe_layout_parent_mixin.map(|mixin| &mixin.0),
                            maybe_layout_element_mixin.map(|mixin| &mixin.0),
                            transform,
                            size,
                            maybe_parent
                                .and_then(|parent| size_mixin_query.get(parent.get()).ok())
                                .map(|size_mixin| &size_mixin.0),
                        ))
                        .unwrap();
                    commands
                        .entity(entity)
                        .insert((LayoutNodeId(new_node_id), StaleLayout));

                    entry.insert(new_node_id);

                    new_node_id
                }
            };

            let mut child_node_ids: Vec<taffy::NodeId> = Vec::with_capacity(children.len());

            // Insert the parent's children into the layout tree
            for child in children {
                if let Some((
                    maybe_child_layout_parent_mixin,
                    maybe_child_layout_elment_mixin,
                    child_transform,
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
                                    *child,
                                    maybe_child_layout_parent_mixin.map(|mixin| &mixin.0),
                                    maybe_child_layout_elment_mixin.map(|mixin| &mixin.0),
                                    child_transform,
                                    child_size,
                                    Some(size),
                                ))
                                .unwrap();
                            commands
                                .entity(*child)
                                .insert((LayoutNodeId(new_node_id), StaleLayout));

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
        }
    }
}

// TODO: System to remove nodes and its children from the layout tree
// if the LayoutParentMixin got removed (e.g. "remove_layout_parents_from_layout_tree")

pub fn update_layout_parent_children(
    mut commands: Commands,
    mut layout_res: ResMut<LayoutRes>,
    changed_children_query: Query<
        (&LayoutNodeId, &Children, &SizeMixin),
        (With<LayoutNodeId>, Changed<Children>),
    >,
    layout_mixin_query: Query<
        (
            Option<&LayoutParentMixin>,
            Option<&LayoutElementMixin>,
            &Transform,
            &SizeMixin,
        ),
        Without<LayoutNodeId>,
    >,
) {
    for (LayoutNodeId(node_id), children, SizeMixin(size)) in changed_children_query.iter() {
        let mut child_node_ids: Vec<taffy::NodeId> = Vec::with_capacity(children.len());

        for child in children {
            if let Some((
                maybe_child_layout_parent_mixin,
                maybe_child_layout_element_mixin,
                child_transform,
                SizeMixin(child_size),
            )) = layout_mixin_query.get(*child).ok()
            {
                let child_node_id = layout_res
                    .tree
                    .new_leaf(LayoutTree::merge_layout_parent_with_element(
                        *child,
                        maybe_child_layout_parent_mixin.map(|mixin| &mixin.0),
                        maybe_child_layout_element_mixin.map(|mixin| &mixin.0),
                        child_transform,
                        child_size,
                        Some(size),
                    ))
                    .unwrap();
                child_node_ids.push(child_node_id);
                commands
                    .entity(*child)
                    .insert((LayoutNodeId(child_node_id), StaleLayout));
            }
        }

        layout_res
            .tree
            .update_children(*node_id, &child_node_ids)
            .unwrap();
    }
}

pub fn mark_nodes_with_layout_change_as_stale(
    mut commands: Commands,
    mut layout_res: ResMut<LayoutRes>,
    tick_res: Res<TickRes>,
    query: Query<
        (
            Entity,
            &LayoutNodeId,
            Option<&LayoutParentMixin>,
            Option<&LayoutElementMixin>,
            Ref<Transform>,
            Ref<SizeMixin>,
            Option<&Parent>,
        ),
        (
            With<LayoutNodeId>,
            Without<StaleLayout>,
            Or<(
                Changed<LayoutParentMixin>,
                Changed<LayoutElementMixin>,
                Changed<Transform>,
                Changed<SizeMixin>,
            )>,
        ),
    >,
    size_mixin_query: Query<&SizeMixin>,
) {
    for (
        entity,
        LayoutNodeId(node_id),
        maybe_layout_parent_mixin,
        maybe_layout_element_mixin,
        transform,
        size_mixin,
        maybe_parent,
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
                LayoutTree::merge_layout_parent_with_element(
                    entity,
                    maybe_layout_parent_mixin.map(|mixin| &mixin.0),
                    maybe_layout_element_mixin.map(|mixin| &mixin.0),
                    transform.as_ref(),
                    &size_mixin.as_ref().0,
                    maybe_parent
                        .and_then(|parent| size_mixin_query.get(parent.get()).ok())
                        .map(|size_mixin| &size_mixin.0),
                ),
            );
            commands.entity(entity).insert(StaleLayout);
        }
    }
}

pub fn update_layout(
    mut commands: Commands,
    mut layout_res: ResMut<LayoutRes>,
    parent_query: Query<
        (Entity, &HierarchyLevel, Option<&Parent>),
        (Added<StaleLayout>, With<LayoutNodeId>),
    >,
    children_query: Query<&Children, With<LayoutNodeId>>,
    mut to_update_nodes_query: Query<
        (&LayoutNodeId, &mut SizeMixin, &mut Transform),
        With<LayoutNodeId>,
    >,
) {
    let mut to_recompute_parents: Vec<Entity> = Vec::new();
    let mut lowest_level = u8::MAX;

    // Identify the most top level parent nodes whose layout need to be recomputed
    for (entity, HierarchyLevel(hierarchy_level), parent) in parent_query.iter() {
        let to_recompute = parent.map(|p| p.get()).unwrap_or(entity);
        if *hierarchy_level < lowest_level {
            lowest_level = *hierarchy_level;
            to_recompute_parents.clear();
            to_recompute_parents.push(to_recompute);
        } else if *hierarchy_level == lowest_level {
            to_recompute_parents.push(to_recompute);
        }
        commands.entity(entity).remove::<StaleLayout>();
    }

    if to_recompute_parents.len() > 0 {
        log::info!("[update_layout] {:?}", to_recompute_parents); // TODO: REMOVE
    }

    // Recompute layout
    for parent in to_recompute_parents {
        update_node_layout_recursive(
            parent,
            &mut to_update_nodes_query,
            &children_query,
            &mut layout_res,
            true,
        );
    }
}

fn update_node_layout_recursive(
    entity: Entity,
    to_update_nodes_query: &mut Query<
        (&LayoutNodeId, &mut SizeMixin, &mut Transform),
        With<LayoutNodeId>,
    >,
    children_query: &Query<&Children, With<LayoutNodeId>>,
    layout_res: &mut ResMut<LayoutRes>,
    compute_layout: bool,
) {
    if let Ok((LayoutNodeId(node_id), mut size_mixin, mut transform)) =
        to_update_nodes_query.get_mut(entity)
    {
        if compute_layout {
            log::info!("[update_node_layout_recursive] Compute: {:?}", size_mixin.0); // TODO: REMOVE
            layout_res
                .tree
                .compute_layout(*node_id, size_mixin.0)
                .unwrap();
        }

        if let Ok(layout) = layout_res.tree.get_layout(*node_id) {
            log::info!("[update_node_layout_recursive] {:?}: {:?}", entity, layout); // TODO: REMOVE
            size_mixin.0.width = Abs::pt(layout.size.width);
            size_mixin.0.height = Abs::pt(layout.size.height);
            transform.translation = Vec3::new(layout.location.x, layout.location.y, 0.0);
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
                );
            }
        }
    }
}
