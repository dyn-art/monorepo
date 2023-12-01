use std::{
    collections::{HashMap, HashSet},
    mem::take,
};

use bevy_ecs::{entity::Entity, system::ResMut};

use crate::core::{
    events::output_event::RenderUpdateEvent,
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

fn process_nodes(
    changed_nodes: &HashMap<Entity, ChangedNode>,
    svg_composition: &mut SVGCompositionRes,
    updates: &mut Vec<RenderUpdateEvent>,
) {
    let mut processed: HashSet<Entity> = HashSet::new();
    for &entity in changed_nodes.keys() {
        process_with_parents(
            entity,
            &changed_nodes,
            &mut processed,
            svg_composition,
            updates,
        );
    }
}

/// Recursively process a node entity and its parents.
fn process_with_parents(
    entity: Entity,
    changed_nodes: &HashMap<Entity, ChangedNode>,
    processed: &mut HashSet<Entity>,
    svg_composition: &mut SVGCompositionRes,
    updates: &mut Vec<RenderUpdateEvent>,
) {
    if !processed.insert(entity) {
        return;
    }

    if let Some(change) = changed_nodes.get(&entity) {
        // Process parent first
        if let Some(parent_id) = change.parent_id {
            process_with_parents(
                parent_id,
                changed_nodes,
                processed,
                svg_composition,
                updates,
            );
        }

        // Process the current node entity
        process_node(entity, change, svg_composition, updates);
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
