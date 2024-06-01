#![cfg(feature = "lua_scripts")]

#[macro_use]
pub mod freeze;
pub mod json;
pub mod lib;
pub mod script;
pub mod serde;

#[cfg(test)]
mod tests {
    use super::{freeze::Frozen, lib::args::LuaScriptArgsMap, script::LuaScript};
    use bevy_app::{App, Update};
    use bevy_ecs::event::EventReader;
    use dyn_comp_bundles::events::{
        CoreInputEvent, InputEvent, UpdateCompositionSizeInputEvent,
        UpdateEntityTransformInputEvent, UpdateTextNodeInputEvent,
    };
    use serde_json::json;
    use std::collections::HashMap;

    fn init() {
        let _ = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
            .is_test(true)
            .try_init();
    }

    #[test]
    fn e2e() {
        init();

        let mut app = App::new();
        CoreInputEvent::register_events(&mut app);
        app.add_systems(
            Update,
            (
                update_composition_size_input_system,
                update_entity_transform_input_system,
                update_text_node_input_system,
            ),
        );

        let code = r#"
            comp.log.warn("This is a warning")
            comp.log.info("This is an info message")
            comp.log.error("This is an error message")
            local sum = comp.sum(1, 2, 3)
            comp.log.info("Sum of 1, 2, 3 is " .. sum)
            comp.log.info("Value of: " .. args.input)

            -- error("A message?")

            local update_composition_size_event = {
                type = "UpdateCompositionSize",
                size = { args.input, 100 }
            }
            local update_entity_transform_event = {
                type = "UpdateEntityTransform",
                id = { type = "ReferenceId", referenceId = args.nodeId },
                x = args.x,
                y = args.y
            }

            comp.log.info("Table Log:", update_entity_transform_event, {10, 20, 30})

            comp.sendEvents({ update_composition_size_event, update_entity_transform_event })

            local date = args.date
            local dateObj = comp.date.parse(date)
            local hours = dateObj.hour
            local minutes = dateObj.minute
            local ampm = hours >= 12 and 'PM' or 'AM'
            hours = hours % 12
            hours = hours ~= 0 and hours or 12
            local minutesStr = minutes < 10 and '0' .. minutes or minutes
            local timeStr = hours .. ":" .. minutesStr .. " " .. ampm
            local dateStr = comp.date.format('%b %d, %Y', date)
            local combinedStr = timeStr .. " · " .. dateStr

            comp.sendEvent({
                type = "UpdateTextNode",
                id = { type = "ReferenceId", referenceId = "n19" },
                text = combinedStr
            })
        "#;

        let script = LuaScript {
            source: code.to_string(),
        };

        let mut args_map: LuaScriptArgsMap = HashMap::new();
        args_map.insert(String::from("input"), json!(10.0));
        args_map.insert(String::from("nodeId"), json!("n2"));
        args_map.insert(String::from("x"), json!(20.0));
        args_map.insert(String::from("y"), json!(30.0));
        args_map.insert(String::from("date"), json!(1717252549107 as i64));

        Frozen::in_scope(&mut app.world, |world| {
            script.run(world, args_map).unwrap();
        });

        app.update();
    }

    fn update_composition_size_input_system(
        mut event_reader: EventReader<UpdateCompositionSizeInputEvent>,
    ) {
        for event in event_reader.read() {
            log::info!("[update_composition_size_input_system] {:?}", event);
        }
    }

    fn update_entity_transform_input_system(
        mut event_reader: EventReader<UpdateEntityTransformInputEvent>,
    ) {
        for event in event_reader.read() {
            log::info!("[update_entity_transform_input_system] {:?}", event);
        }
    }

    fn update_text_node_input_system(mut event_reader: EventReader<UpdateTextNodeInputEvent>) {
        for event in event_reader.read() {
            log::info!("[update_text_node_input_system] {:?}", event);
        }
    }
}
