// Gruvbox Dark - https://github.com/morhetz/gruvbox
use ratatui::style::Color;

use crate::tui::theme::Theme;

pub const THEME: Theme = Theme {
    text: Color::Rgb(235, 219, 178),       // fg: #ebdbb2
    text_secondary: Color::Rgb(168, 153, 132), // fg4: #a89984
    text_dim: Color::Rgb(146, 131, 116),   // gray: #928374

    border: Color::Rgb(80, 73, 69),        // bg2: #504945
    border_dim: Color::Rgb(60, 56, 54),    // bg1: #3c3836

    accent: Color::Rgb(250, 189, 47),      // yellow: #fabd2f
    accent_alt: Color::Rgb(142, 192, 124), // aqua: #8ec07c
    accent_search: Color::Rgb(211, 134, 155), // purple: #d3869b
    accent_rose: Color::Rgb(211, 134, 155),   // purple

    heart: Color::Rgb(251, 73, 52),        // red: #fb4934
    online: Color::Rgb(184, 187, 38),      // green: #b8bb26
    warning: Color::Rgb(254, 128, 25),     // orange: #fe8019

    glow_high: Color::Rgb(211, 134, 155),  // purple
    glow_mid: Color::Rgb(131, 165, 152),   // blue: #83a598

    dot_red: Color::Rgb(251, 73, 52),      // red
    dot_yellow: Color::Rgb(250, 189, 47),  // yellow
    dot_green: Color::Rgb(184, 187, 38),   // green

    terminal_bg: "#282828",
    terminal_fg: "#ebdbb2",
};
