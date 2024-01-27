use std::{collections::HashMap, mem::take};

use bevy_ecs::{entity::Entity, system::ResMut};

use crate::resources::{
    changed_entities::{ChangedEntitiesRes, ChangedEntity},
    svg_composition::{svg_context::SVGContext, SVGCompositionRes},
};

pub fn queue_element_changes(
    mut changed_entities_res: ResMut<ChangedEntitiesRes>,
    mut svg_composition: ResMut<SVGCompositionRes>,
) {
    let changed_entities = take(&mut changed_entities_res.changed_entities);
    let dependency_tree = build_dependency_tree(changed_entities);
    // log::info!(
    //     "[queue_element_changes] Dependency Tree: {:#?}",
    //     dependency_tree
    // ); // TODO: REMOVE

    for root_branch in dependency_tree {
        process_tree_branch(root_branch, &mut svg_composition.context);
    }

    svg_composition.context.process_changed_entities();
}

#[derive(Debug)]
struct ChangedEntityBranch {
    entity: Entity,
    changed: ChangedEntity,
    children: Vec<ChangedEntityBranch>,
}

fn build_dependency_tree(
    mut changed_entities: HashMap<Entity, ChangedEntity>,
) -> Vec<ChangedEntityBranch> {
    let mut children_map: HashMap<Entity, Vec<Entity>> = HashMap::new();
    let mut roots: Vec<Entity> = Vec::new();

    // Identify root entities and prepare a map of children for each parent
    for (entity, changed_entity) in &changed_entities {
        if let Some(parent_id) = changed_entity.parent_id {
            if changed_entities.contains_key(&parent_id) {
                children_map
                    .entry(parent_id)
                    .or_insert_with(Vec::new)
                    .push(*entity);
            } else {
                roots.push(*entity);
            }
        } else {
            roots.push(*entity);
        }
    }

    // Build trees from the roots
    return roots
        .into_iter()
        .map(|root| build_tree(root, &mut changed_entities, &children_map))
        .collect();
}

fn build_tree(
    entity: Entity,
    changed_entities: &mut HashMap<Entity, ChangedEntity>,
    children_map: &HashMap<Entity, Vec<Entity>>,
) -> ChangedEntityBranch {
    let changed_entity = changed_entities
        .remove(&entity)
        .expect("Entity should exist");

    let children = children_map
        .get(&entity)
        .unwrap_or(&Vec::new())
        .iter()
        .map(|&child_entity| build_tree(child_entity, changed_entities, children_map))
        .collect();

    return ChangedEntityBranch {
        entity,
        changed: changed_entity,
        children,
    };
}

fn process_tree_branch(branch: ChangedEntityBranch, cx: &mut SVGContext) {
    process_entity(branch.entity, branch.changed, cx);

    // Recursively process children, if any
    for child in branch.children {
        process_tree_branch(child, cx);
    }
}

fn process_entity(entity: Entity, changed_entity: ChangedEntity, cx: &mut SVGContext) {
    if let Some(bundle) = cx.create_bundle(entity, changed_entity.entity_type) {
        cx.insert_bundle(bundle, changed_entity.parent_id);
        cx.add_changed_entity(changed_entity);
    }
}
