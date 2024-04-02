use crate::{
    components::{Locked, Preselected, Selected},
    events::CursorDownOnEntityInputEvent,
    resources::comp_interaction::{CompInteractionRes, InteractionMode, MouseButton},
};
use bevy_ecs::{
    entity::Entity,
    event::EventReader,
    query::{With, Without},
    system::{Commands, Query, ResMut},
};
use bevy_hierarchy::Parent;
use dyn_comp_bundles::components::{mixins::Root, nodes::CompNode};
use glam::Vec2;

static DOUBLE_CLICK_WINDOW: web_time::Duration = web_time::Duration::from_millis(500);

// Selection Rules for Elements:
// 1. Avoid selecting the top-level (root) frame directly; focus on its children for selection.
//    This ensures actions are targeted towards elements within a frame, not the frame itself.
// 2. In cases of nested frames, select the nested frame within the top-level frame,
//    bypassing its children. This aligns with the principle of managing frames as whole units
//    at the top level before diving into their contents (with the exception of root frames).
// 3. For interacting with elements inside nested frames, a double-click on the desired item is required.
// 4. When selecting items within nested frames, avoid propagating the selection to parent frames.
//    This means selection stays on the most deeply nested item clicked, preserving the intent of the action.
// 5. Locked or invisible nodes should never be selected.
// 6. Selection should respect grouping hierarchies. If an item is part of a group,
//    selecting the item should imply selection of the whole group, unless specifically
//    targeting an item within the group through direct interaction (e.g., double-click), similar to rule 2.
// 7. Prioritize selectable items based on cursor proximity and the z-index.
//    In dense areas with overlapping elements, the item closest to the cursor and highest on the z-index
//    should be selected first, with an option to cycle through overlapping items via repeated clicks
//    or modifier key combinations.

// Events received if clicked on nested Rectangle:
// INFO: [handle_cursor_down_on_entity_event] Start
// INFO: [handle_cursor_down_on_entity_event] Entity: 10v1 <- Rectangle (Clicked on)
// INFO: [handle_cursor_down_on_entity_event] Entity: 8v1 <- Frame Nested
// INFO: [handle_cursor_down_on_entity_event] Entity:6v1 <- Frame Nested
// INFO: [handle_cursor_down_on_entity_event] Entity: 4v1 <- Frame (Root)
// INFO: [handle_cursor_down_on_entity_event] End
pub fn handle_cursor_down_on_entity_event(
    mut commands: Commands,
    mut event_reader: EventReader<CursorDownOnEntityInputEvent>,
    mut comp_interaction_res: ResMut<CompInteractionRes>,
    unselected_node_query: Query<
        &Parent,
        (
            With<CompNode>,
            Without<Root>,
            Without<Selected>,
            Without<Locked>,
        ),
    >,
    preselected_node_query: Query<
        &Preselected,
        (With<CompNode>, With<Preselected>, Without<Locked>),
    >,
    selected_node_query: Query<(Entity, &Selected), (With<CompNode>, With<Selected>)>,
    root_node_query: Query<Entity, (With<CompNode>, With<Root>)>,
) {
    let raycast_entities: Vec<(Entity, Vec2)> = event_reader
        .read()
        .filter_map(|event| {
            if event.button == MouseButton::Left {
                Some((event.entity, event.position))
            } else {
                None
            }
        })
        .collect();

    if raycast_entities.is_empty() {
        return;
    }

    let now = web_time::Instant::now();

    // Preselection of nodes that could be selected or preselected
    // (entity, position, preselect)
    let mut preselected_nodes: Vec<PreselectedNode> = Vec::new();

    // Preselect nodes based on raycasted entities
    for (entity, cursor_position) in raycast_entities.iter().copied() {
        log::info!("[handle_cursor_down_on_entity_event] Entity: {:?}", entity);

        if let Ok(parent) = unselected_node_query.get(entity) {
            // Consider selecting preselected node
            if let Ok(Preselected { timestamp }) = preselected_node_query.get(entity) {
                if now.duration_since(*timestamp) <= DOUBLE_CLICK_WINDOW {
                    preselected_nodes.push(PreselectedNode {
                        entity,
                        cursor_position,
                        preselect: false,
                        was_selected: false,
                        was_preselected: true,
                    });
                    continue;
                } else {
                    commands.entity(entity).remove::<Preselected>();
                }
            }

            let parent_entity = parent.get();
            let is_parent_root = root_node_query.get(parent_entity).is_ok();

            // Consider selection node whose parent is the root
            if is_parent_root {
                preselected_nodes.push(PreselectedNode {
                    entity,
                    cursor_position,
                    preselect: false,
                    was_selected: false,
                    was_preselected: false,
                });
                continue;
            }

            // Consider preselecting node whose parent is selected
            if let Ok((_, Selected { timestamp })) = selected_node_query.get(parent_entity) {
                preselected_nodes.push(PreselectedNode {
                    entity,
                    cursor_position,
                    preselect: now.duration_since(*timestamp) > DOUBLE_CLICK_WINDOW,
                    was_selected: false,
                    was_preselected: false,
                });
                continue;
            }
        }

        // Consider selecting already selected node
        if selected_node_query.get(entity).is_ok() {
            preselected_nodes.push(PreselectedNode {
                entity,
                cursor_position,
                preselect: false,
                was_selected: true,
                was_preselected: false,
            });
        }
    }

    log::info!(
        "[handle_cursor_down_on_entity_event] Preselection: {:?}",
        preselected_nodes
    );

    let mut selected_node: Option<Entity> = None;
    let mut unselect_prev_selected = false;

    // Select or preselect preselected node
    if let Some(PreselectedNode {
        entity,
        cursor_position,
        preselect,
        was_selected,
        was_preselected,
    }) = preselected_nodes.first().copied()
    {
        if preselect {
            commands
                .entity(entity)
                .insert(Preselected { timestamp: now });
        } else {
            if !was_selected {
                let mut entiy_commands = commands.entity(entity);
                entiy_commands.insert(Selected { timestamp: now });
                if was_preselected {
                    entiy_commands.remove::<Preselected>();
                }

                #[cfg(feature = "tracing")]
                log::info!(
                    "[handle_cursor_down_on_entity_event] Selected Entity {:?} at {:?}",
                    entity,
                    position
                );
            }

            selected_node = Some(entity);
            unselect_prev_selected = true;

            comp_interaction_res.interaction_mode = InteractionMode::Translating {
                origin: cursor_position,
                current: cursor_position,
            };
        }
    } else {
        unselect_prev_selected = true;
    }

    // Unselect previously selected nodes that are no longer selected
    if unselect_prev_selected {
        for (entity, _) in selected_node_query.iter() {
            if selected_node.map_or(true, |e| e != entity) {
                commands.entity(entity).remove::<Selected>();
                #[cfg(feature = "tracing")]
                log::info!(
                    "[handle_cursor_down_on_entity_event] Unselected Entity: {:?}",
                    entity
                );
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct PreselectedNode {
    entity: Entity,
    cursor_position: Vec2,
    preselect: bool,
    was_preselected: bool,
    was_selected: bool,
}
