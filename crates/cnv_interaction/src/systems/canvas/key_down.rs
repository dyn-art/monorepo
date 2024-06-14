use crate::{
    components::Selected,
    events::KeyDownOnCnvInputEvent,
    input::keyboard::{KeyCode, KeyCodeButtonInput},
};
use bevy_ecs::{
    change_detection::DetectChangesMut,
    entity::Entity,
    event::EventReader,
    query::With,
    system::{Commands, Query, Res, ResMut},
};
use bevy_hierarchy::{BuildChildren, Children};
use dyn_cnv_bundles::components::marker::Removed;

pub fn key_down_input_system(
    mut event_reader: EventReader<KeyDownOnCnvInputEvent>,
    mut keyboard_input_res: ResMut<KeyCodeButtonInput>,
) {
    keyboard_input_res.bypass_change_detection().clear();
    for event in event_reader.read() {
        log::info!("[key_down_input_system] {:?}", event.key_code);
        keyboard_input_res.press(event.key_code, ());
    }
}

pub fn remove_selected_entity_system(
    mut commands: Commands,
    keyboard_input_res: Res<KeyCodeButtonInput>,
    selected_entities_query: Query<Entity, With<Selected>>,
    children_query: Query<&Children>,
) {
    if keyboard_input_res.any_just_pressed([KeyCode::Backspace, KeyCode::Delete]) {
        for entity in selected_entities_query.iter() {
            commands
                .entity(entity)
                .insert(Removed)
                .remove::<Selected>()
                .remove_parent();

            if let Ok(children) = children_query.get(entity) {
                for child in children.iter() {
                    commands.entity(*child).insert(Removed);
                }
            }
        }
    }
}
