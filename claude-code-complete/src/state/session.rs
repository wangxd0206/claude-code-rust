//! Session Management - Original: sessionDiscovery.ts + sessionHistory.ts
//!
//! Manages user sessions including conversation history,
//! tool use tracking, and session persistence.

use crate::events::types::{ChatMessage, MessageRole, SessionEvent};
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use uuid::Uuid;

/// A single user session
#[derive(Debug, Clone)]
pub struct Session {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub messages: Arc<RwLock<Vec<ChatMessage>>>,
    pub metadata: SessionMetadata,
    pub is_active: bool,
}

#[derive(Debug, Clone, Default)]
pub struct SessionMetadata {
    pub title: Option<String>,
    pub project_path: Option<String>,
    pub model: String,
    pub token_count: usize,
    pub cost: f64,
}

impl Session {
    /// Create a new session
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            messages: Arc::new(RwLock::new(Vec::new())),
            metadata: SessionMetadata {
                model: "claude-3-opus-20240229".to_string(),
                ..Default::default()
            },
            is_active: true,
        }
    }

    /// Add a message to the session
    pub async fn add_message(&self, role: MessageRole, content: String) {
        let message = ChatMessage {
            id: Uuid::new_v4(),
            role,
            content,
            timestamp: Utc::now(),
            metadata: None,
        };

        let mut messages = self.messages.write().await;
        messages.push(message);

        // Update token count estimate
        let token_estimate = content.split_whitespace().count();
        self.metadata.token_count += token_estimate;

        debug!("Added message to session {}", self.id);
    }

    /// Get all messages
    pub async fn get_messages(&self) -> Vec<ChatMessage> {
        let messages = self.messages.read().await;
        messages.clone()
    }

    /// Get message count
    pub async fn message_count(&self) -> usize {
        let messages = self.messages.read().await;
        messages.len()
    }

    /// End the session
    pub fn end(&mut self) {
        self.is_active = false;
        self.updated_at = Utc::now();
        info!("Session {} ended", self.id);
    }

    /// Generate session title from first message
    pub async fn generate_title(&mut self) {
        let messages = self.messages.read().await;
        if let Some(first) = messages.first() {
            let title = if first.content.len() > 50 {
                format!("{}...", &first.content[..50])
            } else {
                first.content.clone()
            };
            self.metadata.title = Some(title);
        }
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}

/// Manages all sessions
#[derive(Debug)]
pub struct SessionManager {
    /// Active sessions (session_id -> Session)
    sessions: DashMap<Uuid, Arc<Session>>,

    /// Currently active session ID
    current_session: Arc<RwLock<Option<Uuid>>>,

    /// Session history (for recovery)
    session_history: Arc<RwLock<Vec<Session>>>,
}

impl SessionManager {
    /// Create a new session manager
    pub fn new() -> Self {
        Self {
            sessions: DashMap::new(),
            current_session: Arc::new(RwLock::new(None)),
            session_history: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Create a new session and set it as current
    pub async fn create_session(&self) -> Arc<Session> {
        let session = Arc::new(Session::new());
        let id = session.id;

        self.sessions.insert(id, session.clone());

        let mut current = self.current_session.write().await;
        *current = Some(id);

        info!("Created new session {}", id);
        session
    }

    /// Get the current session
    pub async fn current_session(&self) -> Option<Arc<Session>> {
        let current = self.current_session.read().await;
        current.and_then(|id| self.sessions.get(&id).map(|s| s.clone()))
    }

    /// Switch to a different session
    pub async fn switch_session(&self, session_id: Uuid) -> Result<(), String> {
        if self.sessions.contains_key(&session_id) {
            let mut current = self.current_session.write().await;
            *current = Some(session_id);
            info!("Switched to session {}", session_id);
            Ok(())
        } else {
            Err(format!("Session {} not found", session_id))
        }
    }

    /// End a session
    pub async fn end_session(&self, session_id: Uuid) {
        if let Some(entry) = self.sessions.get(&session_id) {
            let session = entry.clone();
            drop(entry);

            // Move to history
            let history = Session {
                id: session.id,
                created_at: session.created_at,
                updated_at: session.updated_at,
                messages: Arc::clone(&session.messages),
                metadata: session.metadata.clone(),
                is_active: false,
            };

            self.sessions.remove(&session_id);

            let mut session_history = self.session_history.write().await;
            session_history.push(history);

            // Clear current if it was this session
            let mut current = self.current_session.write().await;
            if *current == Some(session_id) {
                *current = None;
            }

            info!("Ended session {}", session_id);
        }
    }

    /// List all active sessions
    pub fn list_active_sessions(&self) -> Vec<Arc<Session>> {
        self.sessions
            .iter()
            .map(|entry| entry.value().clone())
            .collect()
    }

    /// Get session history
    pub async fn get_history(&self) -> Vec<Session> {
        let history = self.session_history.read().await;
        history.clone()
    }

    /// Clear all sessions
    pub async fn clear_all(&self) {
        self.sessions.clear();
        let mut current = self.current_session.write().await;
        *current = None;
        let mut history = self.session_history.write().await;
        history.clear();
        warn!("All sessions cleared");
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}
