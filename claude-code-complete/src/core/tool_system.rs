//! Tool System - Original: Tool.ts + toolPool.ts
//!
//! Manages all available tools for the AI assistant,
//! including built-in tools and dynamically loaded tools.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use uuid::Uuid;

/// A tool that can be used by the assistant
#[derive(Debug, Clone)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
    pub handler: ToolHandler,
}

pub type ToolHandler = Arc<dyn Fn(serde_json::Value) -> Result<String, String> + Send + Sync>;

/// Tool system manages all available tools
#[derive(Debug)]
pub struct ToolSystem {
    tools: Arc<RwLock<HashMap<String, Tool>>>,
}

impl ToolSystem {
    pub fn new() -> Self {
        let tools = Arc::new(RwLock::new(HashMap::new()));

        let system = Self { tools };
        system.register_built_in_tools();
        system
    }

    fn register_built_in_tools(&self) {
        // Register built-in tools here
        // This would include: BashTool, ReadFileTool, WriteFileTool, etc.
    }

    pub async fn register_tool(&self, tool: Tool) {
        let mut tools = self.tools.write().await;
        info!("Registering tool: {}", tool.name);
        tools.insert(tool.name.clone(), tool);
    }

    pub async fn get_tool(&self, name: &str) -> Option<Tool> {
        let tools = self.tools.read().await;
        tools.get(name).cloned()
    }

    pub async fn list_tools(&self) -> Vec<Tool> {
        let tools = self.tools.read().await;
        tools.values().cloned().collect()
    }

    pub async fn execute_tool(&self, name: &str, input: serde_json::Value) -> Result<String, String> {
        let tool = self.get_tool(name).await
            .ok_or_else(|| format!("Tool '{}' not found", name))?;

        debug!("Executing tool: {} with input: {:?}", name, input);
        (tool.handler)(input)
    }
}

impl Default for ToolSystem {
    fn default() -> Self {
        Self::new()
    }
}
