// log record formatting helpers

use crate::appcontext::Interface;
use crate::log::level::Level;
use serde_json::{Map, Value};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn now_unix() -> String {
    let dur = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    dur.as_secs().to_string()
}

pub fn time_elapsed_ms(start: SystemTime) -> String {
    if start == UNIX_EPOCH {
        return "0ms".to_string();
    }
    let elapsed = SystemTime::now().duration_since(start).unwrap_or_default();
    format!("{}ms", elapsed.as_millis())
}

pub fn base_context_fields(ctx: &crate::appcontext::AppContext) -> Map<String, Value> {
    let mut m = Map::new();
    m.insert(
        "request_id".to_string(),
        Value::String(ctx.get_request_id().to_string()),
    );
    m.insert(
        "user_agent".to_string(),
        Value::String(ctx.get_user_agent().to_string()),
    );
    m.insert(
        "service_version".to_string(),
        Value::String(ctx.get_service_version().to_string()),
    );
    m.insert(
        "time_elapsed".to_string(),
        Value::String(time_elapsed_ms(ctx.get_request_start_time())),
    );
    m
}

/// Build a JSON log line. `fields` is the final field set after merge/replace logic
/// (caller must compute base vs custom). `caller` optional.
pub fn build_record(
    level: Level,
    message: &str,
    caller: Option<&str>,
    fields: Map<String, Value>,
) -> String {
    let mut obj = Map::new();
    obj.insert(
        "level".to_string(),
        Value::String(level.as_str().to_string()),
    );
    obj.insert("time".to_string(), Value::String(now_unix()));
    if let Some(c) = caller {
        obj.insert("caller".to_string(), Value::String(c.to_string()));
    }
    obj.insert("message".to_string(), Value::String(message.to_string()));
    for (k, v) in fields {
        obj.insert(k, v);
    }
    serde_json::to_string(&Value::Object(obj)).unwrap_or_else(|_| "{}".to_string())
}
