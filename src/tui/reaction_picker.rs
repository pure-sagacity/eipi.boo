use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Clear, Paragraph};

use crate::model::reaction;

use super::theme::Theme;

pub fn render(frame: &mut Frame, anchor: Rect, selected: usize, area: Rect, theme: &Theme) {
    let popup = popup_rect(anchor, area);
    frame.render_widget(Clear, popup);
    let block = Block::bordered()
        .border_style(Style::default().fg(theme.accent))
        .title(" Reactions ");
    let inner = block.inner(popup);
    frame.render_widget(block, popup);

    let rows = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .split(inner);

    render_row(frame, rows[0], 0, selected, theme);
    render_row(frame, rows[1], 4, selected, theme);
    frame.render_widget(Paragraph::new(""), rows[2]);
    frame.render_widget(
        Paragraph::new(Line::from(Span::styled(
            reaction::meaning_at(selected),
            Style::default().fg(theme.text_secondary),
        ))),
        rows[3],
    );
}

fn render_row(frame: &mut Frame, area: Rect, start: usize, selected: usize, theme: &Theme) {
    let cols = Layout::horizontal(vec![Constraint::Length(5); 4]).split(area);
    for (offset, cell) in cols.iter().enumerate() {
        let idx = start + offset;
        if idx >= reaction::ALL.len() {
            break;
        }
        let token = reaction::ALL[idx].token;
        let style = if idx == selected {
            Style::default()
                .fg(theme.text)
                .bg(theme.accent)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.text_dim)
        };
        frame.render_widget(
            Paragraph::new(Line::from(vec![
                Span::styled("[", style),
                Span::styled(token, style),
                Span::styled("]", style),
            ])),
            *cell,
        );
    }
}

fn popup_rect(anchor: Rect, area: Rect) -> Rect {
    let width = 28u16.min(area.width.saturating_sub(2));
    let height = 6u16.min(area.height.saturating_sub(2));
    let mut x = anchor
        .x
        .saturating_add(anchor.width / 2)
        .saturating_sub(width / 2);
    if x + width > area.right() {
        x = area.right().saturating_sub(width);
    }

    let below_y = anchor.y.saturating_add(anchor.height).saturating_add(1);
    let y = if below_y + height <= area.bottom() {
        below_y
    } else {
        anchor.y.saturating_sub(height.saturating_add(1))
    };

    Rect::new(x.max(area.x), y.max(area.y), width, height)
}
