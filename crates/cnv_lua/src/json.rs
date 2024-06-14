use super::serde::de::is_sequence;
use piccolo::{Context, Table, Value};
use serde_json::Value as JsonValue;

pub fn json_to_lua_value<'gc>(
    ctx: Context<'gc>,
    json_value: JsonValue,
) -> Result<Value<'gc>, String> {
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

pub fn lua_value_to_json<'gc>(ctx: Context<'gc>, value: Value<'gc>) -> Result<JsonValue, String> {
    match value {
        Value::Nil => Ok(JsonValue::Null),
        Value::Boolean(b) => Ok(JsonValue::Bool(b)),
        Value::Integer(i) => Ok(JsonValue::Number(i.into())),
        Value::Number(n) => Ok(JsonValue::Number(serde_json::Number::from_f64(n).unwrap())),
        Value::String(s) => Ok(JsonValue::String(s.to_str_lossy().to_string())),
        Value::Table(table) => {
            if is_sequence(table) {
                let mut arr = Vec::new();
                for i in 1..=table.length() {
                    let value = table.get(ctx, i);
                    arr.push(lua_value_to_json(ctx, value)?);
                }
                Ok(JsonValue::Array(arr))
            } else {
                let mut map = serde_json::Map::new();
                for (key, value) in table.iter() {
                    let key_str = match key {
                        Value::String(s) => s.to_str_lossy().to_string(),
                        Value::Integer(i) => i.to_string(),
                        Value::Number(n) => n.to_string(),
                        _ => {
                            return Err(format!("Invalid key: {}", key));
                        }
                    };
                    map.insert(key_str, lua_value_to_json(ctx, value)?);
                }
                Ok(JsonValue::Object(map))
            }
        }
        Value::Function(_) => Err(String::from("Cannot map Lua function to JSON!")),
        Value::Thread(_) => Err(String::from("Cannot map Lua thread to JSON!")),
        Value::UserData(_) => Err(String::from("Cannot map Lua userdata to JSON!")),
    }
}
