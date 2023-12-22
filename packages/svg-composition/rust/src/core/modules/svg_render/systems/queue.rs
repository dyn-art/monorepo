use std::{collections::HashMap, mem::take};

use bevy_ecs::{entity::Entity, system::ResMut};

use crate::core::{
    events::output_event::RenderUpdateEvent,
    mixin_change::{MixinChange, MixinChangeChildrenMixin},
    modules::svg_render::resources::{
        changed_components::{ChangedComponentsRes, ChangedNode, ChangedPaint},
        svg_composition::SVGCompositionRes,
    },
};

pub fn queue_render_changes(
    mut changed: ResMut<ChangedComponentsRes>,
    mut svg_composition: ResMut<SVGCompositionRes>,
) {
    let changed_nodes = take(&mut changed.changed_nodes);
    let changed_paints = take(&mut changed.changed_paints);
    let mut updates: Vec<RenderUpdateEvent> = Vec::new();

    // Process nodes & paints and collect render updates emitted during this process
    process_nodes(&changed_nodes, &mut svg_composition, &mut updates);
    process_paints(&changed_paints, &mut svg_composition, &mut updates);

    // Forward render updates into output event channel
    svg_composition.forward_render_updates(updates);
}

// =============================================================================
// Paint
// =============================================================================

fn process_paints(
    changed_paints: &HashMap<Entity, ChangedPaint>,
    svg_composition: &mut SVGCompositionRes,
    updates: &mut Vec<RenderUpdateEvent>,
) {
    for (entity, paint) in changed_paints {
        process_paint(*entity, &paint, svg_composition, updates);
    }
}

/// Processes a paint entity by updating its corresponding SVG element/s based on the provided changes.
fn process_paint(
    entity: Entity,
    changed_paint: &ChangedPaint,
    svg_composition: &mut SVGCompositionRes,
    updates: &mut Vec<RenderUpdateEvent>,
) {
    // Attempt to get or create the paint associated with the entity
    let maybe_paint =
        svg_composition.get_or_create_paint(entity, &changed_paint.paint, &changed_paint.parent_id);

    // Apply collected changes to the SVG paint and drain updates
    if let Some(svg_paint) = maybe_paint {
        svg_paint.apply_paint_change(&changed_paint);
        updates.extend(svg_paint.drain_updates());
    }
}

// =============================================================================
// Node
// =============================================================================

#[derive(Debug)]
struct ChangedNodeBranch<'a> {
    entity: Entity,
    changed: &'a ChangedNode,
    children: Vec<ChangedNodeBranch<'a>>,
}

/// Processes nodes by building and traversing dependency trees
/// to ensure that parents are processed before its children.
fn process_nodes(
    changed_nodes: &HashMap<Entity, ChangedNode>,
    svg_composition: &mut SVGCompositionRes,
    updates: &mut Vec<RenderUpdateEvent>,
) {
    let dependency_tree = build_dependency_trees(changed_nodes);

    // Traverse and process each root node and its descendants
    for root in dependency_tree {
        process_tree_node(&root, svg_composition, updates);
    }
}

/// Builds dependency trees from the changed nodes.
fn build_dependency_trees<'a>(
    changed_nodes: &'a HashMap<Entity, ChangedNode>,
) -> Vec<ChangedNodeBranch<'a>> {
    let mut roots = Vec::new();

    // Identify root nodes (those without parents or whose parents are not in changed_nodes)
    for (&entity, changed_node) in changed_nodes {
        if changed_node.parent_id.is_none()
            || !changed_nodes.contains_key(&changed_node.parent_id.unwrap())
        {
            roots.push(build_branch(entity, changed_nodes));
        }
    }

    return roots;
}

/// Recursively builds a branch in the dependency tree.
fn build_branch<'a>(
    entity: Entity,
    changed_nodes: &'a HashMap<Entity, ChangedNode>,
) -> ChangedNodeBranch<'a> {
    let changed_node = changed_nodes.get(&entity).expect("Node must exist");

    // Build children branches
    let children: Vec<ChangedNodeBranch> =
         // Build branches for children of the current node,
         // while ensuring the correct order of the children based on the children mixin
        if let Some(children_mixin) = find_children_mixin(&changed_node.changes) {
            children_mixin
                .children
                .0
                .iter()
                .filter_map(|&child_entity| {
                    changed_nodes
                        .get(&child_entity)
                        .map(|_| build_branch(child_entity, changed_nodes))
                })
                .collect()
        } else {
            // Build branches for children of the current node (order not relevant)
            changed_nodes
                .iter()
                .filter_map(|(&child_entity, child_node)| {
                    (child_node.parent_id == Some(entity))
                        .then(|| build_branch(child_entity, changed_nodes))
                })
                .collect()
        };

    return ChangedNodeBranch {
        entity,
        changed: changed_node,
        children,
    };
}

/// Finds children mixin change from a list of changes.
fn find_children_mixin(changes: &[MixinChange]) -> Option<&MixinChangeChildrenMixin> {
    changes.iter().find_map(|change| match change {
        MixinChange::Children(children_mixin) => Some(children_mixin),
        _ => None,
    })
}

/// Recursively processes a node in the dependency tree.
fn process_tree_node(
    leaf: &ChangedNodeBranch,
    svg_composition: &mut SVGCompositionRes,
    updates: &mut Vec<RenderUpdateEvent>,
) {
    // Process the current node entity
    process_node(leaf.entity, leaf.changed, svg_composition, updates);

    // Recursively process children, if any
    for child in &leaf.children {
        process_tree_node(child, svg_composition, updates);
    }
}

/// Processes a node entity by updating its corresponding SVG element/s
/// based on the provided changes.
fn process_node(
    entity: Entity,
    changed_node: &ChangedNode,
    svg_composition: &mut SVGCompositionRes,
    updates: &mut Vec<RenderUpdateEvent>,
) {
    // Attempt to get or create the node associated with the entity
    let maybe_node = svg_composition.get_or_create_node(
        entity,
        &changed_node.node_type,
        &changed_node.parent_id,
    );

    // Apply collected changes to the SVG node and drain updates
    if let Some(node) = maybe_node {
        node.apply_node_change(changed_node);
        updates.extend(node.drain_updates());
    }
}
