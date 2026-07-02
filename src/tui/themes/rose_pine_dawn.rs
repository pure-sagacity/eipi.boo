// Rose Pine Dawn - https://rosepinetheme.com
use ratatui::style::Color;

use crate::tui::theme::Theme;

pub const THEME: Theme = Theme {
    text: Color::Rgb(70, 66, 97),          // text: #464261
    text_secondary: Color::Rgb(121, 117, 147), // subtle: #797593
    text_dim: Color::Rgb(152, 147, 165),   // muted: #9893a5

    border: Color::Rgb(242, 233, 225),     // overlay: #f2e9e1
    border_dim: Color::Rgb(255, 250, 243), // surface: #fffaf3

    accent: Color::Rgb(215, 130, 126),     // rose: #d7827e
    accent_alt: Color::Rgb(86, 148, 159),  // foam: #56949f
    accent_search: Color::Rgb(144, 122, 169), // iris: #907aa9
    accent_rose: Color::Rgb(215, 130, 126),   // rose

    heart: Color::Rgb(180, 99, 122),       // love: #b4637a
    online: Color::Rgb(40, 105, 131),      // pine: #286983
    warning: Color::Rgb(234, 157, 52),     // gold: #ea9d34

    glow_high: Color::Rgb(144, 122, 169),  // iris
    glow_mid: Color::Rgb(86, 148, 159),    // foam

    dot_red: Color::Rgb(180, 99, 122),     // love
    dot_yellow: Color::Rgb(234, 157, 52),  // gold
    dot_green: Color::Rgb(40, 105, 131),   // pine

    terminal_bg: "#faf4ed",
    terminal_fg: "#464261",
};
