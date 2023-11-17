use std::{
    collections::{HashMap, HashSet},
    mem::take,
};

use bevy_ecs::{
    component::Component,
    entity::Entity,
    query::{Changed, With},
    system::{Query, ResMut},
};
use bevy_hierarchy::Parent;
use dyn_bevy_render_skeleton::extract_param::Extract;
use dyn_composition::core::modules::node::components::types::Node;

use super::{
    mixin_change::ToMixinChange,
    resources::{
        changed_components::{ChangedComponent, ChangedComponents},
        svg_composition::svg_composition::SVGComposition,
    },
};

pub fn extract_mixin_generic<T: Component + ToMixinChange>(
    mut changed: ResMut<ChangedComponents>,
    query: Extract<Query<(Entity, &Node, &T), (With<Node>, Changed<T>)>>,
    parent_query: Extract<Query<&Parent>>,
) {
    query.for_each(|(entity, node, mixin)| {
        let changed_component = changed.changes.entry(entity).or_insert_with(|| {
            // Try to get the parent entity id
            let mut parent_id: Option<Entity> = None;
            if let Ok(parent) = parent_query.get(entity) {
                parent_id = Some(parent.get());
            }

            return ChangedComponent {
                node_type: node.node_type.clone(),
                changes: vec![],
                parent_id,
            };
        });
        changed_component.changes.push(mixin.to_mixin_change());
    });
}

pub fn queue_render_changes(
    mut changed: ResMut<ChangedComponents>,
    mut svg_composition: ResMut<SVGComposition>,
) {
    let changes = take(&mut changed.changes);
    let mut processed = HashSet::new();

    // Recursive function to process an entity and its parents
    // TODO: Iterative approach bad? Could we run into stackoverflow?
    fn process_with_parents(
        entity: Entity,
        changes: &HashMap<Entity, ChangedComponent>,
        processed: &mut HashSet<Entity>,
        svg_composition: &mut ResMut<SVGComposition>,
    ) {
        if processed.insert(entity) {
            if let Some(change) = changes.get(&entity) {
                if let Some(parent_id) = change.parent_id {
                    // Process parent first
                    process_with_parents(parent_id, changes, processed, svg_composition);
                }
                // Process the current entity
                process_entity(entity, change, svg_composition);
            }
        }
    }

    // Iterate over changes and process each entity
    for &entity in changes.keys() {
        process_with_parents(entity, &changes, &mut processed, &mut svg_composition);
    }
}

// Non iterative approach:
//
// pub fn queue_render_changes(
//     mut changed: ResMut<ChangedComponents>,
//     mut svg_composition: ResMut<SVGComposition>,
// ) {
//     let mut changes = std::mem::take(&mut changed.changes);
//     let mut processed = HashSet::new();
//     let mut to_process = VecDeque::new();

//     // Initialize the processing queue with all entities
//     to_process.extend(changes.keys().cloned());

//     while let Some(entity) = to_process.pop_front() {
//         if processed.insert(entity) {
//             if let Some(change) = changes.remove(&entity) {
//                 process_entity(entity, &change, &mut svg_composition);

//                 // Add parent for processing if it exists and hasn't been processed yet
//                 if let Some(parent_id) = change.parent_id {
//                     if !processed.contains(&parent_id) {
//                         to_process.push_back(parent_id);
//                     }
//                 }
//             }
//         }
//     }
// }

fn process_entity(
    entity: Entity,
    changed_component: &ChangedComponent,
    svg_composition: &mut SVGComposition,
) {
    let maybe_node = svg_composition.get_or_insert_node(
        entity,
        &changed_component.node_type,
        &changed_component.parent_id,
    );
    if let Some(node) = maybe_node {
        node.apply_mixin_changes(&changed_component.changes);
        let updates = node.get_base_mut().drain_updates();
        svg_composition.forward_node_updates(updates);
    }
}
