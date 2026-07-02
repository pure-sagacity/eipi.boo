// Monokai Classic - https://monokai.pro
use ratatui::style::Color;

use crate::tui::theme::Theme;

pub const THEME: Theme = Theme {
    text: Color::Rgb(248, 248, 240),       // white: #f8f8f0
    text_secondary: Color::Rgb(143, 144, 138), // grey: #8f908a
    text_dim: Color::Rgb(51, 56, 66),      // base4: #333842

    border: Color::Rgb(46, 50, 60),        // base3: #2e323c
    border_dim: Color::Rgb(38, 41, 44),    // base2: #26292c

    accent: Color::Rgb(230, 219, 116),     // yellow: #e6db74
    accent_alt: Color::Rgb(102, 217, 239), // aqua: #66d9ef
    accent_search: Color::Rgb(174, 129, 255), // purple: #ae81ff
    accent_rose: Color::Rgb(249, 38, 114),    // pink: #f92672

    heart: Color::Rgb(249, 38, 114),       // pink: #f92672
    online: Color::Rgb(166, 226, 46),      // green: #a6e22e
    warning: Color::Rgb(253, 151, 31),     // orange: #fd971f

    glow_high: Color::Rgb(174, 129, 255),  // purple
    glow_mid: Color::Rgb(102, 217, 239),   // aqua

    dot_red: Color::Rgb(249, 38, 114),     // pink
    dot_yellow: Color::Rgb(230, 219, 116), // yellow
    dot_green: Color::Rgb(166, 226, 46),   // green

    terminal_bg: "#26292c",
    terminal_fg: "#f8f8f0",
};
