//! Permission Manager - Original: permissions/ directory
//!
//! Handles tool use permissions and user approval flow.

use crate::events::types::{PermissionEvent, PermissionMode};
use crate::events::EventBus;
use std::collections::HashMap;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use uuid::Uuid;

/// Manages permissions for tool usage
#[derive(Debug)]
pub struct PermissionManager {
    event_bus: EventBus,
    mode: RwLock<PermissionMode>,
    pending_requests: RwLock<HashMap<Uuid, PermissionRequest>>,
    allowed_tools: RwLock<Vec<String>>,
    denied_tools: RwLock<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct PermissionRequest {
    pub id: Uuid,
    pub tool_name: String,
    pub description: String,
    pub input: serde_json::Value,
}

impl PermissionManager {
    pub fn new(event_bus: EventBus) -> Self {
        Self {
            event_bus,
            mode: RwLock::new(PermissionMode::Confirm),
            pending_requests: RwLock::new(HashMap::new()),
            allowed_tools: RwLock::new(Vec::new()),
            denied_tools: RwLock::new(Vec::new()),
        }
    }

    /// Request permission for a tool
    pub async fn request_permission(
        &self,
        tool_name: String,
        description: String,
        input: serde_json::Value,
    ) -> PermissionResult {
        let mode = *self.mode.read().await;

        match mode {
            PermissionMode::Auto => {
                info!("Auto-granting permission for: {}", tool_name);
                PermissionResult::Granted
            }
            PermissionMode::Never => {
                warn!("Auto-denying permission for: {}", tool_name);
                PermissionResult::Denied("Permission mode is 'Never'".to_string())
            }
            PermissionMode::Confirm => {
                let request = PermissionRequest {
                    id: Uuid::new_v4(),
                    tool_name: tool_name.clone(),
                    description: description.clone(),
                    input,
                };

                let mut pending = self.pending_requests.write().await;
                let id = request.id;
                pending.insert(id, request);

                // Emit event to UI
                self.event_bus.emit(crate::events::types::Event::Permission(
                    PermissionEvent::Requested {
                        request_id: id,
                        tool_name,
                        description,
                    }
                ));

                debug!("Permission requested: {}", id);
                PermissionResult::Pending(id)
            }
        }
    }

    /// Grant a pending permission
    pub async fn grant(&self, request_id: Uuid) -> Result<(), String> {
        let mut pending = self.pending_requests.write().await;
        if let Some(request) = pending.remove(&request_id) {
            self.event_bus.emit(crate::events::types::Event::Permission(
                PermissionEvent::Granted { request_id }
            ));

            let mut allowed = self.allowed_tools.write().await;
            allowed.push(request.tool_name);

            info!("Permission granted: {}", request_id);
            Ok(())
        } else {
            Err("Request not found".to_string())
        }
    }

    /// Deny a pending permission
    pub async fn deny(&self, request_id: Uuid, reason: Option<String>) -> Result<(), String> {
        let mut pending = self.pending_requests.write().await;
        if let Some(request) = pending.remove(&request_id) {
            self.event_bus.emit(crate::events::types::Event::Permission(
                PermissionEvent::Denied { request_id, reason: reason.clone() }
            ));

            let mut denied = self.denied_tools.write().await;
            denied.push(request.tool_name);

            warn!("Permission denied: {}", request_id);
            Ok(())
        } else {
            Err("Request not found".to_string())
        }
    }

    /// Set permission mode
    pub async fn set_mode(&self, mode: PermissionMode) {
        let mut current = self.mode.write().await;
        *current = mode;

        self.event_bus.emit(crate::events::types::Event::Permission(
            PermissionEvent::ModeChanged { mode }
        ));

        info!("Permission mode changed to: {:?}", mode);
    }
}

#[derive(Debug, Clone)]
pub enum PermissionResult {
    Granted,
    Denied(String),
    Pending(Uuid),
}
