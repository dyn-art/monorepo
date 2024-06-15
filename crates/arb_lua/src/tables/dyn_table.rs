use crate::helper::callback;
use piccolo::{Context, Table, Value, Variadic};

use super::{date_table::create_date_table, log_table::create_log_table};

pub fn load_dyn_table_global<'gc>(ctx: Context<'gc>) {
    let dyn_table = create_dyn_table(ctx);
    ctx.set_global("dyn", dyn_table).unwrap();
}

pub fn create_dyn_table<'gc>(ctx: Context<'gc>) -> Table<'gc> {
    let dyn_table = Table::new(&ctx);

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

    let tonumber_callback = callback("tonumber", &ctx, |_, v: Value| match &v {
        Value::Integer(i) => Some(Value::Number(*i as f64)),
        Value::Number(n) => Some(Value::Number(*n)),
        Value::String(s) => {
            if let Ok(n) = s.to_string().parse::<f64>() {
                Some(Value::Number(n))
            } else {
                None
            }
        }
        _ => None,
    });

    dyn_table.set(ctx, "sum", sum_callback).unwrap();
    dyn_table.set(ctx, "tonumber", tonumber_callback).unwrap();

    let log_table = create_log_table(ctx);
    dyn_table.set(ctx, "log", log_table).unwrap();

    let date_table = create_date_table(ctx);
    dyn_table.set(ctx, "date", date_table).unwrap();

    return dyn_table;
}
