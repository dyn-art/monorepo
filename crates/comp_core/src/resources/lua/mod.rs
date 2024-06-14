#![cfg(feature = "lua_scripts")]

pub mod comp_table;

use bevy_ecs::system::Resource;
use comp_table::{load_comp_table_global, FrozenWorld};
use dyn_comp_bundles::reference_id::ReferenceId;
use dyn_comp_lua::{
    script::{LuaScript, LuaScriptError},
    tables::args_table::LuaScriptArgsMap,
};
use piccolo::{Closure, Executor, Function, Lua, StashedExecutor, StaticError};
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct LuaRes {
    scripts: HashMap<ReferenceId, LuaScript>,
}

impl LuaRes {
    pub fn register_script(&mut self, id: ReferenceId, script: LuaScript) {
        self.scripts.insert(id, script);
    }

    fn load_code(lua: &mut Lua, executor: &StashedExecutor, code: &str) -> Result<(), StaticError> {
        lua.try_enter(|ctx| {
            let closure = Closure::load(ctx, None, code.as_bytes())?;
            let function = Function::compose(&ctx, [closure.into()]);
            ctx.fetch(executor).restart(ctx, function, ());
            Ok(())
        })
    }

    pub fn setup_lua(
        &self,
        id: &ReferenceId,
        frozen_world: FrozenWorld,
        args_map: LuaScriptArgsMap,
    ) -> Result<(Lua, StashedExecutor), LuaScriptError> {
        if let Some(LuaScript { source }) = self.scripts.get(id) {
            let mut lua = LuaScript::full_lua(args_map);

            let executor = lua.enter(|ctx| ctx.stash(Executor::new(ctx)));

            lua.enter(|ctx| {
                load_comp_table_global(ctx, frozen_world);
            });

            match LuaRes::load_code(&mut lua, &executor, &source) {
                Ok(_) => {}
                Err(err) => {
                    return Err(LuaScriptError::from_static_error(err));
                }
            };

            return Ok((lua, executor));
        }

        return Err(LuaScriptError::NotFound);
    }

    pub fn execute_lua(lua: &mut Lua, executor: &StashedExecutor) -> Result<(), LuaScriptError> {
        return match lua.execute::<()>(executor) {
            Ok(_) => Ok(()),
            Err(err) => Err(LuaScriptError::from_static_error(err)),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::LuaRes;
    use crate::systems::events::{
        execute_lua_script_input_system, register_lua_script_input_system,
    };
    use bevy_app::{App, First, Update};
    use bevy_ecs::{event::EventReader, schedule::IntoSystemConfigs};
    use dyn_comp_bundles::{
        events::{
            CoreInputEvent, ExecuteLuaScriptInputEvent, InputEvent, RegisterLuaScriptInputEvent,
            UpdateCompositionSizeInputEvent, UpdateEntityTransformInputEvent,
            UpdateTextNodeInputEvent,
        },
        reference_id::ReferenceId,
        LuaScriptWithId,
    };
    use dyn_comp_lua::tables::args_table::LuaScriptArgsMap;
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
        app.init_resource::<LuaRes>();
        app.add_systems(
            First,
            (
                register_lua_script_input_system,
                execute_lua_script_input_system.after(register_lua_script_input_system),
            ),
        );
        app.add_systems(
            Update,
            (
                update_composition_size_input_system,
                update_entity_transform_input_system,
                update_text_node_input_system,
            ),
        );

        let code = r#"
            dyn.log.warn("This is a warning")
            dyn.log.info("This is an info message")
            dyn.log.error("This is an error message")
            local sum = dyn.sum(1, 2, 3)
            dyn.log.info("Sum of 1, 2, 3 is " .. sum)
            dyn.log.info("Value of: " .. args.input)

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

            dyn.log.info("Table Log:", update_entity_transform_event, {10, 20, 30})

            comp.sendEvents({ update_composition_size_event, update_entity_transform_event })

            local date = args.date
            local dateObj = dyn.date.parse(date)
            local hours = dateObj.hour
            local minutes = dateObj.minute
            local ampm = hours >= 12 and 'PM' or 'AM'
            hours = hours % 12
            hours = hours ~= 0 and hours or 12
            local minutesStr = minutes < 10 and '0' .. minutes or minutes
            local timeStr = hours .. ":" .. minutesStr .. " " .. ampm
            local dateStr = dyn.date.format('%b %d, %Y', date)
            local combinedStr = timeStr .. " · " .. dateStr

            comp.sendEvent({
                type = "UpdateTextNode",
                id = { type = "ReferenceId", referenceId = "n19" },
                text = combinedStr
            })
        "#;

        let script_id = ReferenceId::new(String::from("1"));
        let script = LuaScriptWithId {
            id: script_id.clone(),
            source: vec![code.to_string()],
        };

        app.world.send_event(RegisterLuaScriptInputEvent { script });

        let mut args_map: LuaScriptArgsMap = HashMap::new();
        args_map.insert(String::from("input"), json!(10.0));
        args_map.insert(String::from("nodeId"), json!("n2"));
        args_map.insert(String::from("x"), json!(20.0));
        args_map.insert(String::from("y"), json!(30.0));
        args_map.insert(String::from("date"), json!(1717252549107 as i64));

        app.world.send_event(ExecuteLuaScriptInputEvent {
            id: script_id,
            args_map,
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
