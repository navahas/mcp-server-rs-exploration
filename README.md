# MCP Server Exploration

For hosts like Claude Desktop to execute, the binary must be provided.
```
cargo build --release &&  cp ./target/release/stdio ./stdio
```

Then make sure you add the path in yourc Claude Desktop config:
```json
{
    "mcpServers": {
        "rust_math_server": {
            "command": "$PATH/TO/mcp-server-rs-exploration/stdio",
            "args" : []
        }
    }
}
```
