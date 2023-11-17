use std::{
    collections::{HashMap, HashSet},
    mem::take,
};

use bevy_ecs::{entity::Entity, system::ResMut};

use crate::core::modules::svg_render::resources::{
    changed_components::{ChangedComponent, ChangedComponents},
    svg_composition::SVGComposition,
};

pub fn queue_render_changes(
    mut changed: ResMut<ChangedComponents>,
    mut svg_composition: ResMut<SVGComposition>,
) {
    let changes = take(&mut changed.changes);
    let mut processed = HashSet::new();

    // Recursive function to process an entity and its parents
    fn process_with_parents(
        entity: Entity,
        changes: &HashMap<Entity, ChangedComponent>,
        processed: &mut HashSet<Entity>,
        svg_composition: &mut ResMut<SVGComposition>,
    ) {
        if !processed.insert(entity) {
            return;
        }

        if let Some(change) = changes.get(&entity) {
            // Process parent first
            if let Some(parent_id) = change.parent_id {
                process_with_parents(parent_id, changes, processed, svg_composition);
            }

            // Process the current entity
            process_entity(entity, change, svg_composition);
        }
    }

    // Iterate over and process each entity
    for &entity in changes.keys() {
        process_with_parents(entity, &changes, &mut processed, &mut svg_composition);
    }
}

/// Processes an entity by updating its corresponding SVG node based on the provided changes.
fn process_entity(
    entity: Entity,
    changed_component: &ChangedComponent,
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
        let updates = node.get_base_mut().drain_updates();
        svg_composition.forward_node_updates(updates);
    }
}
