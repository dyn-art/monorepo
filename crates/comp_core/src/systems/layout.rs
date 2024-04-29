// v3
// TaffyTree is a resource. Nodes whose parent has layout set to active,
// gets a Lyout Component which contains the Taffy node id.
// This way we don't have to manage a separate hashmap
// and can just check whether an Entity has a Layout Component and go from there?
//
// Then we have multiple systems:
// 1. System that checks on layout active changes in the FrameNode Component.
//    If it is set to active we init it in the Taffy tree and add a Layout
//    Component to each child and itself.
// 2. Here we check if a node has a Lyout Component and whether its Style has changed.
//    If its Style has changed we update it in the Taffy tree and mark it as dirty
//    or something on the Component level so like "layout.dirty = true".
//    Maybe we should just mark the parent as dirty?
// 3. Here we check for each dirty Layout Component and recompute the layout
//    accordingly.

use crate::resources::layout::{layout_tree::LayoutTree, LayoutRes};
use bevy_ecs::{
    entity::Entity,
    query::{Added, Changed, Or, With, Without},
    system::{Commands, Query, ResMut},
};
use bevy_hierarchy::{Children, Parent};
use dyn_comp_bundles::components::{
    marker::StaleLayout,
    mixins::{LayoutTreeNodeId, LeafLayoutMixin, ParentLayoutMixin, SizeMixin},
    nodes::FrameCompNode,
};

pub fn insert_new_node_tree_into_layout_tree(
    mut commands: Commands,
    mut layout_res: ResMut<LayoutRes>,
    frame_without_layout_query: Query<
        (Entity, &FrameCompNode, &Children),
        (Changed<FrameCompNode>, Without<LayoutTreeNodeId>),
    >,
    layout_mixin_query: Query<
        (Option<&ParentLayoutMixin>, Option<&LeafLayoutMixin>),
        Without<LayoutTreeNodeId>,
    >,
) {
    for (entity, frame_comp_node, children) in frame_without_layout_query.iter() {
        if frame_comp_node.layout {
            // Insert parent into layout tree
            let (parent_layout_mixin, leaf_layout_mixin) =
                layout_mixin_query.get(entity).unwrap_or((None, None));
            let node_id = layout_res
                .tree
                .new_leaf(LayoutTree::layout_mixins_to_style(
                    parent_layout_mixin,
                    leaf_layout_mixin,
                ))
                .unwrap();
            commands
                .entity(entity)
                .insert((LayoutTreeNodeId(node_id), StaleLayout));

            // Insert children into layout tree
            for child in children {
                if let Some((maybe_parent_layout_mixin, maybe_leaf_layout_mixin)) =
                    layout_mixin_query.get(*child).ok()
                {
                    let node_id = layout_res
                        .tree
                        .new_leaf(LayoutTree::layout_mixins_to_style(
                            maybe_parent_layout_mixin,
                            maybe_leaf_layout_mixin,
                        ))
                        .unwrap();
                    commands
                        .entity(entity)
                        .insert((LayoutTreeNodeId(node_id), StaleLayout));
                }
            }
        }
    }

    // TODO: Add system to unregister layout
}

pub fn insert_new_children_into_layout_tree(
    mut commands: Commands,
    mut layout_res: ResMut<LayoutRes>,
    query: Query<(Entity, &Children), (With<LayoutTreeNodeId>, Changed<Children>)>,
    layout_mixin_query: Query<
        (Option<&ParentLayoutMixin>, Option<&LeafLayoutMixin>),
        Without<LayoutTreeNodeId>,
    >,
) {
    for (entity, children) in query.iter() {
        for child in children {
            if let Some((maybe_parent_layout_mixin, maybe_leaf_layout_mixin)) =
                layout_mixin_query.get(*child).ok()
            {
                let node_id = layout_res
                    .tree
                    .new_leaf(LayoutTree::layout_mixins_to_style(
                        maybe_parent_layout_mixin,
                        maybe_leaf_layout_mixin,
                    ))
                    .unwrap();
                commands
                    .entity(entity)
                    .insert((LayoutTreeNodeId(node_id), StaleLayout));
            }
        }
    }
}

pub fn sync_updated_layout_styles_to_layout_tree(
    mut commands: Commands,
    mut layout_res: ResMut<LayoutRes>,
    mut query: Query<
        (
            Entity,
            &LayoutTreeNodeId,
            Option<&ParentLayoutMixin>,
            Option<&LeafLayoutMixin>,
        ),
        (
            With<LayoutTreeNodeId>,
            Or<(Changed<ParentLayoutMixin>, Changed<LeafLayoutMixin>)>,
        ),
    >,
) {
    for (entity, LayoutTreeNodeId(node_id), maybe_parent_layout_mixin, maybe_leaf_layout_mixin) in
        query.iter()
    {
        layout_res.tree.update_leaf(
            *node_id,
            LayoutTree::layout_mixins_to_style(maybe_parent_layout_mixin, maybe_leaf_layout_mixin),
        );
        commands.entity(entity).insert(StaleLayout);
    }
}

pub fn update_layout(
    mut commands: Commands,
    mut layout_res: ResMut<LayoutRes>,
    parent_query: Query<(Entity, &LayoutTreeNodeId, &SizeMixin, &Children), Added<StaleLayout>>,
    leaf_query: Query<(Entity, &Parent), Added<StaleLayout>>,
    parent_size_query: Query<(&LayoutTreeNodeId, &SizeMixin)>,
) {
    let mut to_update_entities: Vec<Entity> = Vec::new();

    for (entity, LayoutTreeNodeId(node_id), SizeMixin(size), children) in parent_query.iter() {
        layout_res.tree.compute_layouts(*node_id, *size).unwrap();
        to_update_entities.push(entity);
        to_update_entities.extend(children.iter());
        commands.entity(entity).remove::<StaleLayout>();
    }

    for (entity, parent) in leaf_query.iter() {
        if let Ok((LayoutTreeNodeId(node_id), SizeMixin(size))) =
            parent_size_query.get(parent.get())
        {
            layout_res.tree.compute_layouts(*node_id, *size).unwrap();
            to_update_entities.push(parent.get());
            to_update_entities.push(entity);
            commands.entity(entity).remove::<StaleLayout>();
        }
    }

    // TODO: Update entities
}
