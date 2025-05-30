use serde_json::{json, Value};

pub fn complete(ref_type: &str, name_or_uri: &str, argument_name: &str, argument_value: &str) -> Value {
    json!({
        "completion": {
            "values": ["example1", "example2", "example3"],
            "total": 3,
            "hasMore": false
        }
    })
}
