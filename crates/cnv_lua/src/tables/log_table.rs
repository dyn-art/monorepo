use crate::{helper::callback, json::lua_value_to_json};
use piccolo::{Context, Table, Value, Variadic};

pub fn load_log_table_global<'gc>(ctx: Context<'gc>) {
    let log_table = create_log_table(ctx);
    ctx.set_global("log", log_table).unwrap();
}

pub fn create_log_table<'gc>(ctx: Context<'gc>) -> Table<'gc> {
    let log_table = Table::new(&ctx);

    // Helper function to concatenate multiple Lua values into a single log message
    fn concatenate_log_message<'gc>(ctx: Context<'gc>, v: Variadic<Vec<Value<'gc>>>) -> String {
        v.into_iter()
            .map(|arg| {
                lua_value_to_json(ctx, arg)
                    .unwrap_or(serde_json::Value::Null)
                    .to_string()
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    let warn_callback = callback("warn", &ctx, |ctx, v: Variadic<Vec<Value>>| {
        let log_message = concatenate_log_message(ctx, v);
        log::warn!("{}", log_message);
        Some(Value::Nil)
    });

    let info_callback = callback("info", &ctx, |ctx, v: Variadic<Vec<Value>>| {
        let log_message = concatenate_log_message(ctx, v);
        log::info!("{}", log_message);
        Some(Value::Nil)
    });

    let error_callback = callback("error", &ctx, |ctx, v: Variadic<Vec<Value>>| {
        let log_message = concatenate_log_message(ctx, v);
        log::error!("{}", log_message);
        Some(Value::Nil)
    });

    log_table.set(ctx, "warn", warn_callback).unwrap();
    log_table.set(ctx, "info", info_callback).unwrap();
    log_table.set(ctx, "error", error_callback).unwrap();

    return log_table;
}
