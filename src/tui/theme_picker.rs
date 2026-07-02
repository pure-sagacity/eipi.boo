use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Clear, Paragraph};

use super::theme::Theme;
use super::themes;

pub fn render(frame: &mut Frame, selected: usize, theme: &Theme, area: Rect) {
    let count = themes::ALL.len();
    let max_h = area.height.saturating_sub(4);
    let visible_items = (max_h.saturating_sub(2) as usize).min(count);
    let h = (visible_items as u16) + 2; // 2 border
    let w = 40_u16;

    let pw = w.min(area.width.saturating_sub(2));
    let ph = h.min(area.height.saturating_sub(2));
    let popup = Rect::new(
        (area.width.saturating_sub(pw)) / 2,
        (area.height.saturating_sub(ph)) / 2,
        pw,
        ph,
    );
    frame.render_widget(Clear, popup);

    let block = Block::bordered()
        .border_style(Style::default().fg(theme.accent))
        .title(" Theme ");
    let inner = block.inner(popup);
    frame.render_widget(block, popup);

    // scroll so selected item is always visible
    let scroll = if selected >= visible_items {
        selected - visible_items + 1
    } else {
        0
    };

    let mut lines: Vec<Line> = Vec::new();

    for (i, (name, t)) in themes::ALL
        .iter()
        .enumerate()
        .skip(scroll)
        .take(visible_items)
    {
        let is_selected = i == selected;
        let prefix = if is_selected { " ▶ " } else { "   " };

        let style = if is_selected {
            Style::default().fg(theme.text).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.text_secondary)
        };

        let swatch = vec![
            Span::styled(prefix, style),
            Span::styled(*name, style),
            Span::raw("  "),
            Span::styled("●", Style::default().fg(t.heart)),
            Span::styled("●", Style::default().fg(t.accent)),
            Span::styled("●", Style::default().fg(t.accent_alt)),
            Span::styled("●", Style::default().fg(t.glow_high)),
        ];

        lines.push(Line::from(swatch));
    }

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);

    // hint at bottom
    let hint_y = popup.y + popup.height;
    if hint_y < area.y + area.height {
        let hint = Line::from(vec![
            Span::styled("↑↓ ", Style::default().fg(Color::White)),
            Span::styled("select   ", Style::default().fg(Color::DarkGray)),
            Span::styled("⏎ ", Style::default().fg(Color::White)),
            Span::styled("apply   ", Style::default().fg(Color::DarkGray)),
            Span::styled("esc ", Style::default().fg(Color::White)),
            Span::styled("cancel", Style::default().fg(Color::DarkGray)),
        ])
        .centered();
        frame.render_widget(
            Paragraph::new(hint),
            Rect::new(area.x, hint_y, area.width, 1),
        );
    }
}
