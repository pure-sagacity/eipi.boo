// Solarized Dark - https://ethanschoonover.com/solarized
use ratatui::style::Color;

use crate::tui::theme::Theme;

pub const THEME: Theme = Theme {
    text: Color::Rgb(131, 148, 150),           // base0: #839496
    text_secondary: Color::Rgb(101, 123, 131), // base00: #657b83
    text_dim: Color::Rgb(88, 110, 117),        // base01: #586e75

    border: Color::Rgb(7, 54, 66),         // base02: #073642
    border_dim: Color::Rgb(0, 43, 54),     // base03: #002b36

    accent: Color::Rgb(181, 137, 0),       // yellow: #b58900
    accent_alt: Color::Rgb(42, 161, 152),  // cyan: #2aa198
    accent_search: Color::Rgb(108, 113, 196), // violet: #6c71c4
    accent_rose: Color::Rgb(211, 54, 130),    // magenta: #d33682

    heart: Color::Rgb(220, 50, 47),    // red: #dc322f
    online: Color::Rgb(133, 153, 0),   // green: #859900
    warning: Color::Rgb(203, 75, 22),  // orange: #cb4b16

    glow_high: Color::Rgb(108, 113, 196), // violet
    glow_mid: Color::Rgb(42, 161, 152),   // cyan

    dot_red: Color::Rgb(220, 50, 47),    // red
    dot_yellow: Color::Rgb(181, 137, 0), // yellow
    dot_green: Color::Rgb(133, 153, 0),  // green

    terminal_bg: "#002b36",
    terminal_fg: "#839496",
};
