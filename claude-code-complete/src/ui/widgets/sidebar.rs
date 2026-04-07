use crate::events::types::SidebarTab;
use crate::ui::theme::Theme;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

/// Sidebar widget matching original Claude Code sidebar
pub struct Sidebar {
    pub selected_tab: SidebarTab,
    pub project_name: String,
    pub project_path: String,
    pub show_companion: bool,
}

impl Sidebar {
    pub fn new(project_name: String, project_path: String) -> Self {
        Self {
            selected_tab: SidebarTab::Chat,
            project_name,
            project_path,
            show_companion: true,
        }
    }

    fn render_companion(&self,
        area: Rect,
        buf: &mut Buffer,
        theme: &Theme,
    ) {
        // Gravy companion (pink pig sprite)
        let companion_art = vec![
            "  🟪🟪🟪🟪  ",
            " 🟪🟪🟪🟪🟪🟪 ",
            "🟪🟪⬜⬜🟪🟪🟪",
            "🟪🟪⬜⬜🟪🟪🟪",
            " 🟪🟪🟪🟪🟪🟪 ",
            "  🟪🟪🟪🟪  ",
        ];

        let companion_color = ratatui::style::Color::Rgb(255, 182, 193); // Pink

        for (i, line) in companion_art.iter().enumerate() {
            let y = area.top() + i as u16;
            if y < area.bottom() {
                buf.set_string(
                    area.left + 2,
                    y,
                    line.replace("🟪", "█").replace("⬜", "░"),
                    Style::default().fg(companion_color),
                );
            }
        }

        // Welcome text
        let welcome_text = Paragraph::new(vec![
            Line::from(""),
            Line::from(Span::styled("Welcome back!", theme.title())),
        ])
        .alignment(Alignment::Center);

        let text_area = Rect {
            x: area.left,
            y: area.top() + 7,
            width: area.width,
            height: 2,
        };
        welcome_text.render(text_area, buf);

        // Model info
        let info_text = Paragraph::new(vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("Haiku 4.5", Style::default().fg(theme.foreground_muted)),
            ]),
            Line::from(vec![
                Span::styled("API Usage Billing", Style::default().fg(theme.foreground_muted)),
            ]),
            Line::from(vec![
                Span::styled(
                    self.project_path.clone(),
                    Style::default().fg(theme.foreground_muted),
                ),
            ]),
        ])
        .alignment(Alignment::Center);

        let info_area = Rect {
            x: area.left,
            y: area.top() + 10,
            width: area.width,
            height: 4,
        };
        info_text.render(info_area, buf);
    }
}

impl Widget for &Sidebar {
    fn render(
        self,
        area: Rect,
        buf: &mut Buffer,
    ) {
        let theme = Theme::default();

        // Main sidebar block with pink/purple border
        let sidebar_block = Block::default()
            .borders(Borders::ALL)
            .border_style(
                Style::default()
                    .fg(ratatui::style::Color::Rgb(200, 150, 200)),
            )
            .title("");

        sidebar_block.render(area, buf);

        let inner = area.inner(&ratatui::layout::Margin {
            horizontal: 1,
            vertical: 1,
        });

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(20), // Companion area
                Constraint::Min(1),     // Navigation
                Constraint::Length(3),  // Footer
            ])
            .split(inner);

        // Companion area
        if self.show_companion {
            self.render_companion(chunks[0], buf, &theme);
        }

        // Project footer
        let footer = Paragraph::new(vec![
            Line::from("📂 "),
            Line::from(Span::styled(
                &self.project_name,
                Style::default().add_modifier(Modifier::BOLD),
            )),
        ])
        .alignment(Alignment::Center);

        footer.render(chunks[2], buf);
    }
}
