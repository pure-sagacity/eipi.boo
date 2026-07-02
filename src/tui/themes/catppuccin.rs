// Catppuccin Mocha - https://catppuccin.com
use ratatui::style::Color;

use crate::tui::theme::Theme;

pub const THEME: Theme = Theme {
    text: Color::Rgb(205, 214, 244),           // text: #cdd6f4
    text_secondary: Color::Rgb(166, 173, 200), // subtext0: #a6adc8
    text_dim: Color::Rgb(108, 112, 134),       // overlay0: #6c7086

    border: Color::Rgb(69, 71, 90),        // surface1: #45475a
    border_dim: Color::Rgb(49, 50, 68),    // surface0: #313244

    accent: Color::Rgb(249, 226, 175),        // yellow: #f9e2af
    accent_alt: Color::Rgb(148, 226, 213),    // teal: #94e2d5
    accent_search: Color::Rgb(203, 166, 247), // mauve: #cba6f7
    accent_rose: Color::Rgb(245, 194, 231),   // pink: #f5c2e7

    heart: Color::Rgb(243, 139, 168),   // red: #f38ba8
    online: Color::Rgb(166, 227, 161),  // green: #a6e3a1
    warning: Color::Rgb(250, 179, 135), // peach: #fab387

    glow_high: Color::Rgb(203, 166, 247), // mauve
    glow_mid: Color::Rgb(148, 226, 213),  // teal

    dot_red: Color::Rgb(243, 139, 168),    // red
    dot_yellow: Color::Rgb(249, 226, 175), // yellow
    dot_green: Color::Rgb(166, 227, 161),  // green

    terminal_bg: "#1e1e2e",
    terminal_fg: "#cdd6f4",
};
