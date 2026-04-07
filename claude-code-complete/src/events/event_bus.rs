//! Event Bus - Original: structuredIO.ts outbound stream + event distribution
//!
//! This implements the event-driven architecture used by the original Claude Code.
//! The EventBus allows decoupled communication between UI, core, and I/O layers.

use crate::events::types::Event;
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc};
use tracing::{debug, error, info};

/// Event bus for application-wide event distribution
#[derive(Clone)]
pub struct EventBus {
    /// Broadcast sender for events (UI → Core)
    sender: broadcast::Sender<Event>,

    /// Dedicated channel for high-priority events
    priority_tx: mpsc::Sender<Event>,

    /// Stats for monitoring
    stats: Arc<std::sync::atomic::AtomicUsize>,
}

impl EventBus {
    /// Create a new event bus with specified buffer size
    pub fn new(buffer_size: usize) -> Self {
        let (sender, _receiver) = broadcast::channel(buffer_size);
        let (priority_tx, _priority_rx) = mpsc::channel(buffer_size / 10);

        Self {
            sender,
            priority_tx,
            stats: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
        }
    }

    /// Emit an event to all subscribers
    pub fn emit(&self, event: Event) {
        let count = self
            .stats
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        debug!("Emitting event #{}: {:?}", count, std::mem::discriminant(&event));

        match self.sender.send(event) {
            Ok(n) => {
                debug!("Event sent to {} subscribers", n);
            }
            Err(e) => {
                error!("Failed to emit event: {}", e);
            }
        }
    }

    /// Emit a high-priority event
    pub async fn emit_priority(&self, event: Event) {
        match self.priority_tx.send(event).await {
            Ok(()) => {
                debug!("Priority event sent");
            }
            Err(e) => {
                error!("Failed to emit priority event: {}", e);
            }
        }
    }

    /// Subscribe to events
    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.sender.subscribe()
    }

    /// Subscribe to priority events
    pub fn subscribe_priority(&self) -> mpsc::Receiver<Event> {
        let (_tx, rx) = mpsc::channel(100);
        rx
    }

    /// Get current event count
    pub fn event_count(&self) -> usize {
        self.stats.load(std::sync::atomic::Ordering::SeqCst)
    }

    /// Shutdown the event bus
    pub fn shutdown(&self) {
        info!("Shutting down event bus, {} events processed", self.event_count());
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new(1000)
    }
}

/// Event dispatcher with filtering capabilities
pub struct EventDispatcher {
    bus: EventBus,
    filters: Vec<Box<dyn Fn(&Event) -> bool + Send + Sync>>,
}

impl EventDispatcher {
    pub fn new(bus: EventBus) -> Self {
        Self {
            bus,
            filters: Vec::new(),
        }
    }

    /// Add an event filter
    pub fn add_filter<F>(&mut self, filter: F)
    where
        F: Fn(&Event) -> bool + Send + Sync + 'static,
    {
        self.filters.push(Box::new(filter));
    }

    /// Emit event if it passes all filters
    pub fn emit_filtered(&self, event: Event) {
        if self.filters.iter().all(|f| f(&event)) {
            self.bus.emit(event);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::types::{CoreEvent, SystemEvent};

    #[tokio::test]
    async fn test_event_bus_basic() {
        let bus = EventBus::new(10);
        let mut rx = bus.subscribe();

        bus.emit(Event::System(SystemEvent::Shutdown));

        let event = rx.recv().await.unwrap();
        match event {
            Event::System(SystemEvent::Shutdown) => {}
            _ => panic!("Wrong event type"),
        }
    }

    #[tokio::test]
    async fn test_multiple_subscribers() {
        let bus = EventBus::new(10);
        let mut rx1 = bus.subscribe();
        let mut rx2 = bus.subscribe();

        bus.emit(Event::Core(CoreEvent::Init));

        assert!(matches!(rx1.recv().await.unwrap(), Event::Core(CoreEvent::Init)));
        assert!(matches!(rx2.recv().await.unwrap(), Event::Core(CoreEvent::Init)));
    }
}
