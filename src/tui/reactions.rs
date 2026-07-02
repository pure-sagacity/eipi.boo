use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::Paragraph;

use crate::model::confession::Confession;

use super::theme::Theme;

const SLOTS: [(i16, i16); 6] = [(-2, -1), (2, -2), (8, -1), (3, 1), (10, 1), (5, -3)];

pub fn render(frame: &mut Frame, confession: &Confession, area: Rect, tick: u64, theme: &Theme) {
    let tokens = visible_tokens(confession, tick);
    for (i, token) in tokens.iter().enumerate() {
        let (dx, dy) = SLOTS[i % SLOTS.len()];
        let x = area.x as i16 + dx + ((tick as i16 + confession.id as i16 + i as i16) % 2);
        let y = area.y as i16 + dy - ((tick as i16 + i as i16) % 2);

        if x < 0 || y < 0 {
            continue;
        }

        let rect = Rect::new(x as u16, y as u16, token.len() as u16, 1);
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
        frame.render_widget(Paragraph::new(*token).style(style), rect);
    }
}

fn visible_tokens(confession: &Confession, tick: u64) -> Vec<&str> {
    let mut weighted = Vec::new();
    for reaction in &confession.reactions {
        let weight = reaction.count.clamp(1, 3) as usize;
        for _ in 0..weight {
            weighted.push(reaction.token.as_str());
        }
    }

    if weighted.is_empty() {
        return Vec::new();
    }

    let offset = ((tick / 2) as usize + confession.id as usize) % weighted.len();
    let count = weighted.len().min(3);
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        out.push(weighted[(offset + i) % weighted.len()]);
    }
    out
}
