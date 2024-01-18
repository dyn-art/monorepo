use std::{collections::HashMap, mem::take};

use bevy_ecs::{entity::Entity, system::ResMut};
use dyn_composition::core::modules::node::components::types::NodeType;

use crate::{
    events::output_event::ElementChangeEvent,
    mixin_change::{MixinChange, MixinChangeChildrenMixin},
    resources::{
        changed_components::{ChangedComponentsRes, ChangedNode, ChangedPaint},
        svg_composition::SVGCompositionRes,
    },
};

pub fn queue_element_changes(
    mut changed: ResMut<ChangedComponentsRes>,
    mut svg_composition: ResMut<SVGCompositionRes>,
) {
    let changed_nodes = take(&mut changed.changed_nodes);
    let changed_paints = take(&mut changed.changed_paints);
    let mut element_change_events: Vec<ElementChangeEvent> = Vec::new();

    // Process nodes & paints and collect render changes emitted during this process
    process_nodes(
        &changed_nodes,
        &mut svg_composition,
        &mut element_change_events,
    );
    process_paints(
        &changed_paints,
        &mut svg_composition,
        &mut element_change_events,
    );

    // Forward render changes into output event channel
    for event in element_change_events {
        svg_composition.forward_element_changes(event.id, event.changes);
    }
}

// =============================================================================
// Paint
// =============================================================================

fn process_paints(
    changed_paints: &HashMap<Entity, ChangedPaint>,
    svg_composition: &mut SVGCompositionRes,
    element_change_events: &mut Vec<ElementChangeEvent>,
) {
    for (entity, paint) in changed_paints {
        process_paint(*entity, &paint, svg_composition, element_change_events);
    }
}

/// Processes a paint entity by updating its corresponding SVG element/s based on the provided changes.
fn process_paint(
    entity: Entity,
    changed_paint: &ChangedPaint,
    svg_composition: &mut SVGCompositionRes,
    element_change_events: &mut Vec<ElementChangeEvent>,
) {
    // Attempt to get or create the paint associated with the entity
    let maybe_paint =
        svg_composition.get_or_create_paint(entity, &changed_paint.paint, &changed_paint.parent_id);

    // Apply collected changes to the SVG paint and drain changes
    if let Some(svg_paint) = maybe_paint {
        svg_paint.apply_paint_change(&changed_paint);
        element_change_events.extend(svg_paint.drain_changes());
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
    element_change_events: &mut Vec<ElementChangeEvent>,
) {
    // TODO: Performance improvement
    let dependency_tree = build_dependency_trees(changed_nodes);

    // Traverse and process each root node and its descendants
    for root in dependency_tree {
        process_tree_node(&root, svg_composition, element_change_events);
    }
}

/// Builds dependency trees from the changed nodes.
fn build_dependency_trees<'a>(
    changed_nodes: &'a HashMap<Entity, ChangedNode>,
) -> Vec<ChangedNodeBranch<'a>> {
    let mut children_map = HashMap::new();

    // Preparing a map of children for each parent (for quick lookup)
    for (&entity, changed_node) in changed_nodes {
        if let Some(parent_id) = changed_node.parent_id {
            children_map
                .entry(parent_id)
                .or_insert_with(Vec::new)
                .push(entity);
        }
    }

    // Identify root nodes (those without parents or whose parents are not in changed_nodes)
    return changed_nodes
        .iter()
        .filter_map(|(&entity, changed_node)| {
            (changed_node.parent_id.is_none()
                || !changed_nodes.contains_key(&changed_node.parent_id.unwrap()))
            .then(|| build_branch(entity, changed_nodes, &children_map))
        })
        .collect();
}

/// Recursively builds a branch in the dependency tree.
fn build_branch<'a>(
    entity: Entity,
    changed_nodes: &'a HashMap<Entity, ChangedNode>,
    children_map: &HashMap<Entity, Vec<Entity>>,
) -> ChangedNodeBranch<'a> {
    let changed_node = changed_nodes
        .get(&entity)
        .unwrap_or_else(|| panic!("Node must exist for entity {}", entity.to_bits()));

    // Build children branches
    let children =
        if changed_node.node_type == NodeType::Frame || changed_node.node_type == NodeType::Group {
            // If children_mixin is present, sort children accordingly
            if let Some(children_mixin) = find_children_mixin(&changed_node.changes) {
                children_mixin
                    .children
                    .0
                    .iter()
                    .filter_map(|&child_entity| {
                        changed_nodes
                            .get(&child_entity)
                            .map(|_| build_branch(child_entity, changed_nodes, children_map))
                    })
                    .collect()
            } else {
                // Otherwise, use the children from the children_map (order not relevant)
                children_map
                    .get(&entity)
                    .unwrap_or(&Vec::new())
                    .iter()
                    .map(|&child_entity| build_branch(child_entity, changed_nodes, children_map))
                    .collect()
            }
        } else {
            Vec::new()
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
    element_change_events: &mut Vec<ElementChangeEvent>,
) {
    // Process the current node entity
    process_node(
        leaf.entity,
        leaf.changed,
        svg_composition,
        element_change_events,
    );

    // Recursively process children, if any
    for child in &leaf.children {
        process_tree_node(child, svg_composition, element_change_events);
    }
}

/// Processes a node entity by updating its corresponding SVG element/s
/// based on the provided changes.
fn process_node(
    entity: Entity,
    changed_node: &ChangedNode,
    svg_composition: &mut SVGCompositionRes,
    element_change_events: &mut Vec<ElementChangeEvent>,
) {
    // Attempt to get or create the node associated with the entity
    let maybe_node = svg_composition.get_or_create_node(
        entity,
        &changed_node.node_type,
        &changed_node.parent_id,
    );

    // Apply collected changes to the SVG node and drain changes
    if let Some(node) = maybe_node {
        node.apply_node_change(changed_node);
        element_change_events.extend(node.drain_changes());
    }
}
