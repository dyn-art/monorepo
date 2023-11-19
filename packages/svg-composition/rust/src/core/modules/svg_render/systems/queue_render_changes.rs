use std::{
    collections::{HashMap, HashSet},
    mem::take,
};

use bevy_ecs::{entity::Entity, system::ResMut};
use dyn_composition::core::modules::node::components::mixins::Paint;

use crate::core::modules::svg_render::resources::{
    changed_components::{ChangedComponents, ChangedNode},
    svg_composition::SVGComposition,
};

pub fn queue_render_changes(
    mut changed: ResMut<ChangedComponents>,
    mut svg_composition: ResMut<SVGComposition>,
) {
    let changed_nodes = take(&mut changed.changed_nodes);
    let changed_paints = take(&mut changed.changed_paints);

    process_paints(changed_paints, &mut svg_composition);
    process_nodes(changed_nodes, &mut svg_composition);
}

fn process_paints(changed_paints: HashMap<Entity, Paint>, svg_composition: &mut SVGComposition) {
    // Attempt to get or create the paint associated with the entity
}

fn process_nodes(
    changed_nodes: HashMap<Entity, ChangedNode>,
    svg_composition: &mut SVGComposition,
) {
    let mut processed = HashSet::new();

    // Recursive function to process an entity and its parents
    fn process_with_parents(
        entity: Entity,
        changed_nodes: &HashMap<Entity, ChangedNode>, // TODO: can I pass here changed node directly?
        processed: &mut HashSet<Entity>,
        svg_composition: &mut SVGComposition,
    ) {
        if !processed.insert(entity) {
            return;
        }

        if let Some(change) = changed_nodes.get(&entity) {
            // Process parent first
            if let Some(parent_id) = change.parent_id {
                process_with_parents(parent_id, changed_nodes, processed, svg_composition);
            }

            // Process the current entity
            process_entity(entity, change, svg_composition);
        }
    }

    // Iterate over and process each entity
    for &entity in changed_nodes.keys() {
        process_with_parents(entity, &changed_nodes, &mut processed, svg_composition);
    }
}

/// Processes an entity by updating its corresponding SVG node based on the provided changes.
fn process_entity(
    entity: Entity,
    changed_component: &ChangedNode,
    svg_composition: &mut SVGComposition,
) {
    // Attempt to get or create the node associated with the entity
    let maybe_node = svg_composition.get_or_create_node(
        entity,
        &changed_component.node_type,
        &changed_component.parent_id,
    );

    if let Some(node) = maybe_node {
        // Apply collected changes to the SVG node
        node.apply_mixin_changes(&changed_component.changes);

        // Drain and forward render updates from the node
        let updates = node.drain_updates();
        svg_composition.forward_node_updates(updates);
    }
}
