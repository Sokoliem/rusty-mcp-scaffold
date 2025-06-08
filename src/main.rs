use anyhow::Result;
use rmcp::{ServiceExt, transport::stdio};
use std::fs::OpenOptions;
use tracing_subscriber::{self, EnvFilter, fmt, prelude::*};
use chrono::Local;

mod server;
use server::RustyServer;

fn setup_logging() -> Result<()> {
    // Create a log file with timestamp
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let log_file_path = format!("C:/rusty-server/logs/rusty_server_{}.log", timestamp);
    
    // Create logs directory if it doesn't exist
    std::fs::create_dir_all("C:/rusty-server/logs")?;
    
    // Create log file
    let log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(&log_file_path)?;
    
    // Create a multi-writer that writes to both stderr and file
    let (non_blocking, _guard) = tracing_appender::non_blocking(log_file);
    
    // Set up the subscriber with multiple layers
    let file_layer = fmt::layer()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .with_level(true)
        .pretty();
    
    let stderr_layer = fmt::layer()
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .with_level(true)
        .compact();
    
    // Configure the subscriber
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("debug,rmcp=debug,rusty_server=trace"))
        )
        .with(file_layer)
        .with(stderr_layer)
        .init();
    
    // Log startup information
    tracing::info!("=== Rusty MCP Server Starting ===");
    tracing::info!("Log file: {}", log_file_path);
    tracing::info!("Process ID: {}", std::process::id());
    tracing::info!("Rust version: {}", env!("RUSTC_VERSION"));
    tracing::debug!("Environment variables:");
    for (key, value) in std::env::vars() {
        if key.starts_with("RUST") || key.starts_with("MCP") {
            tracing::debug!("  {} = {}", key, value);
        }
    }
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Set up comprehensive logging
    setup_logging()?;
    
    tracing::info!("Starting Rusty MCP server main function");
    
    // Log command line arguments
    let args: Vec<String> = std::env::args().collect();
    tracing::debug!("Command line arguments: {:?}", args);
    
    // Create server instance
    tracing::info!("Creating RustyServer instance");
    let server = RustyServer::new();
    
    // Set up panic hook for better error logging
    std::panic::set_hook(Box::new(|panic_info| {
        let payload = panic_info.payload();
        let location = panic_info.location()
            .map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()))
            .unwrap_or_else(|| "unknown location".to_string());
        
        let msg = if let Some(s) = payload.downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = payload.downcast_ref::<String>() {
            s.clone()
        } else {
            "Unknown panic payload".to_string()
        };
        
        tracing::error!("PANIC at {}: {}", location, msg);
    }));
    
    // Start the server
    tracing::info!("Starting stdio transport");
    match server.serve(stdio()).await {
        Ok(service) => {
            tracing::info!("Server successfully initialized, waiting for requests...");
            
            // Wait for the service to complete
            match service.waiting().await {
                Ok(_) => {
                    tracing::info!("Server shutting down gracefully");
                }
                Err(e) => {
                    tracing::error!("Server error while waiting: {:?}", e);
                    return Err(e.into());
                }
            }
        }
        Err(e) => {
            tracing::error!("Failed to start server: {:?}", e);
            tracing::error!("Error chain: {:#}", e);
            return Err(e.into());
        }
    }
    
    tracing::info!("Server stopped");
    Ok(())
}
