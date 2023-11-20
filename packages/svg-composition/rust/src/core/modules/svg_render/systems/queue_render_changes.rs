use std::{
    collections::{HashMap, HashSet},
    mem::take,
};

use bevy_ecs::{entity::Entity, system::ResMut};

use crate::core::{
    events::output_event::RenderUpdateEvent,
    modules::svg_render::resources::{
        changed_components::{ChangedComponents, ChangedNode, ChangedPaint},
        svg_composition::SVGComposition,
    },
};

pub fn queue_render_changes(
    mut changed: ResMut<ChangedComponents>,
    mut svg_composition: ResMut<SVGComposition>,
) {
    let changed_nodes = take(&mut changed.changed_nodes);
    let changed_paints = take(&mut changed.changed_paints);
    let mut updates: Vec<RenderUpdateEvent> = Vec::new();

    updates.extend(process_nodes(&changed_nodes, &mut svg_composition));
    updates.extend(process_paints(&changed_paints, &mut svg_composition));
    svg_composition.forward_render_updates(updates);
}

fn process_paints(
    changed_paints: &HashMap<Entity, ChangedPaint>,
    svg_composition: &mut SVGComposition,
) -> Vec<RenderUpdateEvent> {
    let mut updates: Vec<RenderUpdateEvent> = Vec::new();
    for (entity, paint) in changed_paints {
        updates.extend(process_paint(*entity, &paint, svg_composition));
    }
    return updates;
}

fn process_paint(
    entity: Entity,
    changed_paint: &ChangedPaint,
    svg_composition: &mut SVGComposition,
) -> Vec<RenderUpdateEvent> {
    let mut updates: Vec<RenderUpdateEvent> = Vec::new();

    // Attempt to get or create the paint associated with the entity
    let maybe_paint =
        svg_composition.get_or_create_paint(entity, &changed_paint.paint, &changed_paint.parent_id);

    // Apply collected changes to the SVG paint and drain updates
    if let Some(svg_paint) = maybe_paint {
        svg_paint.apply_paint_change(&changed_paint);
        updates.extend(svg_paint.drain_updates());
    }

    return updates;
}

fn process_nodes(
    changed_nodes: &HashMap<Entity, ChangedNode>,
    svg_composition: &mut SVGComposition,
) -> Vec<RenderUpdateEvent> {
    let mut processed: HashSet<Entity> = HashSet::new();
    let mut updates: Vec<RenderUpdateEvent> = Vec::new();

    // Iterate over and process each node
    for &entity in changed_nodes.keys() {
        updates.extend(process_with_parents(
            entity,
            &changed_nodes,
            &mut processed,
            svg_composition,
        ));
    }

    return updates;
}

/// Recursively process an entity and its parents
fn process_with_parents(
    entity: Entity,
    changed_nodes: &HashMap<Entity, ChangedNode>,
    processed: &mut HashSet<Entity>,
    svg_composition: &mut SVGComposition,
) -> Vec<RenderUpdateEvent> {
    let mut updates: Vec<RenderUpdateEvent> = Vec::new();

    if !processed.insert(entity) {
        return updates;
    }

    if let Some(change) = changed_nodes.get(&entity) {
        // Process parent first
        if let Some(parent_id) = change.parent_id {
            updates.extend(process_with_parents(
                parent_id,
                changed_nodes,
                processed,
                svg_composition,
            ));
        }

        // Process the current entity
        updates.extend(process_node(entity, change, svg_composition));
    }

    return updates;
}

/// Processes an entity by updating its corresponding SVG node based on the provided changes.
fn process_node(
    entity: Entity,
    changed_node: &ChangedNode,
    svg_composition: &mut SVGComposition,
) -> Vec<RenderUpdateEvent> {
    let mut updates: Vec<RenderUpdateEvent> = Vec::new();

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

    return updates;
}
