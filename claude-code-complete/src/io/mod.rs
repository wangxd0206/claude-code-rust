//! I/O Layer - Original: structuredIO.ts + remoteIO.ts + print.ts
//!
//! Handles all input/output operations including:
//! - Structured I/O protocol
//! - Terminal rendering
//! - Remote I/O for bridge mode
//! - JSON-RPC communication

pub mod structured_io;
pub mod terminal_renderer;

pub use structured_io::StructuredIO;
pub use terminal_renderer::TerminalRenderer;
