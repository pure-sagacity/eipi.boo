use ratatui::style::Color;

use crate::tui::theme::Theme;

pub const THEME: Theme = Theme {
    text: Color::White,
    text_secondary: Color::Gray,
    text_dim: Color::DarkGray,

    border: Color::Indexed(242),
    border_dim: Color::Indexed(238),

    accent: Color::Yellow,
    accent_alt: Color::Cyan,
    accent_search: Color::Magenta,
    accent_rose: Color::Indexed(175),

    heart: Color::Red,
    online: Color::Green,
    warning: Color::Yellow,

    glow_high: Color::Magenta,
    glow_mid: Color::Cyan,

    dot_red: Color::Red,
    dot_yellow: Color::Yellow,
    dot_green: Color::Green,

    terminal_bg: "#1a1b26",
    terminal_fg: "#c0caf5",
};
