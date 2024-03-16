use std::{collections::HashMap, mem::take};

use bevy_ecs::{entity::Entity, system::ResMut};

use crate::{
    mixin_change::{MixinChange, MixinChangeChildrenMixin},
    resources::{
        changed_entities::{ChangedEntitiesRes, ChangedEntity},
        svg_composition::{svg_context::SVGContext, SVGCompositionRes},
    },
};

pub fn queue_element_changes(
    mut changed_entities_res: ResMut<ChangedEntitiesRes>,
    mut svg_composition: ResMut<SVGCompositionRes>,
) {
    let changed_entities = take(&mut changed_entities_res.changed_entities);

    // TODO: Improve - Could we handle the order when ChildrenMixin is applied? Do we really need to preorder here?
    #[cfg(feature = "output-event")]
    {
        let dependency_trees = build_dependency_trees(changed_entities);
        for dependency_tree in dependency_trees {
            process_dependency_tree(dependency_tree, &mut svg_composition.context);
        }
    }
    #[cfg(not(feature = "output-event"))]
    {
        for (entity, changed_entity) in changed_entities {
            process_entity(entity, changed_entity, &mut svg_composition.context)
        }
    }

    svg_composition.context.process_changed_entities();
}

#[derive(Debug)]
struct ChangedEntityBranch {
    entity: Entity,
    changed: ChangedEntity,
    children: Vec<ChangedEntityBranch>,
}

/// Builds dependency trees from changed entities.
fn build_dependency_trees(
    mut changed_entities: HashMap<Entity, ChangedEntity>,
) -> Vec<ChangedEntityBranch> {
    let mut children_map: HashMap<Entity, Vec<Entity>> = HashMap::new();
    let mut roots: Vec<Entity> = Vec::new();

    // Identify root entities and prepare a map of children for each parent
    for (entity, changed_entity) in &changed_entities {
        if let Some(parent_id) = changed_entity.parent_id {
            if changed_entities.contains_key(&parent_id) {
                children_map.entry(parent_id).or_default().push(*entity);
            } else {
                roots.push(*entity);
            }
        } else {
            roots.push(*entity);
        }
    }

    // Build dependency trees from root entities
    return roots
        .into_iter()
        .map(|root| build_dependency_tree(root, &mut changed_entities, &children_map))
        .collect();
}

/// Builds a dependency tree for a given entity and its children.
fn build_dependency_tree(
    entity: Entity,
    changed_entities: &mut HashMap<Entity, ChangedEntity>,
    children_map: &HashMap<Entity, Vec<Entity>>,
) -> ChangedEntityBranch {
    let changed_entity = changed_entities
        .remove(&entity)
        .expect("Entity should exist");

    let children = find_children_mixin(&changed_entity)
        // If children_mixin is present, sort children accordingly
        .map(|children_mixin| {
            children_mixin
                .children
                .0
                .iter()
                .filter_map(|&child_entity| {
                    if changed_entities.contains_key(&child_entity) {
                        Some(build_dependency_tree(
                            child_entity,
                            changed_entities,
                            children_map,
                        ))
                    } else {
                        None
                    }
                })
                .collect()
        })
        // Otherwise, use the children from the children_map (order not relevant)
        .unwrap_or_else(|| {
            children_map
                .get(&entity)
                .unwrap_or(&Vec::new())
                .iter()
                .map(|&child_entity| {
                    build_dependency_tree(child_entity, changed_entities, children_map)
                })
                .collect()
        });

    return ChangedEntityBranch {
        entity,
        changed: changed_entity,
        children,
    };
}

/// Finds and returns the children mixin change, if any, from a changed entity.
fn find_children_mixin(changed_entity: &ChangedEntity) -> Option<&MixinChangeChildrenMixin> {
    changed_entity
        .changes
        .iter()
        .find_map(|change| match change {
            MixinChange::Children(children_mixin) => Some(children_mixin),
            _ => None,
        })
}

fn process_dependency_tree(tree: ChangedEntityBranch, cx: &mut SVGContext) {
    process_entity(tree.entity, tree.changed, cx);

    // Recursively process children, if any
    for child in tree.children {
        process_dependency_tree(child, cx);
    }
}

fn process_entity(entity: Entity, changed_entity: ChangedEntity, cx: &mut SVGContext) {
    if let Some(bundle) = cx.create_bundle(entity, changed_entity.entity_type) {
        cx.insert_bundle(bundle, changed_entity.parent_id);
        cx.add_changed_entity(changed_entity);
    }
}
