@echo off
echo Testing Rusty MCP Server...
echo.
echo Press Ctrl+C to stop the server.
echo.
cd /d C:\rusty-server
set RUST_LOG=debug,rmcp=debug,rusty_server=trace
set RUST_BACKTRACE=1
target\release\rusty-server.exe
