use serde_json::{json, Value};

pub fn list_resources(cursor: Option<String>) -> Value {
    json!({
        "resources": [
            {
                "uri": "file:///project/src/main.rs",
                "name": "main.rs",
                "description": "Primary application entry point",
                "mimeType": "text/x-rust"
            }
        ],
        "nextCursor": null
    })
}

pub fn read_resource(uri: &str) -> Value {
    json!({
        "contents": [
            {
                "uri": uri,
                "mimeType": "text/x-rust",
                "text": "fn main() {\n    println!(\"Hello world!\");\n}"
            }
        ]
    })
}

pub fn list_resource_templates() -> Value {
    json!({
        "resourceTemplates": [
            {
                "uriTemplate": "file:///{path}",
                "name": "Project Files",
                "description": "Access files in the project directory"
            }
        ]
    })
}
