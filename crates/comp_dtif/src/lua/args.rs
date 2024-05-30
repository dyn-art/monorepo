use std::collections::HashMap;

use piccolo::{Context, Table};

// #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
// #[serde(tag = "type")]
// pub enum LuaScriptArgInputVariant {
//     Number { default: f32 },
//     String { defualt: String },
// }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(tag = "type")]
pub enum LuaScriptArg {
    Number { value: f32 },
    String { value: String },
}

pub type LuaScriptArgsMap = HashMap<String, LuaScriptArg>;

pub fn load_args_table_global<'gc>(ctx: Context<'gc>, args_map: LuaScriptArgsMap) {
    let args_table = create_args_table(ctx, args_map);
    ctx.set_global("args", args_table).unwrap();
}

fn create_args_table<'gc>(ctx: Context<'gc>, args_map: LuaScriptArgsMap) -> Table<'gc> {
    let args_table = Table::new(&ctx);

    for (key, value) in args_map {
        match value {
            LuaScriptArg::Number { value } => {
                args_table.set(ctx, key, value).unwrap();
            }
            LuaScriptArg::String { value } => {
                args_table.set(ctx, key, value).unwrap();
            }
        }
    }

    return args_table;
}
