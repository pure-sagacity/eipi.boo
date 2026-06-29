use ratatui::style::Color;

#[derive(Debug, Clone, Copy)]
pub struct Theme {
    // text
    pub text: Color,
    pub text_secondary: Color,
    pub text_dim: Color,

    // borders
    pub border: Color,
    pub border_dim: Color,

    // accents
    pub accent: Color,        // selected items, confession compose
    pub accent_alt: Color,    // reply compose, medium popularity
    pub accent_search: Color, // search
    pub accent_rose: Color,   // splash keyboard highlight

    // status
    pub heart: Color,
    pub online: Color,
    pub warning: Color, // quit dialog, messages

    // popularity tiers
    pub glow_high: Color,
    pub glow_mid: Color,

    // card dots
    pub dot_red: Color,
    pub dot_yellow: Color,
    pub dot_green: Color,
}
