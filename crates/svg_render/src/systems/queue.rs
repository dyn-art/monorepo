use std::{collections::HashMap, mem::take};

use bevy_ecs::{entity::Entity, system::ResMut};

use crate::{
    events::output_event::ElementChangeEvent,
    mixin_change::{MixinChangeChildrenMixin, NodeMixinChange},
    resources::{
        changed_components::{ChangedComponentsRes, ChangedEntity, ChangedNode, ChangedPaint},
        svg_composition::SVGCompositionRes,
    },
};

pub fn queue_element_changes(
    mut changed: ResMut<ChangedComponentsRes>,
    mut svg_composition: ResMut<SVGCompositionRes>,
) {
    let changed_entities = take(&mut changed.changed_entities);
    let mut element_change_events: Vec<ElementChangeEvent> = Vec::new();

    // Process nodes & paints and collect render changes emitted during this process
    process_changed(
        &changed_entities,
        &mut svg_composition,
        &mut element_change_events,
    );

    // Forward render changes into output event channel
    for event in element_change_events {
        svg_composition.forward_element_changes(event.id, event.changes);
    }
}

#[derive(Debug)]
struct ChangedEntityBranch<'a> {
    entity: Entity,
    changed: &'a ChangedEntity,
    children: Vec<ChangedEntityBranch<'a>>,
}

/// Processes changed entities by building and traversing dependency trees
/// to ensure that parents are processed before its children.
fn process_changed(
    changed_entities: &HashMap<Entity, ChangedEntity>,
    svg_composition: &mut SVGCompositionRes,
    element_change_events: &mut Vec<ElementChangeEvent>,
) {
    // TODO: Performance improvement
    let dependency_tree = build_dependency_trees(changed_entities);

    // Traverse and process each root entity and its descendants
    for root in dependency_tree {
        process_tree_node(&root, svg_composition, element_change_events);
    }
}

/// Builds dependency trees from the changed entities.
fn build_dependency_trees<'a>(
    changed_entities: &'a HashMap<Entity, ChangedEntity>,
) -> Vec<ChangedEntityBranch<'a>> {
    let mut children_map = HashMap::new();

    // Preparing a map of children for each parent (for quick lookup)
    for (&entity, changed_entity) in changed_entities {
        let maybe_parent_id = match changed_entity {
            ChangedEntity::Node(changed_node) => changed_node.parent_id,
            ChangedEntity::Paint(changed_paint) => changed_paint.parent_id,
        };

        if let Some(parent_id) = maybe_parent_id {
            children_map
                .entry(parent_id)
                .or_insert_with(Vec::new)
                .push(entity);
        }
    }

    // Identify root entities (those without parents or whose parents are not in "changed_entities")
    return changed_entities
        .iter()
        .filter_map(|(&entity, changed_entity)| {
            let maybe_parent_id = match changed_entity {
                ChangedEntity::Node(changed_node) => changed_node.parent_id,
                ChangedEntity::Paint(changed_paint) => changed_paint.parent_id,
            };

            (maybe_parent_id.is_none() || !changed_entities.contains_key(&maybe_parent_id.unwrap()))
                .then(|| build_branch(entity, changed_entities, &children_map))
        })
        .collect();
}

/// Recursively builds a branch in the dependency tree.
fn build_branch<'a>(
    entity: Entity,
    changed_entities: &'a HashMap<Entity, ChangedEntity>,
    children_map: &HashMap<Entity, Vec<Entity>>,
) -> ChangedEntityBranch<'a> {
    let changed_entity = changed_entities
        .get(&entity)
        .unwrap_or_else(|| panic!("Changes must exist for entity {}", entity.to_bits()));

    // Build children branches
    let children =
            // If children_mixin is present, sort children accordingly
            if let Some(children_mixin) = find_children_mixin(&changed_entity) {
                children_mixin
                    .children
                    .0
                    .iter()
                    .filter_map(|&child_entity| {
                        changed_entities
                            .get(&child_entity)
                            .map(|_| build_branch(child_entity, changed_entities, children_map))
                    })
                    .collect()
            } else {
                // Otherwise, use the children from the children_map (order not relevant)
                children_map
                    .get(&entity)
                    .unwrap_or(&Vec::new())
                    .iter()
                    .map(|&child_entity| build_branch(child_entity, changed_entities, children_map))
                    .collect()
            };

    return ChangedEntityBranch {
        entity,
        changed: changed_entity,
        children,
    };
}

/// Finds children mixin change from a changed entity.
fn find_children_mixin(changed_entity: &ChangedEntity) -> Option<&MixinChangeChildrenMixin> {
    let changes = match changed_entity {
        ChangedEntity::Node(changed_node) => {
            changed_node.changes.iter().find_map(|change| match change {
                NodeMixinChange::Children(children_mixin) => Some(children_mixin),
                _ => None,
            })
        }
        ChangedEntity::Paint(..) => None, // Paint can't have any children
    };
    changes
}

/// Recursively processes a changed entity in the dependency tree.
fn process_tree_node(
    leaf: &ChangedEntityBranch,
    svg_composition: &mut SVGCompositionRes,
    element_change_events: &mut Vec<ElementChangeEvent>,
) {
    // Process the current entity
    match leaf.changed {
        ChangedEntity::Node(changed_node) => {
            process_node(
                leaf.entity,
                changed_node,
                svg_composition,
                element_change_events,
            );
        }
        ChangedEntity::Paint(changed_paint) => {
            process_paint(
                leaf.entity,
                changed_paint,
                svg_composition,
                element_change_events,
            );
        }
    }

    // Recursively process children, if any
    for child in &leaf.children {
        process_tree_node(child, svg_composition, element_change_events);
    }
}

/// Processes a node entity by updating its corresponding SVG element/s
/// based on the provided changes.
fn process_node(
    entity: Entity,
    changed_node: &ChangedNode,
    svg_composition: &mut SVGCompositionRes,
    element_change_events: &mut Vec<ElementChangeEvent>,
) {
    // Attempt to get or create the node associated with the entity
    let maybe_node = svg_composition.get_or_create_node(
        entity,
        &changed_node.node_type,
        &changed_node.parent_id,
    );

    // Apply collected changes to the SVG node and drain changes
    if let Some((node, id_generator)) = maybe_node {
        node.apply_node_change(changed_node, id_generator);
        element_change_events.extend(node.drain_changes());
    }
}

/// Processes a paint entity by updating its corresponding SVG element/s based on the provided changes.
fn process_paint(
    entity: Entity,
    changed_paint: &ChangedPaint,
    svg_composition: &mut SVGCompositionRes,
    element_change_events: &mut Vec<ElementChangeEvent>,
) {
    // Attempt to get or create the paint associated with the entity
    let maybe_paint = svg_composition.get_or_create_paint(
        entity,
        &changed_paint.paint_type,
        &changed_paint.changes,
        &changed_paint.parent_id,
    );

    // Apply collected changes to the SVG paint and drain changes
    if let Some((svg_paint, id_generator)) = maybe_paint {
        svg_paint.apply_paint_change(&changed_paint, id_generator);
        element_change_events.extend(svg_paint.drain_changes());
    }
}
