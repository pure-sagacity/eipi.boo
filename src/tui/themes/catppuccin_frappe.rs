// Catppuccin Frappe - https://catppuccin.com
use ratatui::style::Color;

use crate::tui::theme::Theme;

pub const THEME: Theme = Theme {
    text: Color::Rgb(198, 208, 245),       // text: #c6d0f5
    text_secondary: Color::Rgb(148, 156, 187), // overlay2: #949cbb
    text_dim: Color::Rgb(115, 121, 148),   // overlay0: #737994

    border: Color::Rgb(81, 87, 109),       // surface1: #51576d
    border_dim: Color::Rgb(65, 69, 89),    // surface0: #414559

    accent: Color::Rgb(229, 200, 144),     // yellow: #e5c890
    accent_alt: Color::Rgb(129, 200, 190), // teal: #81c8be
    accent_search: Color::Rgb(202, 158, 230), // mauve: #ca9ee6
    accent_rose: Color::Rgb(244, 184, 228),   // pink: #f4b8e4

    heart: Color::Rgb(231, 130, 132),      // red: #e78284
    online: Color::Rgb(166, 209, 137),     // green: #a6d189
    warning: Color::Rgb(239, 159, 118),    // peach: #ef9f76

    glow_high: Color::Rgb(202, 158, 230),  // mauve
    glow_mid: Color::Rgb(129, 200, 190),   // teal

    dot_red: Color::Rgb(231, 130, 132),    // red
    dot_yellow: Color::Rgb(229, 200, 144), // yellow
    dot_green: Color::Rgb(166, 209, 137),  // green

    terminal_bg: "#303446",
    terminal_fg: "#c6d0f5",
};
