//! Event System - Original: structuredIO.ts + event bus
//!
//! This module implements the event-driven architecture from the original
//! Claude Code, allowing decoupled communication between UI and core logic.

pub mod event_bus;
pub mod types;

pub use event_bus::EventBus;
pub use types::*;
