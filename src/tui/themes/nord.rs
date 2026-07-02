// Nord - https://www.nordtheme.com
use ratatui::style::Color;

use crate::tui::theme::Theme;

pub const THEME: Theme = Theme {
    text: Color::Rgb(236, 239, 244),       // nord6: #eceff4
    text_secondary: Color::Rgb(216, 222, 233), // nord4: #d8dee9
    text_dim: Color::Rgb(76, 86, 106),     // nord3: #4c566a

    border: Color::Rgb(67, 76, 94),        // nord2: #434c5e
    border_dim: Color::Rgb(59, 66, 82),    // nord1: #3b4252

    accent: Color::Rgb(235, 203, 139),     // nord13 yellow: #ebcb8b
    accent_alt: Color::Rgb(136, 192, 208), // nord8 cyan: #88c0d0
    accent_search: Color::Rgb(180, 142, 173), // nord15 purple: #b48ead
    accent_rose: Color::Rgb(180, 142, 173),   // nord15

    heart: Color::Rgb(191, 97, 106),       // nord11 red: #bf616a
    online: Color::Rgb(163, 190, 140),     // nord14 green: #a3be8c
    warning: Color::Rgb(208, 135, 112),    // nord12 orange: #d08770

    glow_high: Color::Rgb(180, 142, 173),  // nord15 purple
    glow_mid: Color::Rgb(143, 188, 187),   // nord7 teal: #8fbcbb

    dot_red: Color::Rgb(191, 97, 106),     // nord11
    dot_yellow: Color::Rgb(235, 203, 139), // nord13
    dot_green: Color::Rgb(163, 190, 140),  // nord14

    terminal_bg: "#2e3440",
    terminal_fg: "#eceff4",
};
