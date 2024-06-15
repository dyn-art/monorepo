use crate::helper::callback;
use chrono::{DateTime, Datelike, FixedOffset, Local, TimeZone, Timelike};
use piccolo::{Context, Table, Value, Variadic};

pub fn load_date_table_global<'gc>(ctx: Context<'gc>) {
    let date_table = create_date_table(ctx);
    ctx.set_global("date", date_table).unwrap();
}

pub fn create_date_table<'gc>(ctx: Context<'gc>) -> Table<'gc> {
    let date_table = Table::new(&ctx);

    // Get the current time
    let now_callback = callback("now", &ctx, move |ctx, _: Value| {
        let now: DateTime<Local> = Local::now();
        Some(Value::String(ctx.intern(now.to_rfc3339().as_bytes())))
    });

    // Format a given date
    let format_callback = callback("format", &ctx, move |ctx, v: Variadic<Vec<Value>>| {
        if v.len() < 2 {
            return Some(Value::Nil);
        }
        let format_str = match v[0] {
            Value::String(s) => s.to_str_lossy(),
            _ => return Some(Value::Nil),
        };

        let date = match value_to_date(v[1]) {
            Ok(date) => date,
            Err(_) => return Some(Value::Nil),
        };

        let formatted_date = date.format(&format_str).to_string();
        Some(Value::String(ctx.intern(formatted_date.as_bytes())))
    });

    // Parse a given date to table
    let parse_callback = callback("parse", &ctx, move |ctx, v: Variadic<Vec<Value>>| {
        if v.is_empty() {
            return Some(Value::Nil);
        }
        let date = match value_to_date(v[0]) {
            Ok(date) => date,
            Err(_) => return Some(Value::Nil),
        };

        let table = Table::new(&ctx);

        table
            .set(ctx, "year", Value::Integer(date.year().into()))
            .unwrap();
        table
            .set(ctx, "month", Value::Integer(date.month().into()))
            .unwrap();
        table
            .set(ctx, "day", Value::Integer(date.day().into()))
            .unwrap();
        table
            .set(ctx, "hour", Value::Integer(date.hour().into()))
            .unwrap();
        table
            .set(ctx, "minute", Value::Integer(date.minute().into()))
            .unwrap();
        table
            .set(ctx, "second", Value::Integer(date.second().into()))
            .unwrap();

        Some(Value::Table(table))
    });

    date_table.set(ctx, "now", now_callback).unwrap();
    date_table.set(ctx, "format", format_callback).unwrap();
    date_table.set(ctx, "parse", parse_callback).unwrap();

    return date_table;
}

fn local_to_fixed(local_date_time: DateTime<Local>) -> DateTime<FixedOffset> {
    local_date_time.with_timezone(local_date_time.offset())
}

fn value_to_date(value: Value) -> Result<DateTime<FixedOffset>, String> {
    match value {
        Value::String(s) => DateTime::parse_from_rfc3339(&s.to_str_lossy())
            .map_err(|e| format!("Failed to parse date: {}", e)),
        Value::Integer(i) => Ok(local_to_fixed(Local.timestamp_millis_opt(i).unwrap())),
        Value::Number(n) => Ok(local_to_fixed(
            Local.timestamp_millis_opt(n as i64).unwrap(),
        )),
        _ => Err(format!("Cannot parse '{}' as date!", value)),
    }
}
