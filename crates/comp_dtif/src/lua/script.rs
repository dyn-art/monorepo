#![cfg(feature = "lua_scripts")]

use super::{
    args::{load_args_table_global, LuaScriptArgsMap},
    comp::load_comp_table_global,
    freeze::{Freeze, Frozen},
};
use bevy_ecs::world::World;
use piccolo::{Closure, Executor, Function, Lua, StashedExecutor, StaticError};
use std::collections::HashMap;

/// A frozen reference to the ECS [`World`].
pub type FrozenWorld = Frozen<Freeze![&'freeze mut World]>;

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct LuaScript {
    pub source: String,
}

// TODO: Optimize, like run multiple scripts with one lua instance, ..
impl LuaScript {
    pub fn run(&self, world: FrozenWorld, args_map: LuaScriptArgsMap) {
        let mut lua = Lua::full();

        let executor = lua.enter(|ctx| ctx.stash(Executor::new(ctx)));

        lua.enter(|ctx| {
            load_comp_table_global(ctx, world);
            load_args_table_global(ctx, args_map);
        });

        return match Self::run_code(&mut lua, &executor, &self.source) {
            Ok(_) => log::info!("[run] Lua code executed successfully"),
            Err(err) => {
                log::error!("[run] Failed to execute Lua code by exception: {:?}", err);
            }
        };
    }

    fn run_code(lua: &mut Lua, executor: &StashedExecutor, code: &str) -> Result<(), StaticError> {
        lua.try_enter(|ctx| {
            let closure = Closure::load(ctx, None, code.as_bytes())?;
            let function = Function::compose(&ctx, [closure.into()]);
            ctx.fetch(executor).restart(ctx, function, ());
            Ok(())
        })?;

        return lua.execute::<()>(executor);
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct ToRunLuaScript {
    pub key: String,
    pub args_map: LuaScriptArgsMap,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct ToRunLuaScripts(pub Vec<ToRunLuaScript>);

impl ToRunLuaScripts {
    pub fn run_batch(self, scripts: &HashMap<String, LuaScript>, world: &mut World) {
        Frozen::in_scope(world, |world| {
            for ToRunLuaScript {
                key,
                args_map: args,
            } in self.0
            {
                if let Some(script) = scripts.get(&key) {
                    script.run(world.clone(), args);
                }
            }
        });
    }
}
