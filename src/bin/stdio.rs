use mcp_server::transport::stdio;

fn main() {
    stdio::run_stdio_server().unwrap();
}
