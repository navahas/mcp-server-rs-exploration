use std::io::{self, BufRead, Write};
use serde_json::Value;
use crate::dispatcher;

pub fn run_stdio_server() -> io::Result<()> {
    eprintln!("Starting MCP server (stdio)...");

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

        let resp_opt = dispatcher::handle_request(req);

        if let Some(resp) = resp_opt {
            writeln!(stdout, "{}", resp)?;
            stdout.flush()?;

            if resp.get("result") == Some(&Value::Null) && resp.get("id").is_some() {
                eprintln!("Server shutting down...");
                break;
            }
        }
    }

    Ok(())
}
