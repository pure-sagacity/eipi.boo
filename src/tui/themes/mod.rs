mod catppuccin;
mod catppuccin_frappe;
mod catppuccin_latte;
mod catppuccin_macchiato;
mod default;
mod dracula;
mod everforest;
mod gruvbox;
mod kanagawa;
mod monokai;
mod nord;
mod one_dark;
mod rose_pine;
mod rose_pine_dawn;
mod rose_pine_moon;
mod solarized;
mod tokyo_night;

use super::theme::Theme;

pub const ALL: &[(&str, Theme)] = &[
    ("default", default::THEME),
    ("rose-pine", rose_pine::THEME),
    ("rose-pine-moon", rose_pine_moon::THEME),
    ("rose-pine-dawn", rose_pine_dawn::THEME),
    ("catppuccin", catppuccin::THEME),
    ("catppuccin-latte", catppuccin_latte::THEME),
    ("catppuccin-frappe", catppuccin_frappe::THEME),
    ("catppuccin-macchiato", catppuccin_macchiato::THEME),
    ("dracula", dracula::THEME),
    ("gruvbox", gruvbox::THEME),
    ("nord", nord::THEME),
    ("tokyo-night", tokyo_night::THEME),
    ("kanagawa", kanagawa::THEME),
    ("everforest", everforest::THEME),
    ("one-dark", one_dark::THEME),
    ("monokai", monokai::THEME),
    ("solarized", solarized::THEME),
];
