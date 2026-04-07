//! State Management - Original: sessionState.ts + externalMetadataToAppState
//!
//! Centralized state management for the application, matching the
//! original's session state and app state handling.

pub mod app_state;
pub mod session;

pub use app_state::{AppState, GlobalState};
pub use session::{Session, SessionManager};
