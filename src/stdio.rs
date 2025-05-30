mod features;
use features::{tools, prompts, resources, utilities};

use std::io::{self, BufRead, Write};
use serde_json::{Value, json};

fn main() -> io::Result<()> {
    eprintln!("Starting MCP server...");

    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = match line {
            Ok(l) if !l.trim().is_empty() => l,
            _ => continue,
        };

        let req: Value = match serde_json::from_str(&line) {
            Ok(v) => v,
            Err(err) => {
                eprintln!("Invalid JSON: {err}");
                continue;
            }
        };

        let method = req.get("method").and_then(Value::as_str).unwrap_or("");
        let id = req.get("id").cloned().unwrap_or(json!(null));

        let resp = match method {
            "initialize" => {
                let client_ver = req.get("params")
                    .and_then(|p| p.get("protocolVersion"))
                    .and_then(Value::as_str)
                    .unwrap_or("2024-11-05");
                
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "protocolVersion": client_ver,
                        "capabilities": {
                            "tools": {
                                "listChanged": true
                            }
                        },
                        "serverInfo": {
                            "name": "rust-math-server",
                            "version": "1.0.0"
                        }
                    }
                })
            }

            "prompts/list" => {
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": prompts::list_prompts()
                })
            }

            "prompts/call" => {
                let prompt_name = req.get("params")
                    .and_then(|p| p.get("name"))
                    .and_then(Value::as_str)
                    .unwrap_or("");

                let arguments = req.get("params")
                    .and_then(|p| p.get("arguments"))
                    .cloned()
                    .unwrap_or(json!({}));

                let result = prompts::call_prompt(prompt_name, &arguments);

                if result.get("error").is_some() {
                    json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "error": result["error"]
                    })
                } else {
                    json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": result
                    })
                }
            }

            "resources/list" => {
                let cursor = req.get("params")
                    .and_then(|p| p.get("cursor"))
                    .and_then(Value::as_str)
                    .map(|s| s.to_string());

                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": resources::list_resources(cursor)
                })
            }

            "resources/read" => {
                let uri = req.get("params")
                    .and_then(|p| p.get("uri"))
                    .and_then(Value::as_str)
                    .unwrap_or("");

                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": resources::read_resource(uri)
                })
            }

            "resources/templates/list" => {
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": resources::list_resource_templates()
                })
            }

            "tools/list" => {
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": tools::list_tools()
                })
            }

            "tools/call" => {
                let tool_name = req.get("params")
                    .and_then(|p| p.get("name"))
                    .and_then(Value::as_str)
                    .unwrap_or("");

                let arguments = req.get("params")
                    .and_then(|p| p.get("arguments"))
                    .cloned()
                    .unwrap_or(json!({}));

                let result = tools::call_tool(tool_name, &arguments);

                if result.get("error").is_some() {
                    json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "error": result["error"]
                    })
                } else {
                    json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": result
                    })
                }
            }

            "completion/complete" => {
                let ref_type = req.get("params")
                    .and_then(|p| p.get("ref"))
                    .and_then(|r| r.get("type"))
                    .and_then(Value::as_str)
                    .unwrap_or("");

                let name_or_uri = req.get("params")
                    .and_then(|p| p.get("ref"))
                    .and_then(|r| r.get("name").or_else(|| r.get("uri")))
                    .and_then(Value::as_str)
                    .unwrap_or("");

                let argument_name = req.get("params")
                    .and_then(|p| p.get("argument"))
                    .and_then(|a| a.get("name"))
                    .and_then(Value::as_str)
                    .unwrap_or("");

                let argument_value = req.get("params")
                    .and_then(|p| p.get("argument"))
                    .and_then(|a| a.get("value"))
                    .and_then(Value::as_str)
                    .unwrap_or("");

                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": utilities::completion::complete(ref_type, name_or_uri, argument_name, argument_value)
                })
            }

            "logging/setLevel" => {
                let level = req.get("params")
                    .and_then(|p| p.get("level"))
                    .and_then(Value::as_str)
                    .unwrap_or("info");

                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": utilities::logging::set_log_level(level)
                })
            }

            "shutdown" => {
                let response = json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": null
                });
                writeln!(stdout, "{}", response)?;
                stdout.flush()?;
                eprintln!("Server shutting down...");
                break;
            }

            "notifications/initialized" | "notifications/cancelled" => {
                // Notifications don't need responses
                continue;
            }

            _ => {
                eprintln!("Unsupported method: {}", method);
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "error": {
                        "code": -32601,
                        "message": format!("Method not found: {}", method)
                    }
                })
            }
        };

        // Only send response for requests (not notifications)
        if !method.starts_with("notifications/") {
            writeln!(stdout, "{}", resp)?;
            stdout.flush()?;
        }
    }

    Ok(())
}
