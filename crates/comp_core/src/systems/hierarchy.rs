use bevy_ecs::{
    entity::Entity,
    query::{Changed, With, Without},
    system::{Commands, Query},
};
use bevy_hierarchy::{Children, Parent};
use dyn_comp_bundles::components::{marker::Root, mixins::HierarchyLevel, nodes::CompNode};
use std::collections::HashSet;

pub fn add_root_component_system(
    mut commands: Commands,
    query: Query<
        Entity,
        (
            With<Children>,
            Without<Parent>,
            With<CompNode>,
            Without<Root>,
        ),
    >,
) {
    for entity in query.iter() {
        commands.entity(entity).insert(Root);
    }
}

pub fn remove_root_component_system(
    mut commands: Commands,
    query: Query<Entity, (With<Children>, With<Parent>, With<CompNode>, With<Root>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).remove::<Root>();
    }
}

pub fn update_hierarchy_levels(
    mut commands: Commands,
    changed_parents_query: Query<(Entity, &Parent), (With<CompNode>, Changed<Parent>)>,
    children_query: Query<&Children, With<CompNode>>,
    level_query: Query<&HierarchyLevel, With<CompNode>>,
    parent_query: Query<&Parent, With<CompNode>>,
) {
    let mut to_update: Vec<(Entity, HierarchyLevel)> = Vec::new();
    let mut updated_entities = HashSet::new();

    // Detect entities with changed parents and enqueue them for level updates
    for (entity, parent) in changed_parents_query.iter() {
        let parent_entity = parent.get();

        // Parent level is known
        if let Ok(level) = level_query.get(parent_entity) {
            to_update.push((entity, *level));
        }
        // Calculate the initial level if parent's level is unknown
        else {
            let level = calculate_level_for_entity(parent_entity, &parent_query);
            commands.entity(parent_entity).insert(HierarchyLevel(level));
            updated_entities.insert(parent_entity);
            to_update.push((entity, HierarchyLevel(level)));
        }
    }

    // Process each entity that needs its level updated
    while let Some((entity, parent_level)) = to_update.pop() {
        if !updated_entities.contains(&entity) {
            // Set the entity's level to one more than its parent's level
            let new_level = HierarchyLevel(parent_level.0 + 1);
            commands.entity(entity).insert(new_level);
            updated_entities.insert(entity);

            // If the entity has children, enqueue them for level updates
            if let Ok(children) = children_query.get(entity) {
                for &child in children.iter() {
                    to_update.push((child, new_level));
                }
            }
        }
    }
}

// Helper function to calculate an entity's level by traversing up the hierarchy
fn calculate_level_for_entity(entity: Entity, parent_query: &Query<&Parent, With<CompNode>>) -> u8 {
    let mut current_level = 0;
    let mut current_entity = Some(entity);

    while let Some(e) = current_entity {
        if let Ok(parent) = parent_query.get(e) {
            current_entity = Some(parent.get());
            current_level += 1;
        } else {
            break;
        }
    }

    return current_level;
}
