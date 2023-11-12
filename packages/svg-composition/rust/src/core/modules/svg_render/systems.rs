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
use log::info;

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
    info!(
        "Called queue_render_changes with changes: {:?}",
        changed.changes
    );

    // Sort entities
    let mut to_process = Vec::new();
    let mut processed = HashSet::new();
    collect_entities(&mut to_process, &mut processed, &changed.changes, None);

    // Process entities
    for entity in to_process {
        if let Some(changed_component) = changed.changes.get(&entity) {
            process_entity(entity, changed_component, &mut svg_composition);
        }
    }

    info!(
        "End queue_render_changes with svg_composition: {:?}",
        svg_composition
    );
}

fn collect_entities(
    to_process: &mut Vec<Entity>,
    processed: &mut HashSet<Entity>,
    changes: &HashMap<Entity, ChangedComponent>,
    current_entity: Option<Entity>,
) {
    if let Some(entity) = current_entity {
        // Avoid re-processing entities
        if processed.contains(&entity) {
            return;
        }

        if let Some(changed) = changes.get(&entity) {
            if let Some(parent_id) = changed.parent_id {
                collect_entities(to_process, processed, changes, Some(parent_id));
            }

            to_process.push(entity);
            processed.insert(entity);
        }
    }
    // Initial call to function for all entities
    else {
        for &entity in changes.keys() {
            collect_entities(to_process, processed, changes, Some(entity));
        }
    }
}

fn process_entity(
    entity: Entity,
    changed_component: &ChangedComponent,
    svg_composition: &mut SVGComposition,
) {
    info!(
        "Called process_entity for {:?} with changes {:?} and in SvgComposition: {:?}",
        entity, changed_component, svg_composition
    ); // TODO: REMOVE
       // TODO
}
