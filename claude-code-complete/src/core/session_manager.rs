//! Session Manager - Core session handling
//!
//! Manages the lifecycle of user sessions, including:
//! - Session creation and destruction
//! - Message history
//! - Context management

use crate::events::types::{ChatMessage, SessionEvent};
use crate::events::EventBus;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use uuid::Uuid;

/// Manages all active sessions
#[derive(Debug)]
pub struct SessionManager {
    event_bus: EventBus,
    active_sessions: Arc<RwLock<Vec<Session>>>,
}

#[derive(Debug, Clone)]
pub struct Session {
    pub id: Uuid,
    pub messages: Vec<ChatMessage>,
    pub metadata: SessionMetadata,
}

#[derive(Debug, Clone)]
pub struct SessionMetadata {
    pub title: Option<String>,
    pub working_dir: String,
    pub model: String,
}

impl SessionManager {
    pub fn new(event_bus: EventBus) -> Self {
        Self {
            event_bus,
            active_sessions: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn create_session(&self,
        working_dir: String,
    ) -> Session {
        let session = Session {
            id: Uuid::new_v4(),
            messages: Vec::new(),
            metadata: SessionMetadata {
                title: None,
                working_dir,
                model: "claude-3-opus-20240229".to_string(),
            },
        };

        let mut sessions = self.active_sessions.write().await;
        sessions.push(session.clone());

        self.event_bus.emit(Event::Session(SessionEvent::Started {
            session_id: session.id,
            timestamp: chrono::Utc::now(),
        }));

        info!("Created new session {}", session.id);
        session
    }

    pub async fn get_session(&self,
        id: Uuid,
    ) -> Option<Session> {
        let sessions = self.active_sessions.read().await;
        sessions.iter().find(|s| s.id == id).cloned()
    }

    pub async fn end_session(&self,
        id: Uuid,
    ) {
        let mut sessions = self.active_sessions.write().await;
        sessions.retain(|s| s.id != id);

        self.event_bus.emit(Event::Session(SessionEvent::Ended {
            session_id: id,
            reason: "User requested".to_string(),
        }));

        info!("Ended session {}", id);
    }
}

// Import Event
use crate::events::types::Event;
