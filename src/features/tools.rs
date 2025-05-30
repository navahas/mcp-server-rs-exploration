use serde_json::{json, Value};

pub fn list_tools() -> Value {
    json!({
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
    })
}

pub fn call_tool(name: &str, arguments: &Value) -> Value {
    match name {
        "add_numbers" => {
            let sum: f64 = arguments.get("numbers")
                .and_then(Value::as_array)
                .unwrap_or(&vec![])
                .iter()
                .filter_map(Value::as_f64)
                .sum();

            json!({
                "content": [{
                    "type": "text",
                    "text": format!("The sum is: {}", sum)
                }]
            })
        }

        "echo_text" => {
            let text = arguments.get("text")
                .and_then(Value::as_str)
                .unwrap_or("");

            json!({
                "content": [{
                    "type": "text",
                    "text": text
                }]
            })
        }

        "reverse_string" => {
            let text = arguments.get("text")
                .and_then(Value::as_str)
                .unwrap_or("");

            let reversed: String = text.chars().rev().collect();

            json!({
                "content": [{
                    "type": "text",
                    "text": reversed
                }]
            })
        }

        _ => {
            json!({
                "error": {
                    "code": -32601,
                    "message": format!("Tool not found: {}", name)
                }
            })
        }
    }
}
