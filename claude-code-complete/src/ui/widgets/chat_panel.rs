use crate::ui::theme::{Theme, colors};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};
use std::time::{SystemTime, UNIX_EPOCH};

/// Chat panel widget with markdown rendering and typing animation
pub struct ChatPanel {
    pub messages: Vec<Message>,
    pub input: String,
    pub is_typing: bool,
    pub scroll_offset: u16,
    pub cursor_position: usize,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub id: String,
    pub role: MessageRole,
    pub content: String,
    pub timestamp: u64,
    pub is_streaming: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

impl ChatPanel {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            input: String::new(),
            is_typing: false,
            scroll_offset: 0,
            cursor_position: 0,
        }
    }

    pub fn add_message(&mut self, role: MessageRole, content: String) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        self.messages.push(Message {
            id: format!("msg_{}", timestamp),
            role,
            content,
            timestamp,
            is_streaming: false,
        });
    }

    pub fn start_streaming(&mut self) {
        self.is_typing = true;
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        self.messages.push(Message {
            id: format!("msg_{}", timestamp),
            role: MessageRole::Assistant,
            content: String::new(),
            timestamp,
            is_streaming: true,
        });
    }

    pub fn append_streaming(&mut self, chunk: &str) {
        if let Some(last) = self.messages.last_mut() {
            if last.is_streaming {
                last.content.push_str(chunk);
            }
        }
    }

    pub fn end_streaming(&mut self) {
        self.is_typing = false;
        if let Some(last) = self.messages.last_mut() {
            last.is_streaming = false;
        }
    }

    pub fn clear(&mut self) {
        self.messages.clear();
    }

    fn format_content(&self, content: &str) -> Vec<Line> {
        let mut lines = Vec::new();

        // Simple markdown parsing
        for line in content.lines() {
            if line.starts_with("```") {
                lines.push(Line::from(Span::styled(
                    line,
                    Style::default().fg(colors::TEXT_MUTED),
                )));
            } else if line.starts_with("# ") {
                lines.push(Line::from(Span::styled(
                    &line[2..],
                    Style::default()
                        .fg(colors::PRIMARY)
                        .add_modifier(Modifier::BOLD),
                )));
            } else if line.starts_with("## ") {
                lines.push(Line::from(Span::styled(
                    &line[3..],
                    Style::default()
                        .fg(colors::PRIMARY)
                        .add_modifier(Modifier::BOLD),
                )));
            } else if line.starts_with("- ") || line.starts_with("* ") {
                lines.push(Line::from(vec![
                    Span::styled("  • ", Style::default().fg(colors::PRIMARY)),
                    Span::raw(&line[2..]),
                ]));
            } else if line.starts_with("> ") {
                lines.push(Line::from(vec![
                    Span::styled("  │ ", Style::default().fg(colors::BORDER)),
                    Span::styled(&line[2..], Style::default().fg(colors::TEXT_MUTED)),
                ]));
            } else if line.contains("**") {
                let mut result = Vec::new();
                let parts: Vec<&str> = line.split("**").collect();
                for (i, part) in parts.iter().enumerate() {
                    if i % 2 == 1 {
                        result.push(Span::styled(
                            *part,
                            Style::default().add_modifier(Modifier::BOLD),
                        ));
                    } else {
                        result.push(Span::raw(*part));
                    }
                }
                lines.push(Line::from(result));
            } else {
                lines.push(Line::from(line.to_string()));
            }
        }

        lines
    }

    fn render_welcome(&self, area: Rect, buf: &mut Buffer, theme: &Theme) {
        let welcome_text = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "                    Tips for getting started",
                    Style::default()
                        .fg(theme.primary)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "                    Run ",
                    Style::default().fg(theme.foreground_muted),
                ),
                Span::styled("/init", Style::default().fg(theme.accent)),
                Span::styled(
                    " to create a CLAUDE.md file with instr...",
                    Style::default().fg(theme.foreground_muted),
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "                    Recent activity",
                    Style::default()
                        .fg(theme.primary)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    "                    No recent activity",
                    Style::default().fg(theme.foreground_muted),
                ),
            ]),
        ];

        Paragraph::new(welcome_text)
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true })
            .render(area, buf);
    }

    fn render_input(&self, area: Rect, buf: &mut Buffer, theme: &Theme) {
        let input_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(colors::BORDER));

        input_block.render(area, buf);

        let inner = area.inner(&Margin {
            horizontal: 1,
            vertical: 1,
        });

        let input_text = if self.input.is_empty() {
            Paragraph::new("⮕ Try \"how does <filepath> work?\"").style(
                Style::default().fg(colors::TEXT_MUTED),
            )
        } else {
            Paragraph::new(self.input.clone())
        };

        input_text.render(inner, buf);
    }
}

impl Default for ChatPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for &ChatPanel {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let theme = Theme::default();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),  // Status bar
                Constraint::Min(1),     // Messages
                Constraint::Length(1),  // Divider
                Constraint::Length(3),  // Input
            ])
            .split(area);

        // Status bar (announcement)
        let status_bar = Paragraph::new(vec![
            Line::from(vec![
                Span::styled(
                    "↑ Opus now defaults to 1M context · 5x more room, same pricing",
                    Style::default().fg(colors::TEXT_MUTED),
                ),
            ]),
        ])
        .alignment(Alignment::Left);

        status_bar.render(chunks[0], buf);

        // Messages area
        let main_block = Block::default()
            .borders(Borders::ALL)
            .border_style(
                Style::default().fg(ratatui::style::Color::Rgb(200, 150, 200)),
            );

        main_block.render(chunks[1], buf);

        let messages_inner = chunks[1].inner(&Margin {
            horizontal: 1,
            vertical: 1,
        });

        if self.messages.is_empty() {
            self.render_welcome(messages_inner, buf, &theme);
        } else {
            let mut all_lines = Vec::new();

            for msg in &self.messages {
                let avatar = match msg.role {
                    MessageRole::User => "👤",
                    MessageRole::Assistant => "🟣",
                    MessageRole::System => "⚙️",
                };

                let role_color = match msg.role {
                    MessageRole::User => colors::ACCENT,
                    MessageRole::Assistant => colors::PRIMARY,
                    MessageRole::System => colors::TEXT_MUTED,
                };

                let role_name = match msg.role {
                    MessageRole::User => "You",
                    MessageRole::Assistant => "Claude",
                    MessageRole::System => "System",
                };

                // Header
                all_lines.push(Line::from(""));
                all_lines.push(Line::from(vec![
                    Span::raw("  "),
                    Span::styled(avatar, Style::default()),
                    Span::raw(" "),
                    Span::styled(
                        role_name,
                        Style::default()
                            .fg(role_color)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]));
                all_lines.push(Line::from(""));

                // Content
                for line in self.format_content(&msg.content) {
                    all_lines.push(Line::from(vec![
                        Span::raw("    "),
                        Span::styled(line, Style::default().fg(colors::TEXT_PRIMARY)),
                    ]));
                }

                // Typing indicator
                if msg.is_streaming {
                    all_lines.push(Line::from(vec![
                        Span::raw("    "),
                        Span::styled("▌", Style::default().fg(colors::PRIMARY)),
                    ]));
                }
            }

            Paragraph::new(all_lines)
                .wrap(Wrap { trim: true })
                .render(messages_inner, buf);
        }

        // Divider
        let divider = Paragraph::new("─".repeat(area.width as usize))
            .style(Style::default().fg(colors::BORDER));
        divider.render(chunks[2], buf);

        // Input
        self.render_input(chunks[3], buf, &theme);
    }
}
