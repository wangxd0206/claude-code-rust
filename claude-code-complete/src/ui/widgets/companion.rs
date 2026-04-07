use crate::ui::theme::{Theme, colors};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

/// Gravy companion widget (pink pig sprite)
pub struct CompanionWidget {
    pub mood: CompanionMood,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CompanionMood {
    Happy,
    Thinking,
    Sleeping,
    Excited,
}

impl CompanionWidget {
    pub fn new() -> Self {
        Self {
            mood: CompanionMood::Happy,
            message: None,
        }
    }

    pub fn set_mood(&mut self, mood: CompanionMood) {
        self.mood = mood;
    }

    pub fn set_message(&mut self, message: String) {
        self.message = Some(message);
    }

    fn get_sprite(&self) -> Vec<&'static str> {
        match self.mood {
            CompanionMood::Happy => vec![
                "  🟪🟪🟪🟪  ",
                " 🟪🟪🟪🟪🟪🟪 ",
                "🟪🟪⬜⬜🟪🟪🟪",
                "🟪🟪⬜⬜🟪🟪🟪",
                " 🟪🟪🟪🟪🟪🟪 ",
                "  🟪🟪🟪🟪  ",
            ],
            CompanionMood::Thinking => vec![
                "  🟪🟪🟪🟪  ",
                " 🟪🟪🟪🟪🟪🟪 ",
                "🟪🟪⬛⬜🟪🟪🟪",
                "🟪🟪⬜⬜🟪🟪🟪",
                " 🟪🟪🟪🟪🟪🟪 ",
                "  🟪🟪🟪🟪  ",
            ],
            CompanionMood::Sleeping => vec![
                "  🟪🟪🟪🟪  ",
                " 🟪🟪🟪🟪🟪🟪 ",
                "🟪🟪➖➖🟪🟪🟪",
                "🟪🟪➖➖🟪🟪🟪",
                " 🟪🟪🟪🟪🟪🟪 ",
                "  🟪🟪🟪🟪  ",
            ],
            CompanionMood::Excited => vec![
                "  🟪🟪🟪🟪  ",
                " 🟪🟪🟪🟪🟪🟪 ",
                "🟪🟪⬜⬜🟪🟪🟪",
                "🟪🟪⬜⬜🟪🟪🟪",
                " 🟪🟪🟪🟪🟪🟪 ",
                "  🟪🟪🟪🟪  ",
                "   z  Z   ",
            ],
        }
    }

    fn get_status_message(&self) -> &str {
        match self.mood {
            CompanionMood::Happy => "Welcome back!",
            CompanionMood::Thinking => "Claude is thinking...",
            CompanionMood::Sleeping => "zzz...",
            CompanionMood::Excited => "Great to see you!",
        }
    }
}

impl Default for CompanionWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for &CompanionWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let theme = Theme::default();
        let sprite = self.get_sprite();
        let pink = colors::GRAVY_PINK;

        // Render sprite
        for (i, line) in sprite.iter().enumerate() {
            let y = area.top() + i as u16;
            if y < area.bottom() {
                let content = line.replace("🟪", "█").replace("⬜", "░").replace("⬛", "▓").replace("➖", "─");
                let x = area.left() + (area.width / 2).saturating_sub(content.len() as u16 / 2);
                buf.set_string(
                    x,
                    y,
                    content,
                    Style::default().fg(pink),
                );
            }
        }

        // Render status message
        let message = self.message.as_deref().unwrap_or(self.get_status_message());
        let text_area = Rect {
            x: area.left,
            y: area.top() + 8,
            width: area.width,
            height: 2,
        };

        let text = Paragraph::new(vec![
            Line::from(""),
            Line::from(Span::styled(message, theme.title())),
        ])
        .alignment(Alignment::Center);

        text.render(text_area, buf);
    }
}
