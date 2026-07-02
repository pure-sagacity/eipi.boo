use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::Paragraph;

use crate::model::confession::{Confession, total_reactions};

use super::theme::Theme;

const SLOTS: [(i16, i16); 6] = [(-2, -1), (2, -2), (8, -1), (3, 1), (10, 1), (5, -3)];
const MIN_REACTIONS_TO_FLOAT: i64 = 3;

/// Map ASCII reaction tokens to single-char glyphs for floating display
/// so they don't look like terminal noise.
fn float_glyph(token: &str) -> &'static str {
    match token {
        "♥" => "♥",
        ":)" => "☺",
        ":/" => "·",
        "==" => "≡",
        "**" => "✧",
        "~~" => "~",
        "!!" => "!",
        "..." => "·",
        _ => "✦",
    }
}

pub fn render(frame: &mut Frame, confession: &Confession, area: Rect, tick: u64, theme: &Theme) {
    let tokens = visible_tokens(confession, tick);
    for (i, token) in tokens.iter().enumerate() {
        let glyph = float_glyph(token);
        let (dx, dy) = SLOTS[i % SLOTS.len()];
        let x = area.x as i16 + dx + ((tick as i16 + confession.id as i16 + i as i16) % 2);
        let y = area.y as i16 + dy - ((tick as i16 + i as i16) % 2);

        if x < 0 || y < 0 {
            continue;
        }

        let glyph_w = glyph.len() as u16;
        let rect = Rect::new(x as u16, y as u16, glyph_w, 1);
        if rect.right() > frame.area().right() || rect.bottom() > frame.area().bottom() {
            continue;
        }

        let style = if i == 0 {
            Style::default()
                .fg(theme.accent_alt)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.text_dim)
        };
        frame.render_widget(Paragraph::new(glyph).style(style), rect);
    }
}

fn visible_tokens(confession: &Confession, tick: u64) -> Vec<&str> {
    if total_reactions(confession) < MIN_REACTIONS_TO_FLOAT {
        return Vec::new();
    }

    let top = confession.reactions.iter().take(4).collect::<Vec<_>>();
    if top.is_empty() {
        return Vec::new();
    }

    let offset = ((tick / 2) as usize + confession.id as usize) % top.len();
    let count = top.len().min(3);
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        out.push(top[(offset + i) % top.len()].token.as_str());
    }
    out
}
