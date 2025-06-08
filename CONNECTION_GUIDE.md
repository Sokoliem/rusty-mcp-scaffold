# Rusty MCP Server - Connection Guide

## Server Successfully Created!

Your Rust-based MCP server has been created and configured. Here's what was set up:

### Server Location
- **Project Directory**: `C:/rusty-server/`
- **Executable**: `C:/rusty-server/target/release/rusty-server.exe`
- **Logs Directory**: `C:/rusty-server/logs/`

### Available Tools
1. **echo** - Echoes back any message
   - Parameter: `message` (string)
   
2. **calculator** - Performs arithmetic operations
   - Parameters:
     - `operation` (string): add, subtract, multiply, divide
     - `a` (number): first number
     - `b` (number): second number
   
3. **get_stats** - Returns server statistics

### Testing the Server

To test the server standalone:
```bash
cd C:/rusty-server
./test_server.bat
```

### Connecting to Claude Desktop

1. **Configuration Added**: The server has been automatically added to your Claude Desktop configuration
2. **Restart Claude Desktop**: You need to restart Claude Desktop for the changes to take effect
3. **Verify Connection**: After restart, you should see "rusty-server" in the MCP tools list

### Debugging

If the server doesn't connect:

1. **Check Logs**: Look in `C:/rusty-server/logs/` for detailed debug information
2. **Test Standalone**: Run `test_server.bat` to verify the server starts correctly
3. **Common Issues**:
   - Ensure Claude Desktop is fully closed before restarting
   - Check Windows Defender/Antivirus isn't blocking the executable
   - Verify the path in the config file is correct

### Log Files

Logs are written with timestamps to:
- `C:/rusty-server/logs/rusty_server_YYYYMMDD_HHMMSS.log`

The logs include:
- All tool invocations
- Request/response details
- Error messages
- Performance metrics

### Next Steps

1. Restart Claude Desktop
2. In a new conversation, ask Claude to use the rusty-server tools
3. Example prompts:
   - "Use the echo tool from rusty-server to say hello"
   - "Use the calculator tool from rusty-server to add 42 and 58"
   - "Get the stats from rusty-server"

### Extending the Server

To add more functionality:
1. Edit `src/server.rs`
2. Add new tool methods with the `#[tool]` attribute
3. Rebuild with `cargo build --release`
4. Restart Claude Desktop

## Troubleshooting Commands

```powershell
# Check if server executable exists
dir C:\rusty-server\target\release\rusty-server.exe

# View latest log file
dir C:\rusty-server\logs\ | Sort-Object LastWriteTime -Descending | Select-Object -First 1

# Test server manually
C:\rusty-server\test_server.bat
```
