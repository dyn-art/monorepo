use std::collections::{HashMap, HashSet};

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
            // Attempt to get the parent entity id
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
    // Collect entities to process
    let to_process = initiate_entity_collection(&changed.changes);

    // Process each collected entity
    for entity in to_process {
        if let Some(changed_component) = changed.changes.get(&entity) {
            process_entity(entity, changed_component, &mut svg_composition);
        }
    }
}

fn initiate_entity_collection(changes: &HashMap<Entity, ChangedComponent>) -> Vec<Entity> {
    let mut to_process = HashSet::new();
    let mut processed = HashSet::new();

    for &entity in changes.keys() {
        collect_entities_iteratively(&mut to_process, &mut processed, changes, entity);
    }

    to_process.into_iter().collect()
}

fn collect_entities_iteratively(
    to_process: &mut HashSet<Entity>,
    processed: &mut HashSet<Entity>,
    changes: &HashMap<Entity, ChangedComponent>,
    start_entity: Entity,
) {
    let mut stack = vec![start_entity];

    while let Some(entity) = stack.pop() {
        // Skip already processed entities
        if !processed.insert(entity) {
            continue;
        }

        if let Some(changed) = changes.get(&entity) {
            if let Some(parent_id) = changed.parent_id {
                stack.push(parent_id);
            }

            to_process.insert(entity);
        }
    }
}

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
    }
    svg_composition.forward_node_updates(&entity);
}
