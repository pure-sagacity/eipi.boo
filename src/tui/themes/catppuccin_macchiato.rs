// Catppuccin Macchiato - https://catppuccin.com
use ratatui::style::Color;

use crate::tui::theme::Theme;

pub const THEME: Theme = Theme {
    text: Color::Rgb(202, 211, 245),           // text: #cad3f5
    text_secondary: Color::Rgb(147, 154, 183), // overlay2: #939ab7
    text_dim: Color::Rgb(110, 115, 141),       // overlay0: #6e738d

    border: Color::Rgb(73, 77, 100),    // surface1: #494d64
    border_dim: Color::Rgb(54, 58, 79), // surface0: #363a4f

    accent: Color::Rgb(238, 212, 159),        // yellow: #eed49f
    accent_alt: Color::Rgb(139, 213, 202),    // teal: #8bd5ca
    accent_search: Color::Rgb(198, 160, 246), // mauve: #c6a0f6
    accent_rose: Color::Rgb(245, 189, 230),   // pink: #f5bde6

    heart: Color::Rgb(237, 135, 150),   // red: #ed8796
    online: Color::Rgb(166, 218, 149),  // green: #a6da95
    warning: Color::Rgb(245, 169, 127), // peach: #f5a97f

    glow_high: Color::Rgb(198, 160, 246), // mauve
    glow_mid: Color::Rgb(139, 213, 202),  // teal

    dot_red: Color::Rgb(237, 135, 150),    // red
    dot_yellow: Color::Rgb(238, 212, 159), // yellow
    dot_green: Color::Rgb(166, 218, 149),  // green

    terminal_bg: "#24273a",
    terminal_fg: "#cad3f5",
};
