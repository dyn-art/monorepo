#![cfg(feature = "lua_scripts")]

use crate::lua::{json::lua_value_to_json, script::FrozenWorld, serde::from_value};
use dyn_comp_bundles::events::{CoreInputEvent, InputEvent};
use gc_arena::Mutation;
use piccolo::{
    Callback, CallbackReturn, Context, FromMultiValue, IntoMultiValue, IntoValue, Table, Value,
    Variadic,
};

fn callback<'gc, F, A, R>(name: &'static str, mc: &Mutation<'gc>, f: F) -> Callback<'gc>
where
    F: Fn(Context<'gc>, A) -> Option<R> + 'static,
    A: FromMultiValue<'gc>,
    R: IntoMultiValue<'gc>,
{
    Callback::from_fn(mc, move |ctx, _, mut stack| {
        if let Some(res) = f(ctx, stack.consume(ctx)?) {
            stack.replace(ctx, res);
            Ok(CallbackReturn::Return)
        } else {
            Err(format!("Bad argument to {name}").into_value(ctx).into())
        }
    })
}

pub fn load_comp_table_global<'gc>(ctx: Context<'gc>, frozen_world: FrozenWorld) {
    let comp_table = create_comp_table(ctx, frozen_world);
    ctx.set_global("comp", comp_table).unwrap();
}

fn create_comp_table<'gc>(ctx: Context<'gc>, frozen_world: FrozenWorld) -> Table<'gc> {
    let comp_table = Table::new(&ctx);

    let sum_callback = callback("sum", &ctx, |_, v: Variadic<Vec<Value>>| {
        if v.is_empty() {
            None
        } else {
            let sum = v
                .into_iter()
                .try_fold(0.0, |acc, value| match value {
                    Value::Integer(i) => Ok(acc + i as f64),
                    Value::Number(n) => Ok(acc + n),
                    _ => Err(format!("Invalid argument type for 'sum': {:?}", value)),
                })
                .ok()?;

            Some(Value::Number(sum))
        }
    });

    let movable_frozen_world = frozen_world.clone();
    let send_event_callback = callback("sendEvent", &ctx, move |_, v: Value| {
        match from_value::<CoreInputEvent>(v) {
            Ok(event) => {
                movable_frozen_world.with_mut(|mut world| {
                    event.send_into_world(&mut world);
                });
                Some(Value::Nil)
            }
            Err(err) => {
                log::error!(
                    "[send_event_callback] Failed to parse value '{}' as event by exception: {}",
                    v,
                    err
                );
                None
            }
        }
    });

    let movable_frozen_world = frozen_world.clone();
    let send_events_callback = callback("sendEvents", &ctx, move |_, v: Value| match v {
        Value::Table(events_table) => {
            for (_, event_value) in events_table.iter() {
                match from_value::<CoreInputEvent>(event_value) {
                    Ok(event) => {
                        movable_frozen_world.with_mut(|mut world| {
                            event.send_into_world(&mut world);
                        });
                    }
                    Err(err) => {
                        log::error!(
                                "[send_events_callback] Failed to parse value '{}' as event by exception: {}",
                                event_value,
                                err
                            );
                    }
                }
            }
            Some(Value::Nil)
        }
        _ => {
            log::error!("[send_events_callback] Expected a table of events");
            None
        }
    });

    comp_table.set(ctx, "sum", sum_callback).unwrap();
    comp_table
        .set(ctx, "sendEvent", send_event_callback)
        .unwrap();
    comp_table
        .set(ctx, "sendEvents", send_events_callback)
        .unwrap();

    let log_table = create_log_table(ctx);
    comp_table.set(ctx, "log", log_table).unwrap();

    return comp_table;
}

fn create_log_table<'gc>(ctx: Context<'gc>) -> Table<'gc> {
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
