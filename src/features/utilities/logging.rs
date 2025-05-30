use serde_json::{json, Value};

pub fn set_log_level(level: &str) -> Value {
    json!({
        "result": null
    })
}

pub fn log_message(level: &str, logger: &str, data: Value) -> Value {
    json!({
        "level": level,
        "logger": logger,
        "data": data
    })
}
