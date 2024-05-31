#![cfg(feature = "lua_scripts")]

use piccolo::{Context, Table};
use std::collections::HashMap;

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(tag = "type")
)]
pub enum LuaScriptArg {
    Number(LuaScriptNumberArg),
    String(LuaScriptStringArg),
    Boolean(LuaScriptBooleanArg),
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct LuaScriptNumberArg {
    pub value: f32,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct LuaScriptStringArg {
    pub value: String,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct LuaScriptBooleanArg {
    pub value: bool,
}

pub type LuaScriptArgsMap = HashMap<String, LuaScriptArg>;

pub fn load_args_table_global<'gc>(ctx: Context<'gc>, args_map: LuaScriptArgsMap) {
    let args_table = create_args_table(ctx, args_map);
    ctx.set_global("args", args_table).unwrap();
}

fn create_args_table<'gc>(ctx: Context<'gc>, args_map: LuaScriptArgsMap) -> Table<'gc> {
    let args_table = Table::new(&ctx);

    for (key, arg) in args_map {
        match arg {
            LuaScriptArg::Number(LuaScriptNumberArg { value }) => {
                args_table.set(ctx, key, value).unwrap();
            }
            LuaScriptArg::String(LuaScriptStringArg { value }) => {
                args_table.set(ctx, key, value).unwrap();
            }
            LuaScriptArg::Boolean(LuaScriptBooleanArg { value }) => {
                args_table.set(ctx, key, value).unwrap();
            }
        }
    }

    return args_table;
}
