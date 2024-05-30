#![cfg(feature = "lua_scripts")]

use super::{
    args::{load_args_table_global, LuaScriptArgsMap},
    code::run_code,
    comp::load_comp_table_global,
    freeze::{Freeze, Frozen},
};
use bevy_ecs::world::World;
use piccolo::{Executor, Lua};

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

impl LuaScript {
    pub fn run(&self, world: FrozenWorld, args_map: LuaScriptArgsMap) {
        let mut lua = Lua::full();

        let executor = lua.enter(|ctx| ctx.stash(Executor::new(ctx)));

        lua.enter(|ctx| {
            load_comp_table_global(ctx, world);
            load_args_table_global(ctx, args_map);
        });

        return match run_code(&mut lua, &executor, &self.source) {
            Ok(_) => log::info!("Lua code executed successfully"),
            Err(err) => {
                log::error!("Failed to execute Lua code by exception: {:?}", err);
            }
        };
    }
}
