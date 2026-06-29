mod catppuccin;
mod default;
mod rose_pine;
mod solarized;

use super::theme::Theme;

pub const ALL: &[(&str, Theme)] = &[
    ("default", default::THEME),
    ("rose-pine", rose_pine::THEME),
    ("catppuccin", catppuccin::THEME),
    ("solarized", solarized::THEME),
];
