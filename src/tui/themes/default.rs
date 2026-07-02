use ratatui::style::Color;

use crate::tui::theme::Theme;

pub const THEME: Theme = Theme {
    text: Color::Rgb(192, 202, 245),           // #c0caf5
    text_secondary: Color::Rgb(169, 177, 214), // #a9b1d6
    text_dim: Color::Rgb(86, 95, 137),         // #565f89

    border: Color::Rgb(101, 109, 152),   // #656d98
    border_dim: Color::Rgb(65, 72, 104), // #414868

    accent: Color::Rgb(224, 175, 104),        // yellow: #e0af68
    accent_alt: Color::Rgb(115, 218, 202),    // teal: #73daca
    accent_search: Color::Rgb(187, 154, 247), // purple: #bb9af7
    accent_rose: Color::Rgb(247, 118, 142),   // red/pink: #f7768e

    heart: Color::Rgb(247, 118, 142),   // red: #f7768e
    online: Color::Rgb(158, 206, 106),  // green: #9ece6a
    warning: Color::Rgb(255, 158, 100), // orange: #ff9e64

    glow_high: Color::Rgb(187, 154, 247), // purple
    glow_mid: Color::Rgb(125, 207, 255),  // cyan: #7dcfff

    dot_red: Color::Rgb(247, 118, 142),    // red
    dot_yellow: Color::Rgb(224, 175, 104), // yellow
    dot_green: Color::Rgb(158, 206, 106),  // green

    terminal_bg: "#1a1b26",
    terminal_fg: "#c0caf5",
};
