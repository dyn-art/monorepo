use crate::resources::{
    layout::{debug::create_taffy_to_entity_map, layout_tree::LayoutTree, LayoutRes},
    tick::TickRes,
};
use bevy_ecs::{
    change_detection::DetectChanges,
    entity::{Entity, EntityHashMap, EntityHashSet},
    query::{Added, Changed, Or, With, Without},
    system::{Commands, Query, Res, ResMut},
    world::Ref,
};
use bevy_hierarchy::{Children, Parent};
use bevy_transform::components::Transform;
use dyn_arb_bundles::components::{
    marker::{Removed, StaleStaticLayout},
    mixins::{
        AbsoluteLayoutElementMixin, LayoutParentSizingMode, SizeMixin, StaticLayoutElementMixin,
        StaticLayoutNodeId, StaticLayoutParentMixin,
    },
};
use dyn_utils::units::abs::Abs;
use glam::Vec3;
use std::collections::HashMap;

pub fn discover_new_static_layout_parent_nodes(
    mut commands: Commands,
    mut layout_res: ResMut<LayoutRes>,
    added_layout_parent_query: Query<
        (Entity, Option<&StaticLayoutNodeId>, &Children),
        Added<StaticLayoutParentMixin>,
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
    let mut inserted_layout_nodes: EntityHashMap<taffy::NodeId> = EntityHashMap::default();

    for (entity, maybe_layout_node_id, children) in added_layout_parent_query.iter() {
        if let Ok((
            maybe_static_layout_parent_mixin,
            maybe_static_layout_element_mixin,
            SizeMixin(size),
        )) = layout_mixin_query.get(entity)
        {
            // Process and potentially update or create a new layout node
            let layout_node_id = *inserted_layout_nodes.entry(entity).or_insert_with(|| {
                let style = LayoutTree::merge_layout_parent_with_element(
                    maybe_static_layout_parent_mixin.map(|m| &m.0),
                    maybe_static_layout_element_mixin.map(|m| &m.0),
                    &size,
                );

                // Create or update layout node based on presence of an existing node id
                if let Some(StaticLayoutNodeId(existing_layout_node_id)) = maybe_layout_node_id {
                    layout_res.tree.update_leaf(*existing_layout_node_id, style);
                    commands.entity(entity).insert(StaleStaticLayout);
                    *existing_layout_node_id
                } else {
                    let new_layout_node_id = layout_res.tree.new_leaf(style).unwrap();
                    commands
                        .entity(entity)
                        .insert((StaticLayoutNodeId(new_layout_node_id), StaleStaticLayout));
                    new_layout_node_id
                }
            });

            let child_layout_node_ids: Vec<taffy::NodeId> = children
                .iter()
                .filter_map(|child| {
                    if let Some((
                        maybe_child_static_layout_parent_mixin,
                        maybe_child_static_layout_elment_mixin,
                        SizeMixin(child_size),
                    )) = layout_mixin_query.get(*child).ok()
                    {
                        Some(*inserted_layout_nodes.entry(*child).or_insert_with(|| {
                            let style = LayoutTree::merge_layout_parent_with_element(
                                maybe_child_static_layout_parent_mixin.map(|m| &m.0),
                                maybe_child_static_layout_elment_mixin.map(|m| &m.0),
                                child_size,
                            );
                            let new_layout_node_id = layout_res.tree.new_leaf(style).unwrap();

                            // Note: Not marking child node as stale
                            // because the parent is marked as stale
                            commands
                                .entity(*child)
                                .insert(StaticLayoutNodeId(new_layout_node_id));

                            new_layout_node_id
                        }))
                    } else {
                        None
                    }
                })
                .rev()
                .collect();

            if let Err(e) = layout_res
                .tree
                .update_children(layout_node_id, &child_layout_node_ids)
            {
                log::error!(
                    "Failed to update children for node {:?}: {:?}",
                    layout_node_id,
                    e
                );
            }
        }
    }
}

// TODO: System to remove nodes and its children from the layout tree
// if the LayoutParentMixin got removed (e.g. "remove_layout_parents_from_layout_tree")

pub fn update_static_layout_parent_nodes_children(
    mut commands: Commands,
    mut layout_res: ResMut<LayoutRes>,
    changed_children_query: Query<
        (Entity, &StaticLayoutNodeId, &Children),
        (With<StaticLayoutNodeId>, Changed<Children>),
    >,
    layout_mixin_query: Query<
        (
            Option<&StaticLayoutNodeId>,
            Option<&StaticLayoutParentMixin>,
            Option<&StaticLayoutElementMixin>,
            &SizeMixin,
        ),
        (Without<AbsoluteLayoutElementMixin>,),
    >,
) {
    for (entity, StaticLayoutNodeId(layout_node_id), children) in changed_children_query.iter() {
        // Process each child, adding new nodes to the layout tree if necessary
        let child_layout_node_ids: Vec<taffy::NodeId> = children
            .iter()
            .filter_map(|child| {
                if let Some((
                    maybe_child_layout_node_id,
                    maybe_child_static_layout_parent_mixin,
                    maybe_child_static_layout_elment_mixin,
                    SizeMixin(size),
                )) = layout_mixin_query.get(*child).ok()
                {
                    if let Some(StaticLayoutNodeId(child_layout_node_id)) =
                        maybe_child_layout_node_id
                    {
                        Some(*child_layout_node_id)
                    } else {
                        let style = LayoutTree::merge_layout_parent_with_element(
                            maybe_child_static_layout_parent_mixin.map(|mixin| &mixin.0),
                            maybe_child_static_layout_elment_mixin.map(|mixin| &mixin.0),
                            size,
                        );
                        let new_layout_node_id = layout_res.tree.new_leaf(style).unwrap();

                        commands
                            .entity(*child)
                            // Note: Not marking child node as stale
                            // because the parent gets marked as stale
                            .insert(StaticLayoutNodeId(new_layout_node_id));

                        Some(new_layout_node_id)
                    }
                } else {
                    None
                }
            })
            .rev()
            .collect();

        if let Err(e) = layout_res
            .tree
            .update_children(*layout_node_id, &child_layout_node_ids)
        {
            log::error!(
                "Failed to update children for node {:?}: {:?}",
                layout_node_id,
                e
            );
        }

        // Mark parent node as stale
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
        StaticLayoutNodeId(layout_node_id),
        maybe_static_layout_parent_mixin,
        maybe_static_layout_element_mixin,
        transform,
        size_mixin,
    ) in query.iter()
    {
        // Check if Transform or Size has been altered during the current update cycle or the previous one.
        // Modifications within the current cycle (e.g., Translation, Resizing) indicate active user or system interactions
        // that require immediate attention to ensure accurate layout representation.
        // Changes from the previous cycle are typically residual updates from the layout system itself,
        // and should not trigger further updates in this system to avoid redundancy and potential feedback loops.
        //
        // https://discord.com/channels/691052431525675048/1228316069207216130
        //
        // We monitor Transform changes even if not directly utilized, to uphold and enforce layout-driven positioning,
        // ensuring that all spatial adjustments align strictly with the intended layout specifications.
        if transform.last_changed().get() > tick_res.first_in_cycle.get()
            || size_mixin.last_changed().get() > tick_res.first_in_cycle.get()
        {
            let new_style = LayoutTree::merge_layout_parent_with_element(
                maybe_static_layout_parent_mixin.map(|mixin| &mixin.0),
                maybe_static_layout_element_mixin.map(|mixin| &mixin.0),
                &size_mixin.0,
            );
            layout_res.tree.update_leaf(*layout_node_id, new_style);
            commands.entity(entity).insert(StaleStaticLayout);
        }
    }
}

pub fn remove_stale_layout_nodes(
    mut layout_res: ResMut<LayoutRes>,
    query: Query<&StaticLayoutNodeId, (Added<Removed>, With<StaticLayoutNodeId>)>,
) {
    for StaticLayoutNodeId(layout_node_id) in query.iter() {
        if let Err(e) = layout_res.tree.remove(*layout_node_id) {
            log::error!("Failed to remove layout node {:?}: {:?}", layout_node_id, e);
        }
    }
}

pub fn update_static_layout(
    mut commands: Commands,
    mut layout_res: ResMut<LayoutRes>,
    stale_nodes_query: Query<Entity, (Added<StaleStaticLayout>, With<StaticLayoutNodeId>)>,
    parent_query: Query<(Entity, Option<&Parent>), With<StaticLayoutNodeId>>,
    children_query: Query<&Children, With<StaticLayoutNodeId>>,
    mut to_update_nodes_query: Query<
        (
            &StaticLayoutNodeId,
            &mut SizeMixin,
            &mut Transform,
            Option<&StaticLayoutParentMixin>,
        ),
        With<StaticLayoutNodeId>,
    >,
    layout_node_id_query: Query<(Entity, &StaticLayoutNodeId)>,
) {
    if stale_nodes_query.is_empty() {
        return;
    }

    let mut to_recompute_parents: EntityHashSet = EntityHashSet::default();

    // Find the most top-level parent for each stale node
    for entity in stale_nodes_query.iter() {
        let top_parent = find_topmost_parent(entity, &parent_query);
        to_recompute_parents.insert(top_parent);
        commands.entity(entity).remove::<StaleStaticLayout>();
    }

    // Recompute layout
    let taffy_to_entity = create_taffy_to_entity_map(&layout_node_id_query);
    for parent in to_recompute_parents.iter() {
        update_node_layout_recursive(
            *parent,
            &mut to_update_nodes_query,
            &children_query,
            &mut layout_res,
            true,
            &taffy_to_entity,
        );
    }
}

fn find_topmost_parent(
    entity: Entity,
    parent_query: &Query<(Entity, Option<&Parent>), With<StaticLayoutNodeId>>,
) -> Entity {
    let mut next = entity;
    let mut current = entity;
    while let Ok((current_entity, Some(current_parent))) = parent_query.get(next) {
        current = current_entity;
        next = current_parent.get();
    }
    return current;
}

fn update_node_layout_recursive(
    entity: Entity,
    to_update_nodes_query: &mut Query<
        (
            &StaticLayoutNodeId,
            &mut SizeMixin,
            &mut Transform,
            Option<&StaticLayoutParentMixin>,
        ),
        With<StaticLayoutNodeId>,
    >,
    children_query: &Query<&Children, With<StaticLayoutNodeId>>,
    layout_res: &mut ResMut<LayoutRes>,
    is_root: bool,
    taffy_to_entity: &HashMap<taffy::NodeId, Entity>,
) {
    if let Ok((
        StaticLayoutNodeId(layout_node_id),
        mut size_mixin,
        mut transform,
        maybe_static_layout_parent_mixin,
    )) = to_update_nodes_query.get_mut(entity)
    {
        // Root nodes have their layout computed only once at the beginning.
        if is_root {
            let mut taffy_size = taffy::Size {
                width: taffy::AvailableSpace::Definite(size_mixin.0.width()),
                height: taffy::AvailableSpace::Definite(size_mixin.0.height()),
            };

            if let Some(StaticLayoutParentMixin(static_layout_parent)) =
                maybe_static_layout_parent_mixin
            {
                taffy_size.width = match static_layout_parent.horizontal_sizing_mode {
                    LayoutParentSizingMode::Fixed => {
                        taffy::AvailableSpace::Definite(size_mixin.0.width())
                    }
                    LayoutParentSizingMode::Hug => taffy::AvailableSpace::MinContent,
                };
                taffy_size.height = match static_layout_parent.vertical_sizing_mode {
                    LayoutParentSizingMode::Fixed => {
                        taffy::AvailableSpace::Definite(size_mixin.0.height())
                    }
                    LayoutParentSizingMode::Hug => taffy::AvailableSpace::MinContent,
                };
            };

            if let Err(e) = layout_res.tree.compute_layout(*layout_node_id, taffy_size) {
                log::error!(
                    "Failed to compute layout for node {:?}: {:?}",
                    layout_node_id,
                    e
                );
                return;
            }
            // layout_res
            //     .tree
            //     .print_branch(*layout_node_id, &taffy_to_entity); // TODO: REMOVE
        }

        // Apply computed layout to the node's properties
        //
        // TODO: Taffy rounds all floating numbers to whole numbers
        // See: https://github.com/DioxusLabs/taffy/issues/77
        if let Ok(layout) = layout_res.tree.get_layout(*layout_node_id) {
            // log::info!(
            //     "[update_node_layout_recursive] {:?}: {:?}",
            //     layout_node_id,
            //     layout
            // ); // TODO: REMOVE

            size_mixin.0.width = Abs::pt(layout.size.width);
            size_mixin.0.height = Abs::pt(layout.size.height);

            // Skipping this step for the root
            // since its position is typically static or already set.
            // Also, Taffy sets it to 0, if its the root node of the layout computation.
            if !is_root {
                transform.translation = Vec3::new(layout.location.x, layout.location.y, 0.0);
            }
        } else {
            log::error!("Layout not found for node {:?}", layout_node_id);
        }

        // Recursively update the layout for all child nodes
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
