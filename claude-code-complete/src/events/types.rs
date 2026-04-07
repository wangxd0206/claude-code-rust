//! Event Types - Matching original Claude Code event system
//!
//! These types mirror the original SDKMessage, StdinMessage types
//! used in structuredIO.ts for communication between components.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Main event type for the application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    // UI Events
    Ui(UiEvent),

    // Core Events
    Core(CoreEvent),

    // Session Events
    Session(SessionEvent),

    // Tool Events
    Tool(ToolEvent),

    // MCP Events
    Mcp(McpEvent),

    // Permission Events
    Permission(PermissionEvent),

    // System Events
    System(SystemEvent),
}

/// UI-related events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UiEvent {
    /// User clicked somewhere
    Click { x: u16, y: u16 },

    /// User pressed a key
    Key(KeyEvent),

    /// Resize event
    Resize { width: u16, height: u16 },

    /// Focus changed
    Focus(FocusTarget),

    /// Scroll event
    Scroll { direction: ScrollDirection, amount: u16 },

    /// Hover event
    Hover { x: u16, y: u16, target: String },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum KeyEvent {
    Char(char),
    Enter,
    Tab,
    Backspace,
    Escape,
    Up,
    Down,
    Left,
    Right,
    Ctrl(char),
    Shift(KeyEvent),
    Alt(KeyEvent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FocusTarget {
    Input,
    Sidebar(SidebarTab),
    Chat,
    Settings,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SidebarTab {
    Chat,
    Files,
    Git,
    Mcp,
    Settings,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ScrollDirection {
    Up,
    Down,
    Left,
    Right,
}

/// Core application events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoreEvent {
    /// Application initialized
    Init,

    /// Configuration changed
    ConfigChanged { key: String, value: serde_json::Value },

    /// Error occurred
    Error { message: String, source: Option<String> },

    /// State updated
    StateUpdated { key: String, value: serde_json::Value },

    /// Theme changed
    ThemeChanged(Theme),

    /// Notifications
    Notification { level: NotificationLevel, message: String },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Theme {
    Dark,
    Light,
    System,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum NotificationLevel {
    Info,
    Success,
    Warning,
    Error,
}

/// Session events - matching original session lifecycle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionEvent {
    /// New session started
    Started {
        session_id: Uuid,
        timestamp: DateTime<Utc>,
    },

    /// Session resumed
    Resumed {
        session_id: Uuid,
    },

    /// Session ended
    Ended {
        session_id: Uuid,
        reason: String,
    },

    /// Message sent by user
    UserMessage {
        session_id: Uuid,
        message: ChatMessage,
    },

    /// Message received from assistant
    AssistantMessage {
        session_id: Uuid,
        message: ChatMessage,
    },

    /// Streaming response started
    StreamStarted {
        session_id: Uuid,
        message_id: Uuid,
    },

    /// Streaming response chunk
    StreamChunk {
        session_id: Uuid,
        message_id: Uuid,
        content: String,
    },

    /// Streaming response ended
    StreamEnded {
        session_id: Uuid,
        message_id: Uuid,
    },

    /// Tool use requested
    ToolUseRequested {
        session_id: Uuid,
        tool_use: ToolUseRequest,
    },

    /// Tool use result
    ToolUseResult {
        session_id: Uuid,
        tool_use_id: Uuid,
        result: ToolUseResult,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: Uuid,
    pub role: MessageRole,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolUseRequest {
    pub id: Uuid,
    pub name: String,
    pub input: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToolUseResult {
    Success { content: String },
    Error { message: String },
}

/// Tool-related events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToolEvent {
    /// Tool registered
    Registered { name: String, description: String },

    /// Tool unregistered
    Unregistered { name: String },

    /// Tool execution started
    ExecutionStarted {
        tool_name: String,
        tool_use_id: Uuid,
    },

    /// Tool execution completed
    ExecutionCompleted {
        tool_name: String,
        tool_use_id: Uuid,
        duration_ms: u64,
    },

    /// Tool execution failed
    ExecutionFailed {
        tool_name: String,
        tool_use_id: Uuid,
        error: String,
    },
}

/// MCP (Model Context Protocol) events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum McpEvent {
    /// Server connected
    ServerConnected { server_id: String },

    /// Server disconnected
    ServerDisconnected { server_id: String, reason: Option<String> },

    /// Tool registered from MCP
    ToolRegistered { server_id: String, tool: McpToolInfo },

    /// Resource requested
    ResourceRequested {
        server_id: String,
        uri: String,
    },

    /// Prompt requested
    PromptRequested {
        server_id: String,
        name: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpToolInfo {
    pub name: String,
    pub description: Option<String>,
    pub input_schema: serde_json::Value,
}

/// Permission events - matching original permission system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PermissionEvent {
    /// Permission requested
    Requested {
        request_id: Uuid,
        tool_name: String,
        description: String,
    },

    /// Permission granted
    Granted { request_id: Uuid },

    /// Permission denied
    Denied { request_id: Uuid, reason: Option<String> },

    /// Permission mode changed
    ModeChanged { mode: PermissionMode },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PermissionMode {
    Auto,
    Confirm,
    Never,
}

/// System events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemEvent {
    /// Shutdown requested
    Shutdown,

    /// Reload configuration
    ReloadConfig,

    /// Memory usage update
    MemoryUsage { used: u64, total: u64 },

    /// File watcher event
    FileWatcher { path: String, event: FileWatcherEvent },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileWatcherEvent {
    Created,
    Modified,
    Deleted,
    Renamed { from: String },
}
