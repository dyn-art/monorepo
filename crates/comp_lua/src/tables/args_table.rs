use crate::json::json_to_lua_value;
use piccolo::{Context, Table};
use std::collections::HashMap;

pub type LuaScriptArgsMap = HashMap<String, serde_json::Value>;

pub fn load_args_table_global<'gc>(ctx: Context<'gc>, args_map: LuaScriptArgsMap) {
    let args_table = create_args_table(ctx, args_map);
    ctx.set_global("args", args_table).unwrap();
}

fn create_args_table<'gc>(ctx: Context<'gc>, args_map: LuaScriptArgsMap) -> Table<'gc> {
    let args_table = Table::new(&ctx);

    for (key, arg) in args_map {
        match json_to_lua_value(ctx, arg) {
            Ok(lua_value) => {
                args_table.set(ctx, key, lua_value).unwrap();
            }
            Err(err) => {
                log::error!("[create_args_table] Failed to convert JSON value to Lua value by exception: {}", err);
            }
        }
    }

    return args_table;
}
