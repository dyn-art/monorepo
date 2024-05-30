#![cfg(feature = "lua_scripts")]

use super::script::FrozenWorld;
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

    let send_event_callback = callback("send_event", &ctx, move |_, v: Value| {
        if let Value::String(s) = v {
            match serde_json::from_str::<CoreInputEvent>(&s.to_str_lossy()) {
                Ok(event) => {
                    frozen_world.with_mut(|mut world| {
                        event.send_into_world(&mut world);
                    });
                    Some(Value::Nil)
                }
                Err(err) => {
                    log::error!(
                        "[send_event_callback] Failed to deserialize event '{}' by exception: {}",
                        s,
                        err
                    );
                    None
                }
            }
        } else {
            log::error!(
                "[send_event_callback] Invalid argument type for 'send_event': {}",
                v
            );
            None
        }
    });

    comp_table.set(ctx, "sum", sum_callback).unwrap();
    comp_table
        .set(ctx, "send_event", send_event_callback)
        .unwrap();

    let log_table = create_log_table(ctx);
    comp_table.set(ctx, "log", log_table).unwrap();

    return comp_table;
}

fn create_log_table<'gc>(ctx: Context<'gc>) -> Table<'gc> {
    let log_table = Table::new(&ctx);

    let warn_callback = callback("warn", &ctx, |_, v: Value| {
        if let Value::String(s) = v {
            log::warn!("{}", s.to_str_lossy());
            Some(Value::Nil)
        } else {
            None
        }
    });

    let info_callback = callback("info", &ctx, |_, v: Value| {
        if let Value::String(s) = v {
            log::info!("{}", s.to_str_lossy());
            Some(Value::Nil)
        } else {
            None
        }
    });

    let error_callback = callback("error", &ctx, |_, v: Value| {
        if let Value::String(s) = v {
            log::error!("{}", s.to_str_lossy());
            Some(Value::Nil)
        } else {
            None
        }
    });

    log_table.set(ctx, "warn", warn_callback).unwrap();
    log_table.set(ctx, "info", info_callback).unwrap();
    log_table.set(ctx, "error", error_callback).unwrap();

    return log_table;
}
