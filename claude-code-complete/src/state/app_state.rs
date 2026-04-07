//! Application State - Central state container
//!
//! Matches original externalMetadataToAppState function
//! for converting external state to internal app state.

use crate::events::types::{Event, SidebarTab, Theme};
use crate::state::{Session, SessionManager};
use dashmap::DashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Global application state
#[derive(Debug, Clone)]
pub struct AppState {
    /// Current working directory
    pub current_dir: Arc<RwLock<PathBuf>>,

    /// Current theme
    pub theme: Arc<RwLock<Theme>>,

    /// Current sidebar tab
    pub current_tab: Arc<RwLock<SidebarTab>>,

    /// Input text
    pub input_text: Arc<RwLock<String>>,

    /// Scroll positions for panels
    pub scroll_positions: Arc<DashMap<String, u16>>,

    /// Configuration
    pub config: Arc<RwLock<AppConfig>>,

    /// Runtime flags
    pub flags: RuntimeFlags,

    /// Statistics
    pub stats: AppStats,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
    pub max_tokens: u32,
    pub temperature: f32,
    pub auto_save: bool,
    pub theme: Theme,
    pub sidebar_width: u16,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            base_url: "https://api.anthropic.com".to_string(),
            model: "claude-3-opus-20240229".to_string(),
            max_tokens: 4096,
            temperature: 0.7,
            auto_save: true,
            theme: Theme::Dark,
            sidebar_width: 40,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct RuntimeFlags {
    pub verbose: bool,
    pub no_interactive: bool,
    pub experimental: bool,
}

#[derive(Debug, Clone, Default)]
pub struct AppStats {
    pub messages_sent: usize,
    pub messages_received: usize,
    pub tokens_used: usize,
    pub api_calls: usize,
    pub tool_executions: usize,
    pub errors: usize,
}

impl AppState {
    /// Create new application state
    pub fn new() -> Self {
        Self {
            current_dir: Arc::new(RwLock::new(std::env::current_dir().unwrap_or_default())),
            theme: Arc::new(RwLock::new(Theme::Dark)),
            current_tab: Arc::new(RwLock::new(SidebarTab::Chat)),
            input_text: Arc::new(RwLock::new(String::new())),
            scroll_positions: Arc::new(DashMap::new()),
            config: Arc::new(RwLock::new(AppConfig::default())),
            flags: RuntimeFlags::default(),
            stats: AppStats::default(),
        }
    }

    /// Update current directory
    pub async fn set_current_dir(&self, path: PathBuf) {
        let mut dir = self.current_dir.write().await;
        *dir = path;
    }

    /// Get current directory
    pub async fn get_current_dir(&self) -> PathBuf {
        self.current_dir.read().await.clone()
    }

    /// Set theme
    pub async fn set_theme(&self, theme: Theme) {
        let mut t = self.theme.write().await;
        *t = theme;
    }

    /// Get theme
    pub async fn get_theme(&self) -> Theme {
        *self.theme.read().await
    }

    /// Switch sidebar tab
    pub async fn switch_tab(&self, tab: SidebarTab) {
        let mut current = self.current_tab.write().await;
        *current = tab;
    }

    /// Get current tab
    pub async fn get_tab(&self) -> SidebarTab {
        *self.current_tab.read().await
    }

    /// Update input text
    pub async fn set_input(&self, text: String) {
        let mut input = self.input_text.write().await;
        *input = text;
    }

    /// Get input text
    pub async fn get_input(&self) -> String {
        self.input_text.read().await.clone()
    }

    /// Append to input
    pub async fn append_input(&self, text: &str) {
        let mut input = self.input_text.write().await;
        input.push_str(text);
    }

    /// Get scroll position
    pub fn get_scroll(&self, panel: &str) -> u16 {
        self.scroll_positions.get(panel).map(|v| *v).unwrap_or(0)
    }

    /// Set scroll position
    pub fn set_scroll(&self, panel: &str, position: u16) {
        self.scroll_positions.insert(panel.to_string(), position);
    }

    /// Update configuration
    pub async fn update_config(&self, config: AppConfig) {
        let mut c = self.config.write().await;
        *c = config;
    }

    /// Get configuration
    pub async fn get_config(&self) -> AppConfig {
        self.config.read().await.clone()
    }

    /// Increment stat
    pub fn increment_stat(&mut self, stat: &str) {
        match stat {
            "messages_sent" => self.stats.messages_sent += 1,
            "messages_received" => self.stats.messages_received += 1,
            "tokens_used" => self.stats.tokens_used += 1,
            "api_calls" => self.stats.api_calls += 1,
            "tool_executions" => self.stats.tool_executions += 1,
            "errors" => self.stats.errors += 1,
            _ => {}
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

/// Global state container for the entire application
#[derive(Debug)]
pub struct GlobalState {
    pub app: AppState,
    pub session_manager: SessionManager,
    pub current_session_id: Arc<RwLock<Option<Uuid>>>,
}

impl GlobalState {
    pub fn new() -> Self {
        Self {
            app: AppState::new(),
            session_manager: SessionManager::new(),
            current_session_id: Arc::new(RwLock::new(None)),
        }
    }

    /// Get current session
    pub async fn current_session(&self) -> Option<Arc<Session>> {
        self.session_manager.current_session().await
    }
}

impl Default for GlobalState {
    fn default() -> Self {
        Self::new()
    }
}
