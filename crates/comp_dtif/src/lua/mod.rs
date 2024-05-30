#![cfg(feature = "lua_scripts")]

pub mod comp;
#[macro_use]
pub mod freeze;
pub mod args;
pub mod script;

#[cfg(test)]
mod tests {
    use super::{
        args::{LuaScriptArg, LuaScriptArgsMap},
        freeze::Frozen,
        script::LuaScript,
    };
    use bevy_app::{App, Update};
    use bevy_ecs::event::EventReader;
    use dyn_comp_bundles::events::{CoreInputEvent, InputEvent, UpdateCompositionSizeInputEvent};
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
        app.add_systems(Update, event_listener_system);

        let code = r#"
            comp.log.warn("This is a warning")
            comp.log.info("This is an info message")
            comp.log.error("This is an error message")
            local sum = comp.sum(1, 2, 3)
            comp.log.info("Sum of 1, 2, 3 is " .. sum)
            comp.log.info("Value of: " .. args.input)

            local my_event = '{"type":"UpdateCompositionSize","size":[' .. args.input .. ',100]}'
            comp.send_event(my_event)
        "#;

        let script = LuaScript {
            source: code.to_string(),
        };

        let mut args_map: LuaScriptArgsMap = HashMap::new();
        args_map.insert(String::from("input"), LuaScriptArg::Number { value: 10.0 });

        Frozen::in_scope(&mut app.world, |world| {
            script.run(world, args_map);
        });

        app.update();
    }

    fn event_listener_system(mut event_reader: EventReader<UpdateCompositionSizeInputEvent>) {
        for event in event_reader.read() {
            log::info!("Event: {:?}", event);
        }
    }
}
