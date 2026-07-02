// Catppuccin Latte - https://catppuccin.com
use ratatui::style::Color;

use crate::tui::theme::Theme;

pub const THEME: Theme = Theme {
    text: Color::Rgb(76, 79, 105),         // text: #4c4f69
    text_secondary: Color::Rgb(124, 127, 147), // overlay2: #7c7f93
    text_dim: Color::Rgb(156, 160, 176),   // overlay0: #9ca0b0

    border: Color::Rgb(188, 192, 204),     // surface1: #bcc0cc
    border_dim: Color::Rgb(204, 208, 218), // surface0: #ccd0da

    accent: Color::Rgb(223, 142, 29),      // yellow: #df8e1d
    accent_alt: Color::Rgb(23, 146, 153),  // teal: #179299
    accent_search: Color::Rgb(136, 57, 239), // mauve: #8839ef
    accent_rose: Color::Rgb(234, 118, 203),  // pink: #ea76cb

    heart: Color::Rgb(210, 15, 57),        // red: #d20f39
    online: Color::Rgb(64, 160, 43),       // green: #40a02b
    warning: Color::Rgb(254, 100, 11),     // peach: #fe640b

    glow_high: Color::Rgb(136, 57, 239),   // mauve
    glow_mid: Color::Rgb(23, 146, 153),    // teal

    dot_red: Color::Rgb(210, 15, 57),      // red
    dot_yellow: Color::Rgb(223, 142, 29),  // yellow
    dot_green: Color::Rgb(64, 160, 43),    // green

    terminal_bg: "#eff1f5",
    terminal_fg: "#4c4f69",
};
