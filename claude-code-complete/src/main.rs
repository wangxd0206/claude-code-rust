//! Claude Code Complete - Full Rust Implementation
//!
//! A complete reimplementation of Claude Code in Rust, with:
//! - Full event-driven architecture
//! - Ratatui terminal UI (matching original Ink implementation)
//! - Session management
//! - Tool system
//! - MCP protocol support
//! - Real-time updates and click feedback

mod app;
mod events;
mod state;
mod ui;

use app::App;
use clap::Parser;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

/// Claude Code CLI arguments
#[derive(Parser, Debug)]
#[command(name = "claude-code")]
#[command(about = "High-performance Rust implementation of Claude Code", long_about = None)]
struct Cli {
    /// Run in verbose mode
    #[arg(short, long)]
    verbose: bool,

    /// Print version
    #[arg(short, long)]
    version: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Print version and exit
    if cli.version {
        println!("claude-code v999.0.0-restored (Rust implementation)");
        return Ok(());
    }

    // Setup logging
    let log_level = if cli.verbose {
        Level::DEBUG
    } else {
        Level::INFO
    };

    let subscriber = FmtSubscriber::builder()
        .with_max_level(log_level)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    info!("Starting Claude Code - Rust implementation");

    // Create and run the application
    let mut app = App::new();
    app.run().await?;

    info!("Shutting down gracefully");
    Ok(())
}
