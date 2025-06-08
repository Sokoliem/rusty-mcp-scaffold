# Rusty MCP Scaffold

A minimal Rust-based Model Context Protocol (MCP) server scaffold for quickly bootstrapping MCP servers that integrate with Claude Desktop.

## Features

This scaffold provides a working MCP server implementation with:

- 🦀 **Pure Rust Implementation** using the `rmcp` crate
- 🔧 **Example Tools**: Echo, Calculator, and Stats tools for reference
- 📝 **Comprehensive Logging**: File and console logging with configurable levels
- 🚀 **Easy Setup**: Pre-configured for Claude Desktop integration
- 📦 **Minimal Dependencies**: Only essential crates included

## Quick Start

1. **Clone the repository**
   ```bash
   git clone https://github.com/Sokoliem/rusty-mcp-scaffold.git
   cd rusty-mcp-scaffold
   ```

2. **Build the server**
   ```bash
   cargo build --release
   ```

3. **Configure Claude Desktop**
   
   Add to your `claude_desktop_config.json`:
   ```json
   "rusty-server": {
     "command": "C:\\path\\to\\rusty-mcp-scaffold\\target\\release\\rusty-server.exe",
     "args": [],
     "env": {
       "RUST_LOG": "debug",
       "RUST_BACKTRACE": "1"
     }
   }
   ```

4. **Restart Claude Desktop** and your server should be available!

## Project Structure

```
rusty-mcp-scaffold/
├── Cargo.toml          # Project dependencies
├── build.rs            # Build script for compile-time info
├── src/
│   ├── main.rs         # Server entry point and logging setup
│   └── server.rs       # MCP server implementation and tools
├── logs/               # Generated log files (gitignored)
└── target/             # Build artifacts (gitignored)
```

## Available Tools

### 1. Echo Tool
Echoes back any message sent to it.
```
Parameters:
- message (string): The message to echo back
```

### 2. Calculator Tool
Performs basic arithmetic operations.
```
Parameters:
- operation (string): "add", "subtract", "multiply", or "divide"
- a (number): First number
- b (number): Second number
```

### 3. Get Stats Tool
Returns server statistics including request count.
```
Parameters: None
```

## Extending the Server

To add new tools:

1. Add a new method to the `RustyServer` implementation in `src/server.rs`:
   ```rust
   #[tool(description = "Your tool description")]
   async fn your_tool_name(
       &self,
       #[tool(param)]
       #[schemars(description = "Parameter description")]
       param_name: String,
   ) -> Result<CallToolResult, McpError> {
       // Your implementation
       Ok(CallToolResult::success(vec![Content::text("Result")]))
   }
   ```

2. Rebuild the server:
   ```bash
   cargo build --release
   ```

3. Restart Claude Desktop to use the new tool

## Logging

Logs are written to:
- **Console**: stderr output for immediate feedback
- **Files**: `logs/rusty_server_YYYYMMDD_HHMMSS.log` for detailed debugging

Configure logging verbosity with the `RUST_LOG` environment variable:
- `error`: Only errors
- `warn`: Warnings and errors
- `info`: General information
- `debug`: Debug information
- `trace`: Very detailed trace information

## Development

### Prerequisites
- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- Windows, macOS, or Linux

### Testing Standalone
```bash
# Run with default logging
cargo run

# Run with trace logging
RUST_LOG=trace cargo run

# Run the release build
./target/release/rusty-server
```

### Building for Distribution
```bash
# Build optimized binary
cargo build --release

# Binary will be at target/release/rusty-server (or .exe on Windows)
```

## Troubleshooting

1. **Server doesn't appear in Claude Desktop**
   - Ensure Claude Desktop is fully restarted
   - Check the path in your config is correct
   - Look for errors in the log files

2. **Tools not working**
   - Check `logs/` directory for error messages
   - Verify the server is running with `test_server.bat` (Windows)
   - Ensure RUST_LOG is set for debugging

3. **Build failures**
   - Update Rust: `rustup update`
   - Clean build: `cargo clean && cargo build --release`

## Dependencies

- `rmcp`: MCP protocol implementation
- `tokio`: Async runtime
- `tracing`: Structured logging
- `serde`/`serde_json`: JSON serialization
- `schemars`: JSON schema generation

## License

MIT License - feel free to use this scaffold for your own MCP servers!

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Resources

- [MCP Documentation](https://modelcontextprotocol.io/)
- [rmcp Crate](https://crates.io/crates/rmcp)
- [Claude Desktop](https://claude.ai/download)
