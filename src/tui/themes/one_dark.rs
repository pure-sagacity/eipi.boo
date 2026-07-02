// One Dark - https://github.com/joshdick/onedark.vim
use ratatui::style::Color;

use crate::tui::theme::Theme;

pub const THEME: Theme = Theme {
    text: Color::Rgb(171, 178, 191),       // fg: #abb2bf
    text_secondary: Color::Rgb(92, 99, 112), // comments: #5c6370
    text_dim: Color::Rgb(75, 82, 99),      // gutter: #4b5263

    border: Color::Rgb(62, 68, 82),        // visual: #3e4452
    border_dim: Color::Rgb(44, 50, 60),    // cursor: #2c323c

    accent: Color::Rgb(229, 192, 123),     // yellow: #e5c07b
    accent_alt: Color::Rgb(86, 182, 194),  // cyan: #56b6c2
    accent_search: Color::Rgb(198, 120, 221), // purple: #c678dd
    accent_rose: Color::Rgb(198, 120, 221),   // purple

    heart: Color::Rgb(224, 108, 117),      // red: #e06c75
    online: Color::Rgb(152, 195, 121),     // green: #98c379
    warning: Color::Rgb(209, 154, 102),    // orange: #d19a66

    glow_high: Color::Rgb(198, 120, 221),  // purple
    glow_mid: Color::Rgb(97, 175, 239),    // blue: #61afef

    dot_red: Color::Rgb(224, 108, 117),    // red
    dot_yellow: Color::Rgb(229, 192, 123), // yellow
    dot_green: Color::Rgb(152, 195, 121),  // green

    terminal_bg: "#282c34",
    terminal_fg: "#abb2bf",
};
