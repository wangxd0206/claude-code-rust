use crate::ui::theme::{Theme, colors};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

/// Status bar at the bottom of the screen
pub struct StatusBar {
    pub message: String,
    pub version: String,
    pub show_shortcuts: bool,
}

impl StatusBar {
    pub fn new() -> Self {
        Self {
            message: "Claude Code has switched from npm to native installer. Run `claude inst...".to_string(),
            version: "v999.0.0-restored".to_string(),
            show_shortcuts: true,
        }
    }

    pub fn set_message(&mut self, message: String) {
        self.message = message;
    }
}

impl Default for StatusBar {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for &StatusBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text = if self.show_shortcuts {
            vec![
                Line::from(vec![
                    Span::styled("? for shortcuts  ", Style::default().fg(colors::TEXT_MUTED)),
                    Span::styled(
                        &self.message,
                        Style::default().fg(colors::TEXT_MUTED),
                    ),
                ]),
            ]
        } else {
            vec![
                Line::from(vec![
                    Span::styled(&self.message, Style::default().fg(colors::TEXT_MUTED)),
                ]),
            ]
        };

        Paragraph::new(text)
            .alignment(Alignment::Left)
            .render(area, buf);
    }
}
