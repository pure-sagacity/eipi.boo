// Catppuccin Mocha - https://catppuccin.com
use ratatui::style::Color;

use crate::tui::theme::Theme;

pub const THEME: Theme = Theme {
    text: Color::Indexed(189),           // text: #cdd6f4
    text_secondary: Color::Indexed(250), // subtext0: #a6adc8
    text_dim: Color::Indexed(245),       // overlay0: #6c7086

    border: Color::Indexed(60),     // surface1: #45475a
    border_dim: Color::Indexed(59), // surface0: #313244

    accent: Color::Indexed(220),        // yellow: #f9e2af
    accent_alt: Color::Indexed(159),    // teal: #94e2d5
    accent_search: Color::Indexed(183), // mauve: #cba6f7
    accent_rose: Color::Indexed(211),   // pink: #f5c2e7

    heart: Color::Indexed(210),   // red: #f38ba8
    online: Color::Indexed(120),  // green: #a6e3a1
    warning: Color::Indexed(215), // peach: #fab387

    glow_high: Color::Indexed(183), // mauve
    glow_mid: Color::Indexed(159),  // teal

    dot_red: Color::Indexed(210),    // red
    dot_yellow: Color::Indexed(220), // yellow
    dot_green: Color::Indexed(120),  // green

    terminal_bg: "#1e1e2e",
    terminal_fg: "#cdd6f4",
};
