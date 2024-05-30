#![cfg(feature = "lua_scripts")]

use piccolo::{Closure, Function, Lua, StashedExecutor, StaticError};

pub fn run_code(lua: &mut Lua, executor: &StashedExecutor, code: &str) -> Result<(), StaticError> {
    lua.try_enter(|ctx| {
        let closure = Closure::load(ctx, None, code.as_bytes())?;
        let function = Function::compose(&ctx, [closure.into()]);
        ctx.fetch(executor).restart(ctx, function, ());
        Ok(())
    })?;

    return lua.execute::<()>(executor);
}
