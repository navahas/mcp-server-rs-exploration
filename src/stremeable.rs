use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use serde_json::{json, Value};

const RS: char = 0x1E as char; // Record Separator for json-seq

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3333")?;
    eprintln!("MCP streaming server listening on http://127.0.0.1:3333");

    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            std::thread::spawn(move || {
                if let Err(e) = handle_streaming_connection(&mut stream) {
                    eprintln!("Connection error: {}", e);
                }
            });
        }
    }

    Ok(())
}

fn handle_streaming_connection(stream: &mut TcpStream) -> std::io::Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);

    // Read and discard HTTP request headers
    let mut request_line = String::new();
    reader.read_line(&mut request_line)?;

    if !request_line.starts_with("POST") {
        write_http_response(stream, 405, "Method Not Allowed", "Only POST allowed")?;
        return Ok(());
    }

    let mut headers = String::new();
    loop {
        let mut line = String::new();
        reader.read_line(&mut line)?;
        if line == "\r\n" || line.is_empty() {
            break;
        }
        headers.push_str(&line);
    }

    if !headers.contains("application/json-seq") {
        write_http_response(stream, 415, "Unsupported Media Type", "Expected application/json-seq")?;
        return Ok(());
    }

    // Send initial 200 OK response with chunked transfer encoding
    let mut writer = stream;
    write!(
        writer,
        "HTTP/1.1 200 OK\r\nContent-Type: application/json-seq\r\nTransfer-Encoding: chunked\r\n\r\n"
    )?;
    writer.flush()?;

    // Begin streaming: read and respond to each JSON message
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line)? == 0 {
            break; // client disconnected
        }

        if let Some(json_str) = line.strip_prefix(RS) {
            let req: Value = match serde_json::from_str(json_str.trim()) {
                Ok(val) => val,
                Err(e) => {
                    send_chunked_json(&mut writer, &json!({
                        "jsonrpc": "2.0",
                        "id": null,
                        "error": {
                            "code": -32700,
                            "message": format!("Parse error: {}", e)
                        }
                    }))?;
                    continue;
                }
            };

            let method = req.get("method").and_then(|v| v.as_str()).unwrap_or("");
            let id = req.get("id").cloned().unwrap_or(json!(null));
            let params = req.get("params").cloned().unwrap_or(json!({}));

            let response = match method {
                "initialize" => {
                    let version = params.get("protocolVersion").and_then(Value::as_str).unwrap_or("2024-11-05");
                    json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": {
                            "protocolVersion": version,
                            "capabilities": { "tools": { "listChanged": true } },
                            "serverInfo": {
                                "name": "rust-streaming-server",
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
                                                "items": { "type": "number" },
                                                "description": "Array of numbers to add"
                                            }
                                        },
                                        "required": ["numbers"]
                                    }
                                }
                            ]
                        }
                    })
                }
                "tools/call" => {
                    let name = params.get("name").and_then(Value::as_str).unwrap_or("");
                    let args = params.get("arguments").cloned().unwrap_or(json!({}));
                    match name {
                        "add_numbers" => {
                            let sum: i64 = args.get("numbers")
                                .and_then(Value::as_array)
                                .unwrap_or(&vec![])
                                .iter()
                                .filter_map(Value::as_i64)
                                .sum();

                            json!({
                                "jsonrpc": "2.0",
                                "id": id,
                                "result": {
                                    "content": [{
                                        "type": "text",
                                        "text": format!("The sum is: {}", sum)
                                    }]
                                }
                            })
                        }
                        _ => json!({
                            "jsonrpc": "2.0",
                            "id": id,
                            "error": {
                                "code": -32601,
                                "message": format!("Unknown tool: {}", name)
                            }
                        })
                    }
                }
                _ => json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "error": {
                        "code": -32601,
                        "message": format!("Unknown method: {}", method)
                    }
                })
            };

            send_chunked_json(&mut writer, &response)?;
        }
    }

    Ok(())
}

fn write_http_response(stream: &mut TcpStream, code: u16, status: &str, body: &str) -> std::io::Result<()> {
    write!(
        stream,
        "HTTP/1.1 {} {}\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{}",
        code,
        status,
        body.len(),
        body
    )?;
    stream.flush()
}

fn send_chunked_json<W: Write>(mut writer: W, json_value: &Value) -> std::io::Result<()> {
    let msg = format!("{}{}", RS, json_value.to_string());
    let msg_bytes = msg.as_bytes();
    let chunk_header = format!("{:X}\r\n", msg_bytes.len());

    writer.write_all(chunk_header.as_bytes())?;
    writer.write_all(msg_bytes)?;
    writer.write_all(b"\r\n")?;
    writer.flush()
}
