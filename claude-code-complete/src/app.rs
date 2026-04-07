//! Main Application - Complete Ratatui UI implementation
//!
//! This is the core UI application implementing the original Claude Code
//! interface using Ratatui (Rust equivalent of Ink).

use crate::events::types::*;
use crate::events::EventBus;
use crate::state::{GlobalState, AppState};
use crate::ui::theme::Theme;
use crate::ui::widgets::{ChatPanel, CompanionWidget, Sidebar, StatusBar};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CrosstermEvent, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame, Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

/// Application mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppMode {
    Normal,
    Input,
    Help,
    Settings,
    Quitting,
}

/// Main application struct
pub struct App {
    /// Event bus for communication
    pub event_bus: EventBus,

    /// Global state
    pub state: GlobalState,

    /// Current mode
    pub mode: AppMode,

    /// Theme
    pub theme: Theme,

    /// UI Components
    pub sidebar: Sidebar,
    pub chat_panel: ChatPanel,
    pub status_bar: StatusBar,
    pub companion: CompanionWidget,

    /// Tick counter for animations
    pub tick_count: u64,

    /// Last tick time
    pub last_tick: Instant,

    /// Running flag
    pub running: bool,

    /// Event receiver
    event_rx: mpsc::Receiver<Event>,
}

impl App {
    /// Create a new application instance
    pub fn new() -> Self {
        let event_bus = EventBus::new(1000);
        let state = GlobalState::new();
        let theme = Theme::default();

        let project_name = "claude-code-rev".to_string();
        let project_path = "~/Development/claude-code-rev".to_string();

        let sidebar = Sidebar::new(project_name, project_path);
        let chat_panel = ChatPanel::new();
        let status_bar = StatusBar::new();
        let companion = CompanionWidget::new();

        let (_tx, rx) = mpsc::channel(100);

        Self {
            event_bus,
            state,
            mode: AppMode::Normal,
            theme,
            sidebar,
            chat_panel,
            status_bar,
            companion,
            tick_count: 0,
            last_tick: Instant::now(),
            running: true,
            event_rx: rx,
        }
    }

    /// Run the application
    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Main event loop
        let result = self.run_loop(&mut terminal).await;

        // Restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        result
    }

    async fn run_loop<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut last_tick = Instant::now();
        let tick_rate = Duration::from_millis(100);

        while self.running {
            // Draw UI
            terminal.draw(|f| self.draw(f))?;

            // Handle events
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if crossterm::event::poll(timeout)? {
                if let CrosstermEvent::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        self.handle_key_event(key.code).await;
                    }
                }
            }

            // Tick for animations
            if last_tick.elapsed() >= tick_rate {
                self.on_tick();
                last_tick = Instant::now();
            }
        }

        Ok(())
    }

    /// Handle key events
    async fn handle_key_event(&mut self, key: KeyCode) {
        match self.mode {
            AppMode::Normal => match key {
                KeyCode::Char('q') => {
                    self.mode = AppMode::Quitting;
                    self.running = false;
                }
                KeyCode::Char('i') => {
                    self.mode = AppMode::Input;
                }
                KeyCode::Char('h') => {
                    self.mode = AppMode::Help;
                }
                KeyCode::Char('s') => {
                    self.mode = AppMode::Settings;
                }
                KeyCode::Enter => {
                    self.send_message().await;
                }
                _ => {}
            },
            AppMode::Input => match key {
                KeyCode::Esc => {
                    self.mode = AppMode::Normal;
                }
                KeyCode::Char(c) => {
                    self.chat_panel.input.push(c);
                }
                KeyCode::Backspace => {
                    self.chat_panel.input.pop();
                }
                KeyCode::Enter => {
                    self.send_message().await;
                }
                _ => {}
            },
            AppMode::Help => {
                if key == KeyCode::Esc {
                    self.mode = AppMode::Normal;
                }
            }
            AppMode::Settings => {
                if key == KeyCode::Esc {
                    self.mode = AppMode::Normal;
                }
            }
            _ => {}
        }
    }

    /// Send a message
    async fn send_message(&mut self) {
        let input = self.chat_panel.input.clone();
        if !input.is_empty() {
            // Add user message
            self.chat_panel.add_message(
                crate::ui::widgets::chat_panel::MessageRole::User,
                input.clone(),
            );
            self.chat_panel.input.clear();

            // Emit event to core
            self.event_bus.emit(Event::Session(SessionEvent::UserMessage {
                session_id: uuid::Uuid::new_v4(),
                message: crate::events::types::ChatMessage {
                    id: uuid::Uuid::new_v4(),
                    role: crate::events::types::MessageRole::User,
                    content: input,
                    timestamp: chrono::Utc::now(),
                    metadata: None,
                },
            }));

            // Start streaming response
            self.companion.set_mood(crate::ui::widgets::companion::CompanionMood::Thinking);
            self.chat_panel.start_streaming();

            // Simulate response (in real implementation, this would call the API)
            tokio::spawn(simulate_response(
                self.event_bus.clone(),
                self.chat_panel.messages.len() - 1,
            ));
        }
    }

    /// Called on every tick for animations
    fn on_tick(&mut self) {
        self.tick_count += 1;

        // Update animations based on tick
        if self.chat_panel.is_typing {
            // Trigger redraw
        }
    }

    /// Draw the UI
    fn draw(&self,
        frame: &mut Frame,
    ) {
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Title bar
                Constraint::Min(1),    // Main content
                Constraint::Length(1), // Status bar
            ])
            .split(frame.area());

        // Title bar
        self.render_title_bar(frame, main_layout[0]);

        // Main content
        self.render_main_content(frame, main_layout[1]);

        // Status bar
        self.status_bar.render(main_layout[2], frame.buffer_mut());

        // Modal overlays
        match self.mode {
            AppMode::Help => self.render_help_modal(frame),
            AppMode::Settings => self.render_settings_modal(frame),
            _ => {}
        }
    }

    fn render_title_bar(&self,
        frame: &mut Frame,
        area: Rect,
    ) {
        let title = format!("  Claude Code {}  ", self.status_bar.version);
        let text = Paragraph::new(Line::from(vec![
            Span::styled(title, self.theme.title()),
            Span::raw(" "),
            Span::styled("[🗑️] [⚙️] [☁️] [📥]   ", self.theme.muted()),
        ]));
        frame.render_widget(text, area);
    }

    fn render_main_content(
        &self,
        frame: &mut Frame,
        area: Rect,
    ) {
        let content_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(35), // Sidebar
                Constraint::Percentage(65), // Chat
            ])
            .split(area);

        // Sidebar
        self.sidebar.render(content_layout[0], frame.buffer_mut());

        // Chat panel
        self.chat_panel.render(content_layout[1], frame.buffer_mut());
    }

    fn render_help_modal(&self,
        frame: &mut Frame,
    ) {
        let area = centered_rect(60, 70, frame.area());
        frame.render_widget(Clear, area);

        let help_text = vec![
            Line::from(vec![Span::styled("Keyboard Shortcuts", self.theme.title())]),
            Line::from(""),
            Line::from("  i          - Enter input mode"),
            Line::from("  Enter      - Send message"),
            Line::from("  Esc        - Exit input mode"),
            Line::from("  h          - Show this help"),
            Line::from("  s          - Open settings"),
            Line::from("  q          - Quit"),
            Line::from("  ↑/↓        - Scroll messages"),
            Line::from("  Tab        - Switch sidebar tab"),
            Line::from(""),
            Line::from("Press Esc to close"),
        ];

        let help = Paragraph::new(help_text)
            .block(
                Block::default()
                    .title("Help")
                    .borders(Borders::ALL)
                    .border_style(self.theme.border_highlight_style()),
            )
            .wrap(ratatui::widgets::Wrap { trim: true });

        frame.render_widget(help, area);
    }

    fn render_settings_modal(&self,
        frame: &mut Frame,
    ) {
        let area = centered_rect(60, 70, frame.area());
        frame.render_widget(Clear, area);

        let settings_text = Paragraph::new("Settings panel - coming soon")
            .block(
                Block::default()
                    .title("Settings")
                    .borders(Borders::ALL)
                    .border_style(self.theme.border_highlight_style()),
            );

        frame.render_widget(settings_text, area);
    }
}

/// Helper to create a centered rectangle
fn centered_rect(
    percent_x: u16,
    percent_y: u16,
    r: Rect,
) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

/// Simulate API response (for demo)
async fn simulate_response(
    event_bus: EventBus,
    message_index: usize,
) {
    // Simulate typing delay
    tokio::time::sleep(Duration::from_secs(1)).await;

    let response = "Hello! I'm Claude, your AI coding assistant. I can help you understand your codebase, write code, and answer questions about your project.\n\nWhat would you like to work on today?";

    // Stream response
    for word in response.split_whitespace() {
        tokio::time::sleep(Duration::from_millis(50)).await;
        // In real implementation, emit stream chunk event
    }

    // End streaming
    event_bus.emit(Event::Session(SessionEvent::StreamEnded {
        session_id: uuid::Uuid::new_v4(),
        message_id: uuid::Uuid::new_v4(),
    }));
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
