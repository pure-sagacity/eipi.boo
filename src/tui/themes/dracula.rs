// Dracula - https://draculatheme.com
use ratatui::style::Color;

use crate::tui::theme::Theme;

pub const THEME: Theme = Theme {
    text: Color::Rgb(248, 248, 242),       // fg: #f8f8f2
    text_secondary: Color::Rgb(98, 114, 164), // comment: #6272a4
    text_dim: Color::Rgb(68, 71, 90),      // currentLine: #44475a

    border: Color::Rgb(68, 71, 90),        // currentLine: #44475a
    border_dim: Color::Rgb(40, 42, 54),    // bg: #282a36

    accent: Color::Rgb(241, 250, 140),     // yellow: #f1fa8c
    accent_alt: Color::Rgb(139, 233, 253), // cyan: #8be9fd
    accent_search: Color::Rgb(189, 147, 249), // purple: #bd93f9
    accent_rose: Color::Rgb(255, 121, 198),   // pink: #ff79c6

    heart: Color::Rgb(255, 85, 85),        // red: #ff5555
    online: Color::Rgb(80, 250, 123),      // green: #50fa7b
    warning: Color::Rgb(255, 184, 108),    // orange: #ffb86c

    glow_high: Color::Rgb(189, 147, 249),  // purple
    glow_mid: Color::Rgb(139, 233, 253),   // cyan

    dot_red: Color::Rgb(255, 85, 85),      // red
    dot_yellow: Color::Rgb(241, 250, 140), // yellow
    dot_green: Color::Rgb(80, 250, 123),   // green

    terminal_bg: "#282a36",
    terminal_fg: "#f8f8f2",
};
