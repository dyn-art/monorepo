use crate::resources::{
    layout::{layout_tree::LayoutTree, LayoutRes},
    tick::TickRes,
};
use bevy_ecs::{
    change_detection::DetectChanges,
    entity::Entity,
    query::{Added, Changed, Or, With, Without},
    system::{Commands, Query, Res, ResMut},
    world::Ref,
};
use bevy_hierarchy::{Children, Parent};
use bevy_transform::components::Transform;
use dyn_comp_bundles::components::{
    marker::StaleLayout,
    mixins::{LayoutElementMixin, LayoutNodeId, LayoutParentMixin, SizeMixin},
};
use dyn_utils::units::abs::Abs;
use glam::Vec3;
use hashbrown::HashSet;

pub fn add_new_layout_parents_to_layout_tree(
    mut commands: Commands,
    mut layout_res: ResMut<LayoutRes>,
    added_layout_parent_query: Query<
        (Entity, &Children),
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
) {
    for (parent, children) in added_layout_parent_query.iter() {
        if let Ok((
            maybe_parent_layout_parent_mixin,
            maybe_parent_layout_element_mixin,
            parent_transform,
            SizeMixin(parent_size),
        )) = layout_mixin_query.get(parent)
        {
            // Insert parent into layout tree
            let parent_node_id = layout_res
                .tree
                .new_leaf(
                    parent,
                    LayoutTree::merge_layout_parent_with_element(
                        maybe_parent_layout_parent_mixin.map(|mixin| &mixin.0),
                        maybe_parent_layout_element_mixin.map(|mixin| &mixin.0),
                        parent_transform,
                        parent_size,
                    ),
                )
                .unwrap();
            commands
                .entity(parent)
                .insert((LayoutNodeId(parent_node_id), StaleLayout));

            // Insert parent's children into layout tree
            for child in children {
                if let Some((
                    maybe_child_layout_parent_mixin,
                    maybe_child_layout_elment_mixin,
                    child_transform,
                    SizeMixin(child_size),
                )) = layout_mixin_query.get(*child).ok()
                {
                    let child_node_id = layout_res
                        .tree
                        .new_leaf(
                            *child,
                            LayoutTree::merge_layout_parent_with_element(
                                maybe_child_layout_parent_mixin.map(|mixin| &mixin.0),
                                maybe_child_layout_elment_mixin.map(|mixin| &mixin.0),
                                child_transform,
                                child_size,
                            ),
                        )
                        .unwrap();
                    commands
                        .entity(*child)
                        .insert((LayoutNodeId(child_node_id), StaleLayout));
                }
            }
        }
    }
}

// TODO: System to remove nodes and its children from the layout tree
// if the LayoutParentMixin got removed (e.g. "remove_layout_parents_from_layout_tree")

pub fn update_layout_parent_children(
    mut commands: Commands,
    mut layout_res: ResMut<LayoutRes>,
    changed_children_query: Query<&Children, (With<LayoutNodeId>, Changed<Children>)>,
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
    for children in changed_children_query.iter() {
        for child in children {
            if let Some((
                maybe_layout_parent_mixin,
                maybe_layout_element_mixin,
                transform,
                SizeMixin(size),
            )) = layout_mixin_query.get(*child).ok()
            {
                let node_id = layout_res
                    .tree
                    .new_leaf(
                        *child,
                        LayoutTree::merge_layout_parent_with_element(
                            maybe_layout_parent_mixin.map(|mixin| &mixin.0),
                            maybe_layout_element_mixin.map(|mixin| &mixin.0),
                            transform,
                            size,
                        ),
                    )
                    .unwrap();
                commands
                    .entity(*child)
                    .insert((LayoutNodeId(node_id), StaleLayout));
            }
        }
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
) {
    for (
        entity,
        LayoutNodeId(node_id),
        maybe_layout_parent_mixin,
        maybe_layout_element_mixin,
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
                LayoutTree::merge_layout_parent_with_element(
                    maybe_layout_parent_mixin.map(|mixin| &mixin.0),
                    maybe_layout_element_mixin.map(|mixin| &mixin.0),
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

    parent_query: Query<(Entity, Option<&Parent>), (Added<StaleLayout>, With<LayoutNodeId>)>,
    children_query: Query<&Children, With<LayoutNodeId>>,
    mut to_update_nodes_query: Query<
        (&LayoutNodeId, &mut SizeMixin, &mut Transform),
        With<LayoutNodeId>,
    >,
) {
    let mut to_update_parents: HashSet<Entity> = HashSet::new();

    // Identify parent nodes whose layout needs to be recomputed
    for (entity, parent) in parent_query.iter() {
        to_update_parents.insert(parent.map(|p| p.get()).unwrap_or(entity));
        commands.entity(entity).remove::<StaleLayout>();
    }

    // Recompute layout
    for parent in to_update_parents {
        if let Ok((LayoutNodeId(parent_node_id), mut parent_size_mixin, mut parent_transform)) =
            to_update_nodes_query.get_mut(parent)
        {
            log::info!("[update_layout] Compute Layout: {:?}", parent_size_mixin.0); // TODO: REMOVE
            layout_res
                .tree
                .compute_layouts(*parent_node_id, parent_size_mixin.0)
                .unwrap();

            if let Ok(layout) = layout_res.tree.get_layout(*parent_node_id) {
                log::info!("[update_layout] Parent {:?}: {:?}", parent, layout); // TODO: REMOVE
                parent_size_mixin.0.width = Abs::pt(layout.size.width);
                parent_size_mixin.0.height = Abs::pt(layout.size.height);
                parent_transform.translation = Vec3::new(layout.location.x, layout.location.y, 0.0);
            }

            if let Ok(children) = children_query.get(parent) {
                for child in children {
                    if let Ok((
                        LayoutNodeId(child_node_id),
                        mut child_size_mixin,
                        mut child_transform,
                    )) = to_update_nodes_query.get_mut(*child)
                    {
                        if let Ok(child_layout) = layout_res.tree.get_layout(*child_node_id) {
                            log::info!("[update_layout] Child {:?}: {:?}", child, child_layout); // TODO: REMOVE
                            child_size_mixin.0.width = Abs::pt(child_layout.size.width);
                            child_size_mixin.0.height = Abs::pt(child_layout.size.height);
                            child_transform.translation =
                                Vec3::new(child_layout.location.x, child_layout.location.y, 0.0);
                        }
                    }
                }
            }
        }
    }
}
