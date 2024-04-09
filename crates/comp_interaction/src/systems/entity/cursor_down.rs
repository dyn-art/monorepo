use crate::{
    components::{Locked, Preselected, Selected},
    events::CursorDownOnEntityInputEvent,
    input::mouse::{
        MouseButton, MouseButtonOnEntity, MouseButtonOnEntityButtonInputRes, MouseButtonValue,
    },
    resources::comp_interaction::{CompInteractionRes, InteractionMode, InteractionTool},
};
use bevy_ecs::{
    change_detection::DetectChangesMut,
    entity::Entity,
    event::EventReader,
    query::{With, Without},
    system::{Commands, Query, Res, ResMut},
};
use bevy_hierarchy::Parent;
use dyn_comp_bundles::components::{
    mixins::{HierarchyLevel, Root},
    nodes::CompNode,
};
use glam::Vec2;
use std::collections::HashSet;

// https://stackoverflow.com/questions/29917287/what-is-the-max-delay-between-two-clicks-to-trigger-a-double-click-event
static DOUBLE_CLICK_WINDOW: web_time::Duration = web_time::Duration::from_millis(500);

pub fn cursor_down_on_entity_input_system(
    mut event_reader: EventReader<CursorDownOnEntityInputEvent>,
    mut mouse_button_input_res: ResMut<MouseButtonOnEntityButtonInputRes>,
) {
    mouse_button_input_res.bypass_change_detection().clear();
    for event in event_reader.read() {
        log::info!(
            "[cursor_down_on_entity_input_system] {:?} on {:?}",
            event.button,
            event.entity
        );
        mouse_button_input_res.press(
            MouseButtonOnEntity {
                entity: event.entity,
                button: event.button,
            },
            MouseButtonValue {
                position: event.position,
            },
        );
    }
}

// Events received if clicked on nested Rectangle:
// INFO: [cursor_down_on_entity_system] Start
// INFO: [cursor_down_on_entity_system] Entity: 10v1 <- Rectangle (Clicked on)
// INFO: [cursor_down_on_entity_system] Entity: 8v1 <- Frame Nested
// INFO: [cursor_down_on_entity_system] Entity:6v1 <- Frame Nested
// INFO: [cursor_down_on_entity_system] Entity: 4v1 <- Frame (Root)
// INFO: [cursor_down_on_entity_system] End
//
// Becomes in just_pressed:
// [(4v1, Vec2(284.48438, 285.9297)),
// (6v1, Vec2(284.48438, 285.9297)),
// (8v1, Vec2(284.48438, 285.9297)),
// (10v1, Vec2(284.48438, 285.9297))]
pub fn cursor_down_on_entity_system(
    mut commands: Commands,
    mut comp_interaction_res: ResMut<CompInteractionRes>,
    mouse_button_input_res: Res<MouseButtonOnEntityButtonInputRes>,
    unselected_node_query: Query<
        (Option<&Parent>, Option<&HierarchyLevel>),
        (
            With<CompNode>,
            Without<Root>, // TODO: Only entities Without<Root> and Without<FrameCompNode> (so root frames) should be excluded not all root or frame nodes
            Without<Selected>,
            Without<Locked>,
        ),
    >,
    preselected_node_query: Query<
        &Preselected,
        (With<CompNode>, With<Preselected>, Without<Locked>),
    >,
    selected_node_query: Query<
        (Entity, Option<&Parent>, Option<&HierarchyLevel>),
        (With<CompNode>, With<Selected>),
    >,
    root_node_query: Query<Entity, (With<CompNode>, With<Root>)>,
) {
    match comp_interaction_res.interaction_tool {
        InteractionTool::Select => {}
        _ => return,
    };

    let raycast_entities: Vec<(Entity, Vec2)> = mouse_button_input_res
        .get_just_pressed()
        .filter_map(|(key, value)| {
            if key.button == MouseButton::Left {
                Some((key.entity, value.position))
            } else {
                None
            }
        })
        .collect();

    if raycast_entities.is_empty() {
        return;
    }

    let now = web_time::Instant::now();
    let mut selection_candidates: Vec<SelectionCandidate> = Vec::new();

    let selected_node_parents: HashSet<Option<Entity>> = selected_node_query
        .iter()
        .map(|(_, maybe_parent, _)| {
            if let Some(parent) = maybe_parent {
                Some(parent.get())
            } else {
                None
            }
        })
        .collect();

    let shallowest_selected_hierarchy_level =
        selected_node_query
            .iter()
            .fold(0, |acc, (_, _, maybe_level)| {
                if let Some(level) = maybe_level {
                    acc.max(level.0)
                } else {
                    acc
                }
            });

    // Find nodes that could be selected or preselected
    for (entity, cursor_position) in raycast_entities.iter().rev().copied() {
        log::info!("[cursor_down_on_entity_system] Entity {:?}", entity,);

        if let Ok((maybe_parent, maybe_hierarchy_level)) = unselected_node_query.get(entity) {
            // Consider selecting preselected node
            if let Ok(Preselected { timestamp }) = preselected_node_query.get(entity) {
                if now.duration_since(*timestamp) <= DOUBLE_CLICK_WINDOW {
                    selection_candidates.push(SelectionCandidate {
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

            if let Some(parent) = maybe_parent {
                let parent_entity = parent.get();
                let is_parent_root = root_node_query.get(parent_entity).is_ok();

                // Consider selecting node whose parent is the root
                if is_parent_root {
                    selection_candidates.push(SelectionCandidate {
                        entity,
                        cursor_position,
                        preselect: false,
                        was_selected: false,
                        was_preselected: false,
                    });
                    continue;
                }

                // Consider preselecting node whose parent is selected
                if selected_node_query.get(parent_entity).is_ok() {
                    selection_candidates.push(SelectionCandidate {
                        entity,
                        cursor_position,
                        preselect: true,
                        was_selected: false,
                        was_preselected: false,
                    });
                    continue;
                }
            }

            // Consider selecting node whose hierarchy level is deeper
            // than the shallowest/topmost selected node
            // or whose hierarchy level is on the same level and has a shared parent (siblings)
            if let Some(hierarchy_level) = maybe_hierarchy_level {
                if hierarchy_level.0 < shallowest_selected_hierarchy_level
                    || (hierarchy_level.0 == shallowest_selected_hierarchy_level
                        && selected_node_parents.contains(&maybe_parent.map(|p| p.get())))
                {
                    selection_candidates.push(SelectionCandidate {
                        entity,
                        cursor_position,
                        preselect: false,
                        was_selected: false,
                        was_preselected: false,
                    });
                    continue;
                }
            }
        }

        // Consider selecting already selected node
        if selected_node_query.get(entity).is_ok() {
            selection_candidates.push(SelectionCandidate {
                entity,
                cursor_position,
                preselect: false,
                was_selected: true,
                was_preselected: false,
            });
        }
    }

    log::info!(
        "[cursor_down_on_entity_system] Preselection: {:?}",
        selection_candidates
    );

    let mut selected_node: Option<Entity> = None;
    let mut unselect_prev_selected = selection_candidates.len() == 0;

    // Go through selection candiates and preselect nodes
    // until actually selectable node found
    for SelectionCandidate {
        entity,
        cursor_position,
        preselect,
        was_selected,
        was_preselected,
    } in selection_candidates.iter().copied()
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
                    "[cursor_down_on_entity_system] Selected Entity {:?} at {:?}",
                    entity,
                    cursor_position
                );
            }

            selected_node = Some(entity);
            unselect_prev_selected = true;

            comp_interaction_res.interaction_mode = InteractionMode::Translating {
                origin: cursor_position,
                current: cursor_position,
            };

            break;
        }
    }

    // Unselect previously selected nodes that are no longer selected
    if unselect_prev_selected {
        for (entity, _, _) in selected_node_query.iter() {
            if selected_node.map_or(true, |e| e != entity) {
                commands.entity(entity).remove::<Selected>();
                #[cfg(feature = "tracing")]
                log::info!(
                    "[cursor_down_on_entity_system] Unselected Entity: {:?}",
                    entity
                );
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct SelectionCandidate {
    entity: Entity,
    cursor_position: Vec2,
    preselect: bool,
    was_preselected: bool,
    was_selected: bool,
}
