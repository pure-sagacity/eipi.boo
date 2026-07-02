// Rose Pine Moon - https://rosepinetheme.com
use ratatui::style::Color;

use crate::tui::theme::Theme;

pub const THEME: Theme = Theme {
    text: Color::Rgb(224, 222, 244),       // text: #e0def4
    text_secondary: Color::Rgb(144, 140, 170), // subtle: #908caa
    text_dim: Color::Rgb(110, 106, 134),   // muted: #6e6a86

    border: Color::Rgb(57, 53, 82),        // overlay: #393552
    border_dim: Color::Rgb(42, 39, 63),    // surface: #2a273f

    accent: Color::Rgb(234, 154, 151),     // rose: #ea9a97
    accent_alt: Color::Rgb(156, 207, 216), // foam: #9ccfd8
    accent_search: Color::Rgb(196, 167, 231), // iris: #c4a7e7
    accent_rose: Color::Rgb(234, 154, 151),   // rose

    heart: Color::Rgb(235, 111, 146),      // love: #eb6f92
    online: Color::Rgb(62, 143, 176),      // pine: #3e8fb0
    warning: Color::Rgb(246, 193, 119),    // gold: #f6c177

    glow_high: Color::Rgb(196, 167, 231),  // iris
    glow_mid: Color::Rgb(156, 207, 216),   // foam

    dot_red: Color::Rgb(235, 111, 146),    // love
    dot_yellow: Color::Rgb(246, 193, 119), // gold
    dot_green: Color::Rgb(62, 143, 176),   // pine

    terminal_bg: "#232136",
    terminal_fg: "#e0def4",
};
