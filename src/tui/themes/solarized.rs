// Solarized Dark - https://ethanschoonover.com/solarized
use ratatui::style::Color;

use crate::tui::theme::Theme;

pub const THEME: Theme = Theme {
    text: Color::Indexed(246),           // base0: #839496
    text_secondary: Color::Indexed(244), // base00: #657b83
    text_dim: Color::Indexed(241),       // base01: #586e75

    border: Color::Indexed(240),     // base02: #073642
    border_dim: Color::Indexed(236), // base03: #002b36

    accent: Color::Indexed(136),       // yellow: #b58900
    accent_alt: Color::Indexed(37),    // cyan: #2aa198
    accent_search: Color::Indexed(61), // violet: #6c71c4
    accent_rose: Color::Indexed(168),  // magenta: #d33682

    heart: Color::Indexed(160),   // red: #dc322f
    online: Color::Indexed(64),   // green: #859900
    warning: Color::Indexed(166), // orange: #cb4b16

    glow_high: Color::Indexed(61), // violet
    glow_mid: Color::Indexed(37),  // cyan

    dot_red: Color::Indexed(160),    // red
    dot_yellow: Color::Indexed(136), // yellow
    dot_green: Color::Indexed(64),   // green
};
