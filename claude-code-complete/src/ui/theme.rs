//! Theme Module - Original Claude Code Color Scheme
//!
//! Exact color matching from the original TypeScript implementation

use ratatui::style::{Color, Modifier, Style};

/// Original Claude Code color palette
pub mod colors {
    use ratatui::style::Color;

    // Primary brand colors
    pub const PRIMARY: Color = Color::Rgb(147, 112, 219);      // #9370db - Purple
    pub const PRIMARY_LIGHT: Color = Color::Rgb(177, 156, 217); // #b19cd9
    pub const PRIMARY_DARK: Color = Color::Rgb(122, 92, 183);   // #7a5cb7

    // Accent colors
    pub const ACCENT: Color = Color::Rgb(255, 140, 66);        // #ff8c42 - Orange
    pub const ACCENT_LIGHT: Color = Color::Rgb(255, 180, 100); // #ffb464

    // Background colors (dark theme)
    pub const BG_DARKEST: Color = Color::Rgb(13, 13, 15);      // #0d0d0f
    pub const BG_DARKER: Color = Color::Rgb(18, 18, 23);       // #121217
    pub const BG_DARK: Color = Color::Rgb(26, 26, 36);         // #1a1a24
    pub const BG_MEDIUM: Color = Color::Rgb(36, 36, 51);       // #242433
    pub const BG_LIGHT: Color = Color::Rgb(45, 45, 63);        // #2d2d3f
    pub const BG_LIGHTER: Color = Color::Rgb(56, 56, 77);      // #38384d

    // Text colors
    pub const TEXT_PRIMARY: Color = Color::Rgb(255, 255, 255);    // White
    pub const TEXT_SECONDARY: Color = Color::Rgb(196, 196, 212); // #c4c4d4
    pub const TEXT_MUTED: Color = Color::Rgb(136, 136, 159);     // #88889f
    pub const TEXT_DISABLED: Color = Color::Rgb(85, 85, 102);    // #555566

    // Border and accent
    pub const BORDER: Color = Color::Rgb(51, 51, 68);         // #333344
    pub const BORDER_LIGHT: Color = Color::Rgb(68, 68, 85);   // #444455

    // Status colors
    pub const SUCCESS: Color = Color::Rgb(16, 185, 129);      // #10b981
    pub const WARNING: Color = Color::Rgb(245, 158, 11);      // #f59e0b
    pub const ERROR: Color = Color::Rgb(239, 68, 68);         // #ef4444
    pub const INFO: Color = Color::Rgb(59, 130, 246);         // #3b82f6

    // Gravy companion colors (pink)
    pub const GRAVY_PINK: Color = Color::Rgb(255, 182, 193);  // Light pink
    pub const GRAVY_PINK_DARK: Color = Color::Rgb(255, 105, 180); // Hot pink
}

use colors::*;

/// Theme struct containing all styles
#[derive(Debug, Clone)]
pub struct Theme {
    pub background: Color,
    pub background_secondary: Color,
    pub background_tertiary: Color,
    pub foreground: Color,
    pub foreground_muted: Color,
    pub primary: Color,
    pub accent: Color,
    pub border: Color,
    pub border_highlight: Color,
    pub success: Color,
    pub error: Color,
    pub warning: Color,
}

impl Theme {
    /// Original Claude Code dark theme
    pub fn dark() -> Self {
        Self {
            background: BG_DARKEST,
            background_secondary: BG_DARKER,
            background_tertiary: BG_DARK,
            foreground: TEXT_PRIMARY,
            foreground_muted: TEXT_SECONDARY,
            primary: PRIMARY,
            accent: ACCENT,
            border: BORDER,
            border_highlight: PRIMARY,
            success: SUCCESS,
            error: ERROR,
            warning: WARNING,
        }
    }

    /// Light theme variant
    pub fn light() -> Self {
        Self {
            background: Color::Rgb(255, 255, 255),
            background_secondary: Color::Rgb(245, 245, 245),
            background_tertiary: Color::Rgb(235, 235, 235),
            foreground: Color::Rgb(31, 31, 31),
            foreground_muted: Color::Rgb(100, 100, 100),
            primary: PRIMARY,
            accent: ACCENT,
            border: Color::Rgb(200, 200, 200),
            border_highlight: PRIMARY,
            success: SUCCESS,
            error: ERROR,
            warning: WARNING,
        }
    }

    // Style helpers
    pub fn title(&self) -> Style {
        Style::default()
            .fg(self.foreground)
            .add_modifier(Modifier::BOLD)
    }

    pub fn subtitle(&self) -> Style {
        Style::default()
            .fg(self.foreground_muted)
    }

    pub fn primary_style(&self) -> Style {
        Style::default()
            .fg(self.primary)
    }

    pub fn accent_style(&self) -> Style {
        Style::default()
            .fg(self.accent)
    }

    pub fn border_style(&self) -> Style {
        Style::default()
            .fg(self.border)
    }

    pub fn border_highlight_style(&self) -> Style {
        Style::default()
            .fg(self.border_highlight)
    }

    pub fn muted(&self) -> Style {
        Style::default()
            .fg(self.foreground_muted)
    }

    pub fn success_style(&self) -> Style {
        Style::default()
            .fg(self.success)
    }

    pub fn error_style(&self) -> Style {
        Style::default()
            .fg(self.error)
    }

    pub fn block_title(&self) -> Style {
        Style::default()
            .fg(self.primary)
            .add_modifier(Modifier::BOLD)
    }

    pub fn selected(&self) -> Style {
        Style::default()
            .bg(self.background_tertiary)
            .fg(self.primary)
    }

    pub fn hover(&self) -> Style {
        Style::default()
            .bg(self.background_secondary)
    }

    /// Block styling with Claude Code appearance
    pub fn block(&self, title: Option<&str>) -> ratatui::widgets::Block {
        use ratatui::widgets::{Block, Borders};

        let mut block = Block::default()
            .borders(Borders::ALL)
            .border_style(self.border_style());

        if let Some(title_text) = title {
            block = block.title(title_text).title_style(self.block_title());
        }

        block
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}
