# MCP Server Exploration

This project is an exploration of the latest MCP specification revision:
- [MCP Spec – March 26, 2025](https://modelcontextprotocol.io/specification/2025-03-26/basic/lifecycle)

## Building the Server

To build the server binary:

```sh cargo build --release && cp ./target/release/stdio ./stdio ```

## Integrating with Claude Desktop

To use this server with Claude Desktop, add the executable path to your Claude
Desktop configuration:

```json
{
    "mcpServers": {
        "rust_math_server": {
            "command": "$PATH/TO/mcp-server-rs-exploration/stdio",
                "args": []
        }
    }
}
```

Replace $PATH/TO with the actual path to your local clone of this project.
