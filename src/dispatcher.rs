use serde_json::{Value, json};
use crate::features::{tools, prompts, resources, utilities};

pub fn handle_request(req: Value) -> Option<Value> {
    let method = req.get("method").and_then(Value::as_str).unwrap_or("");
    let id = req.get("id").cloned().unwrap_or(json!(null));

    let response = match method {
        "initialize" => {
            let client_ver = req.get("params")
                .and_then(|p| p.get("protocolVersion"))
                .and_then(Value::as_str)
                .unwrap_or("2025-03-26");

            json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "protocolVersion": client_ver,
                    "capabilities": {
                        "tools": { "listChanged": true },
                        "prompts": { "listChanged": true },
                        "resources": { "listChanged": true, "subscribe": true },
                        "completions": {},
                        "logging": {}
                    },
                    "serverInfo": {
                        "name": "rust-mcp-server",
                        "version": "1.0.0"
                    }
                }
            })
        }

        "tools/list" => json!({ "jsonrpc": "2.0", "id": id, "result": tools::list_tools() }),
        "tools/call" => {
            let name = req.pointer("/params/name").and_then(Value::as_str).unwrap_or("");
            let args = req.pointer("/params/arguments").cloned().unwrap_or(json!({}));
            let result = tools::call_tool(name, &args);
            wrap_result_or_error(id, result)
        }

        "prompts/list" => json!({ "jsonrpc": "2.0", "id": id, "result": prompts::list_prompts() }),
        "prompts/call" => {
            let name = req.pointer("/params/name").and_then(Value::as_str).unwrap_or("");
            let args = req.pointer("/params/arguments").cloned().unwrap_or(json!({}));
            let result = prompts::call_prompt(name, &args);
            wrap_result_or_error(id, result)
        }

        "resources/list" => {
            let cursor = req.pointer("/params/cursor").and_then(Value::as_str).map(|s| s.to_string());
            json!({ "jsonrpc": "2.0", "id": id, "result": resources::list_resources(cursor) })
        }

        "resources/read" => {
            let uri = req.pointer("/params/uri").and_then(Value::as_str).unwrap_or("");
            json!({ "jsonrpc": "2.0", "id": id, "result": resources::read_resource(uri) })
        }

        "resources/templates/list" => json!({ "jsonrpc": "2.0", "id": id, "result": resources::list_resource_templates() }),

        "completion/complete" => {
            let fallback_ref = json!({});
            let r = req.pointer("/params/ref").unwrap_or(&fallback_ref);

            let fallback_arg = json!({});
            let a = req.pointer("/params/argument").unwrap_or(&fallback_arg);
            let result = utilities::completion::complete(
                r.get("type").and_then(Value::as_str).unwrap_or(""),
                r.get("name").or_else(|| r.get("uri")).and_then(Value::as_str).unwrap_or(""),
                a.get("name").and_then(Value::as_str).unwrap_or(""),
                a.get("value").and_then(Value::as_str).unwrap_or(""),
            );
            json!({ "jsonrpc": "2.0", "id": id, "result": result })
        }

        "logging/setLevel" => {
            let level = req.pointer("/params/level").and_then(Value::as_str).unwrap_or("info");
            json!({ "jsonrpc": "2.0", "id": id, "result": utilities::logging::set_log_level(level) })
        }

        "shutdown" => {
            return Some(json!({ "jsonrpc": "2.0", "id": id, "result": null }))
        }

        "notifications/initialized" | "notifications/cancelled" => return None,

        _ => json!({
            "jsonrpc": "2.0",
            "id": id,
            "error": {
                "code": -32601,
                "message": format!("Method not found: {}", method)
            }
        }),
    };

    Some(response)
}

fn wrap_result_or_error(id: Value, result: Value) -> Value {
    if result.get("error").is_some() {
        json!({ "jsonrpc": "2.0", "id": id, "error": result["error"] })
    } else {
        json!({ "jsonrpc": "2.0", "id": id, "result": result })
    }
}
