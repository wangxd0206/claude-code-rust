//! Structured I/O - Original: structuredIO.ts
//!
//! Handles structured message I/O for SDK communication.

use crate::events::types::Event;
use crate::events::EventBus;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

/// Structured I/O handler
#[derive(Debug)]
pub struct StructuredIO {
    event_bus: EventBus,
    input_rx: mpsc::Receiver<String>,
    output_tx: mpsc::Sender<String>,
}

impl StructuredIO {
    pub fn new(event_bus: EventBus) -> (Self, mpsc::Sender<String>, mpsc::Receiver<String>) {
        let (input_tx, input_rx) = mpsc::channel(100);
        let (output_tx, output_rx) = mpsc::channel(100);

        let io = Self {
            event_bus,
            input_rx,
            output_tx,
        };

        (io, input_tx, output_rx)
    }

    pub async fn run(&mut self) {
        info!("Structured I/O started");

        while let Some(message) = self.input_rx.recv().await {
            debug!("Received message: {}", message);
            // Process incoming messages
        }

        info!("Structured I/O stopped");
    }

    pub async fn send(&self, message: String) -> Result<(), String> {
        self.output_tx
            .send(message)
            .await
            .map_err(|e| format!("Failed to send: {}", e))
    }
}
