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

            "tools/list" => {
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "tools": [
                            {
                                "name": "add_numbers",
                                "description": "Add a list of numbers together",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {
                                        "numbers": {
                                            "type": "array",
                                            "items": {
                                                "type": "number"
                                            },
                                            "description": "Array of numbers to add"
                                        }
                                    },
                                    "required": ["numbers"]
                                }
                            },
                            {
                                "name": "echo_text",
                                "description": "Echo back the provided text",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {
                                        "text": {
                                            "type": "string",
                                            "description": "Text to echo back"
                                        }
                                    },
                                    "required": ["text"]
                                }
                            },
                            {
                                "name": "reverse_string",
                                "description": "Reverse the characters in a string",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {
                                        "text": {
                                            "type": "string",
                                            "description": "String to reverse"
                                        }
                                    },
                                    "required": ["text"]
                                }
                            }
                        ]
                    }
                })
            }

            "resources/list" => {
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "resources": []
                    }
                })
            }

            "prompts/list" => {
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "prompts": []
                    }
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

                match tool_name {
                    "add_numbers" => {
                        let sum: i64 = req.get("params")
                            .and_then(Value::as_array)
                            .unwrap_or(&vec![])
                            .iter()
                            .filter_map(Value::as_i64)
                            .sum();
                        
                        json!({
                            "jsonrpc": "2.0",
                            "id": id,
                            "result": {
                                "content": [
                                    {
                                        "type": "text",
                                        "text": format!("The sum is: {}", sum)
                                    }
                                ]
                            }
                        })
                    }
                    
                    "echo_text" => {
                        let text = arguments.get("text")
                            .and_then(Value::as_str)
                            .unwrap_or("");
                        
                        json!({
                            "jsonrpc": "2.0",
                            "id": id,
                            "result": {
                                "content": [
                                    {
                                        "type": "text",
                                        "text": text
                                    }
                                ]
                            }
                        })
                    }
                    
                    "reverse_string" => {
                        let text = arguments.get("text")
                            .and_then(Value::as_str)
                            .unwrap_or("");
                        
                        let reversed: String = text.chars().rev().collect();
                        
                        json!({
                            "jsonrpc": "2.0",
                            "id": id,
                            "result": {
                                "content": [
                                    {
                                        "type": "text",
                                        "text": reversed
                                    }
                                ]
                            }
                        })
                    }
                    
                    _ => {
                        json!({
                            "jsonrpc": "2.0",
                            "id": id,
                            "error": {
                                "code": -32601,
                                "message": format!("Tool not found: {}", tool_name)
                            }
                        })
                    }
                }
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
