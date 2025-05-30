use serde_json::{json, Value};

pub fn list_prompts() -> Value {
    json!({
        "prompts": [
            {
                "name": "summarize_text",
                "description": "Summarize the provided text content",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "text": {
                            "type": "string",
                            "description": "Text to summarize"
                        }
                    },
                    "required": ["text"]
                }
            },
            {
                "name": "translate_text",
                "description": "Translate text to a specified language",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "text": {
                            "type": "string",
                            "description": "Text to translate"
                        },
                        "language": {
                            "type": "string",
                            "description": "Target language code (e.g., 'es', 'fr')"
                        }
                    },
                    "required": ["text", "language"]
                }
            }
        ]
    })
}

pub fn call_prompt(name: &str, arguments: &Value) -> Value {
    match name {
        "summarize_text" => {
            let text = arguments.get("text")
                .and_then(Value::as_str)
                .unwrap_or("");

            // Placeholder summary logic
            let summary = if text.len() > 100 {
                format!("Summary: {}...", &text[..100])
            } else {
                format!("Summary: {}", text)
            };

            json!({
                "content": [
                    {
                        "type": "text",
                        "text": summary
                    }
                ]
            })
        }

        "translate_text" => {
            let text = arguments.get("text")
                .and_then(Value::as_str)
                .unwrap_or("");
            let language = arguments.get("language")
                .and_then(Value::as_str)
                .unwrap_or("en");

            // Placeholder translation logic
            let translated = format!("Translated '{}' to [{}]: {}", text, language, text);

            json!({
                "content": [
                    {
                        "type": "text",
                        "text": translated
                    }
                ]
            })
        }

        _ => {
            json!({
                "error": {
                    "code": -32601,
                    "message": format!("Prompt not found: {}", name)
                }
            })
        }
    }
}
