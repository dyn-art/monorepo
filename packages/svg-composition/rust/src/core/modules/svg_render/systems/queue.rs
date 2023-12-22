use std::{collections::HashMap, mem::take};

use bevy_ecs::{entity::Entity, system::ResMut};
use log::info;

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

/// A structure representing a node in the dependency tree.
#[derive(Debug)]
struct Leaf {
    entity: Entity,
    changed: ChangedNode,
    children: Option<Vec<Leaf>>,
}

fn build_dependency_trees(changed_nodes: &HashMap<Entity, ChangedNode>) -> Vec<Leaf> {
    let mut roots = Vec::new();

    // Identify roots
    for &entity in changed_nodes.keys() {
        if let Some(changed_node) = changed_nodes.get(&entity) {
            if changed_node.parent_id.is_none()
                || !changed_nodes.contains_key(&changed_node.parent_id.unwrap())
            {
                roots.push(build_leaf(entity, changed_nodes));
            }
        }
    }

    return roots;
}

fn build_leaf(entity: Entity, changed_nodes: &HashMap<Entity, ChangedNode>) -> Leaf {
    let changed_node = changed_nodes.get(&entity).unwrap();
    let mut children = Vec::new();

    // Build children leaves if they are in changed_nodes
    if let Some(children_mixin) = find_children_mixin(&changed_node.changes) {
        for &child_entity in &children_mixin.children.0 {
            if changed_nodes.contains_key(&child_entity) {
                children.push(build_leaf(child_entity, changed_nodes));
            }
        }
    } else {
        // Find and process direct children of the current node
        for (&child_entity, child_node) in changed_nodes {
            if child_node.parent_id == Some(entity) {
                children.push(build_leaf(child_entity, changed_nodes));
            }
        }
    }

    Leaf {
        entity,
        changed: changed_node.clone(), // TODO: avoid clone
        children: if children.is_empty() {
            None
        } else {
            Some(children)
        },
    }
}

fn find_children_mixin(changes: &[MixinChange]) -> Option<&MixinChangeChildrenMixin> {
    changes.iter().find_map(|change| {
        if let MixinChange::Children(children_mixin) = change {
            Some(children_mixin)
        } else {
            None
        }
    })
}

fn process_nodes(
    changed_nodes: &HashMap<Entity, ChangedNode>,
    svg_composition: &mut SVGCompositionRes,
    updates: &mut Vec<RenderUpdateEvent>,
) {
    let dependency_tree = build_dependency_trees(changed_nodes);

    // Traverse and process each node in the dependency tree
    for root in dependency_tree {
        process_tree_node(&root, svg_composition, updates);
    }
}

/// Recursively processes a node in the dependency tree.
fn process_tree_node(
    leaf: &Leaf,
    svg_composition: &mut SVGCompositionRes,
    updates: &mut Vec<RenderUpdateEvent>,
) {
    // Process the current node entity
    process_node(leaf.entity, &leaf.changed, svg_composition, updates);

    // Process child nodes, if any
    if let Some(children) = &leaf.children {
        for child in children {
            process_tree_node(child, svg_composition, updates);
        }
    }
}

/// Processes a node entity by updating its corresponding SVG element/s based on the provided changes.
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
        node.apply_node_change(&changed_node);
        updates.extend(node.drain_updates());
    }
}
