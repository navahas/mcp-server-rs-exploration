use serde_json::{json, Value};

pub fn paginate_resources(cursor: Option<String>) -> Value {
    json!({
        "resources": [
            {
                "uri": "file:///project/src/lib.rs",
                "name": "lib.rs",
                "description": "Library module",
                "mimeType": "text/x-rust"
            }
        ],
        "nextCursor": null
    })
}
