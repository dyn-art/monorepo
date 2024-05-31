#![cfg(feature = "lua_scripts")]

use piccolo::{Context, Table, Value};
use serde_json::Value as JsonValue;
use std::collections::HashMap;

pub type LuaScriptArgsMap = HashMap<String, JsonValue>;

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

fn json_to_lua_value<'gc>(ctx: Context<'gc>, json_value: JsonValue) -> Result<Value<'gc>, String> {
    match json_value {
        JsonValue::Null => Ok(Value::Nil),
        JsonValue::Bool(b) => Ok(Value::Boolean(b)),
        JsonValue::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(Value::Integer(i))
            } else if let Some(f) = n.as_f64() {
                Ok(Value::Number(f))
            } else {
                Err(format!("Invalid number: {}", n))
            }
        }
        JsonValue::String(s) => Ok(Value::String(ctx.intern(s.as_bytes()))),
        JsonValue::Array(arr) => {
            let table = Table::new(&ctx);
            for (index, item) in arr.into_iter().enumerate() {
                table
                    .set(ctx, index as i64 + 1, json_to_lua_value(ctx, item)?)
                    .unwrap();
            }
            Ok(Value::Table(table))
        }
        JsonValue::Object(obj) => {
            let table = Table::new(&ctx);
            for (key, value) in obj.into_iter() {
                table.set(ctx, key, json_to_lua_value(ctx, value)?).unwrap();
            }
            Ok(Value::Table(table))
        }
    }
}
