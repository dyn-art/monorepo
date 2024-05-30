use super::script::WorldRef;
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

pub fn load_comp_global<'gc>(ctx: Context<'gc>, world_ref: WorldRef) {
    let comp_table = load_comp(ctx, world_ref);
    ctx.set_global("comp", comp_table).unwrap();
}

fn load_comp<'gc>(ctx: Context<'gc>, world_ref: WorldRef) -> Table<'gc> {
    let comp = Table::new(&ctx);

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

    let send_event_callback = callback("send_event", &ctx, move |ctx, v: Value| {
        if let Value::String(s) = v {
            // let world_value = ctx.get_global("world");
            // let world_ref = match world_value {
            //     Value::UserData(ud) => ud.downcast_static::<WorldRef>().ok(),
            //     _ => None,
            // }?;

            match serde_json::from_str::<CoreInputEvent>(&s.to_str_lossy()) {
                Ok(event) => {
                    // TODO: Doesn't work: called `Result::unwrap()` on an `Err` value: Expired
                    world_ref.with_mut(|mut world| {
                        event.send_into_world(&mut world);
                    });
                    Some(Value::Nil)
                }
                Err(err) => {
                    log::error!("Failed to deserialize event: {}", err);
                    None
                }
            }
        } else {
            log::error!("Invalid argument type for 'send_event': {}", v);
            None
        }
    });

    comp.set(ctx, "sum", sum_callback).unwrap();
    comp.set(ctx, "send_event", send_event_callback).unwrap();

    let log_table = load_log(ctx);
    comp.set(ctx, "log", log_table).unwrap();

    return comp;
}

fn load_log<'gc>(ctx: Context<'gc>) -> Table<'gc> {
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
