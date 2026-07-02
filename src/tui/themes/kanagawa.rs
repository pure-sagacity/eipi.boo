// Kanagawa - https://github.com/rebelot/kanagawa.nvim
use ratatui::style::Color;

use crate::tui::theme::Theme;

pub const THEME: Theme = Theme {
    text: Color::Rgb(220, 215, 186),       // fujiWhite: #dcd7ba
    text_secondary: Color::Rgb(200, 192, 147), // oldWhite: #c8c093
    text_dim: Color::Rgb(84, 84, 109),     // sumiInk4: #54546d

    border: Color::Rgb(54, 54, 70),        // sumiInk3: #363646
    border_dim: Color::Rgb(42, 42, 55),    // sumiInk2: #2a2a37

    accent: Color::Rgb(230, 195, 132),     // carpYellow: #e6c384
    accent_alt: Color::Rgb(122, 168, 159), // waveAqua2: #7aa89f
    accent_search: Color::Rgb(149, 127, 184), // oniViolet: #957fb8
    accent_rose: Color::Rgb(210, 126, 153),   // sakuraPink: #d27e99

    heart: Color::Rgb(195, 64, 67),        // autumnRed: #c34043
    online: Color::Rgb(152, 187, 108),     // springGreen: #98bb6c
    warning: Color::Rgb(255, 160, 102),    // surimiOrange: #ffa066

    glow_high: Color::Rgb(149, 127, 184),  // oniViolet
    glow_mid: Color::Rgb(126, 156, 216),   // crystalBlue: #7e9cd8

    dot_red: Color::Rgb(195, 64, 67),      // autumnRed
    dot_yellow: Color::Rgb(230, 195, 132), // carpYellow
    dot_green: Color::Rgb(152, 187, 108),  // springGreen

    terminal_bg: "#1f1f28",
    terminal_fg: "#dcd7ba",
};
