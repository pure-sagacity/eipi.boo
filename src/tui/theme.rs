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

    // terminal colors (OSC escape sequences)
    pub terminal_bg: &'static str, // hex like "#1e1e2e"
    pub terminal_fg: &'static str, // hex like "#cdd6f4"
}

impl Theme {
    /// Returns OSC escape bytes to set terminal bg/fg colors
    pub fn osc_bytes(&self) -> Vec<u8> {
        format!(
            "\x1b]11;{}\x1b\\\x1b]10;{}\x1b\\",
            self.terminal_bg, self.terminal_fg
        )
        .into_bytes()
    }

    /// Returns OSC escape bytes to reset terminal colors to default
    pub fn osc_reset() -> Vec<u8> {
        b"\x1b]111\x1b\\\x1b]110\x1b\\".to_vec()
    }
}
