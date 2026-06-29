// Rose Pine - https://rosepinetheme.com
use ratatui::style::Color;

use crate::tui::theme::Theme;

pub const THEME: Theme = Theme {
    text: Color::Indexed(255),           // text: #e0def4
    text_secondary: Color::Indexed(249), // subtle: #908caa
    text_dim: Color::Indexed(243),

    border: Color::Indexed(60),          // highlight med: #44415a
    border_dim: Color::Indexed(59),      // highlight low: #21202e

    accent: Color::Indexed(217),         // rose: #ebbcba
    accent_alt: Color::Indexed(111),     // foam: #9ccfd8
    accent_search: Color::Indexed(183),  // iris: #c4a7e7
    accent_rose: Color::Indexed(217),    // rose

    heart: Color::Indexed(204),          // love: #eb6f92
    online: Color::Indexed(150),         // pine: #31748f
    warning: Color::Indexed(222),        // gold: #f6c177

    glow_high: Color::Indexed(183),      // iris
    glow_mid: Color::Indexed(111),       // foam

    dot_red: Color::Indexed(204),        // love
    dot_yellow: Color::Indexed(222),     // gold
    dot_green: Color::Indexed(150),      // pine
};
