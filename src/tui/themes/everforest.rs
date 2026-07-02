// Everforest Dark - https://github.com/sainnhe/everforest
use ratatui::style::Color;

use crate::tui::theme::Theme;

pub const THEME: Theme = Theme {
    text: Color::Rgb(211, 198, 170),       // fg: #d3c6aa
    text_secondary: Color::Rgb(133, 146, 137), // grey1: #859289
    text_dim: Color::Rgb(79, 88, 94),      // bg4: #4f585e

    border: Color::Rgb(61, 72, 77),        // bg2: #3d484d
    border_dim: Color::Rgb(52, 63, 68),    // bg1: #343f44

    accent: Color::Rgb(219, 188, 127),     // yellow: #dbbc7f
    accent_alt: Color::Rgb(131, 192, 146), // aqua: #83c092
    accent_search: Color::Rgb(214, 153, 182), // purple: #d699b6
    accent_rose: Color::Rgb(214, 153, 182),   // purple

    heart: Color::Rgb(230, 126, 128),      // red: #e67e80
    online: Color::Rgb(167, 192, 128),     // green: #a7c080
    warning: Color::Rgb(230, 152, 117),    // orange: #e69875

    glow_high: Color::Rgb(214, 153, 182),  // purple
    glow_mid: Color::Rgb(127, 187, 179),   // blue: #7fbbb3

    dot_red: Color::Rgb(230, 126, 128),    // red
    dot_yellow: Color::Rgb(219, 188, 127), // yellow
    dot_green: Color::Rgb(167, 192, 128),  // green

    terminal_bg: "#2d353b",
    terminal_fg: "#d3c6aa",
};
