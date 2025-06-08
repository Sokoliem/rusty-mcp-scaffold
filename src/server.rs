use rmcp::{
    Error as McpError, RoleServer, ServerHandler, model::*, schemars, 
    service::RequestContext, tool,
};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CalculatorRequest {
    #[schemars(description = "The operation to perform: add, subtract, multiply, divide")]
    pub operation: String,
    #[schemars(description = "The first number")]
    pub a: f64,
    #[schemars(description = "The second number")]
    pub b: f64,
}

#[derive(Debug, Clone)]
pub struct RustyServer {
    request_count: Arc<Mutex<u64>>,
}

#[tool(tool_box)]
impl RustyServer {
    pub fn new() -> Self {
        tracing::debug!("Creating new RustyServer instance");
        Self {
            request_count: Arc::new(Mutex::new(0)),
        }
    }
    
    async fn increment_request_count(&self) {
        let mut count = self.request_count.lock().await;
        *count += 1;
        tracing::trace!("Request count incremented to: {}", *count);
    }
    
    #[tool(description = "Echo back the provided message")]
    async fn echo(
        &self,
        #[tool(param)]
        #[schemars(description = "The message to echo back")]
        message: String,
    ) -> Result<CallToolResult, McpError> {
        tracing::info!("Echo tool called with message: {}", message);
        self.increment_request_count().await;
        
        let response = format!("Echo: {}", message);
        tracing::debug!("Echo response: {}", response);
        
        Ok(CallToolResult::success(vec![Content::text(response)]))
    }
    
    #[tool(description = "Perform basic calculator operations")]
    async fn calculator(
        &self,
        #[tool(aggr)] request: CalculatorRequest,
    ) -> Result<CallToolResult, McpError> {
        tracing::info!(
            "Calculator tool called - operation: {}, a: {}, b: {}", 
            request.operation, request.a, request.b
        );
        self.increment_request_count().await;
        
        let result = match request.operation.to_lowercase().as_str() {
            "add" | "addition" | "+" => {
                let res = request.a + request.b;
                tracing::debug!("Addition result: {} + {} = {}", request.a, request.b, res);
                res
            },
            "subtract" | "subtraction" | "-" => {
                let res = request.a - request.b;
                tracing::debug!("Subtraction result: {} - {} = {}", request.a, request.b, res);
                res
            },
            "multiply" | "multiplication" | "*" => {
                let res = request.a * request.b;
                tracing::debug!("Multiplication result: {} * {} = {}", request.a, request.b, res);
                res
            },
            "divide" | "division" | "/" => {
                if request.b == 0.0 {
                    tracing::warn!("Division by zero attempted");
                    return Err(McpError::invalid_params(
                        "Division by zero is not allowed",
                        None,
                    ));
                }
                let res = request.a / request.b;
                tracing::debug!("Division result: {} / {} = {}", request.a, request.b, res);
                res
            },
            op => {
                tracing::error!("Unknown operation requested: {}", op);
                return Err(McpError::invalid_params(
                    "Unknown operation. Supported operations: add, subtract, multiply, divide",
                    None,
                ));
            }
        };
        
        let response = format!("{} {} {} = {}", request.a, request.operation, request.b, result);
        Ok(CallToolResult::success(vec![Content::text(response)]))
    }
    
    #[tool(description = "Get server statistics")]
    async fn get_stats(&self) -> Result<CallToolResult, McpError> {
        tracing::info!("Get stats tool called");
        let count = self.request_count.lock().await;
        
        let stats = format!(
            "Server Statistics:\n\
             - Total requests processed: {}\n\
             - Server version: {}\n\
             - Uptime: running",
            *count,
            env!("CARGO_PKG_VERSION")
        );
        
        tracing::debug!("Stats response: {}", stats);
        Ok(CallToolResult::success(vec![Content::text(stats)]))
    }
}

#[tool(tool_box)]
impl ServerHandler for RustyServer {
    fn get_info(&self) -> ServerInfo {
        tracing::info!("get_info called - returning server capabilities");
        
        let info = ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            server_info: Implementation {
                name: "rusty-server".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
            instructions: Some(
                "This is a minimal Rust MCP server for testing connectivity. \
                 It provides echo and calculator tools for basic operations."
                    .to_string()
            ),
        };
        
        tracing::debug!("Server info: {:?}", info);
        info
    }
    
    async fn initialize(
        &self,
        request: InitializeRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, McpError> {
        tracing::info!("Initialize request received");
        tracing::debug!("Client info: {:?}", request.client_info);
        tracing::debug!("Protocol version: {:?}", request.protocol_version);
        tracing::debug!("Capabilities: {:?}", request.capabilities);
        
        let result = Ok(self.get_info());
        tracing::info!("Initialize complete");
        result
    }
}
