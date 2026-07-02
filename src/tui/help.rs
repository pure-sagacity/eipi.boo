use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Clear, Paragraph, Wrap};

use super::theme::Theme;

pub fn render(frame: &mut Frame, area: Rect, theme: &Theme) {
    let popup = centered_popup(frame, area, 58, 19);
    let block = Block::bordered().border_style(Style::default().fg(theme.border));
    let inner = block.inner(popup);
    frame.render_widget(block, popup);

    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "eipi.boo",
            Style::default().fg(theme.text).add_modifier(Modifier::BOLD),
        ))
        .centered(),
        Line::from(Span::styled(
            "Confessions over ssh",
            Style::default().fg(theme.accent_search),
        ))
        .centered(),
        Line::from(Span::styled(
            "browse, reply, search, and disappear.",
            Style::default().fg(theme.text_dim),
        ))
        .centered(),
        rule(theme),
        header_line(inner.width, theme),
        Line::from(""),
        keybind("move", "↑↓←→ / hjkl", theme),
        keybind("mouse", "🖱 click / wheel", theme),
        keybind("tab", "select next confession", theme),
        keybind("enter", "open replies", theme),
        keybind("space", "open feed", theme),
        keybind("v", "upvote", theme),
        keybind("n", "new confession", theme),
        keybind("/", "search", theme),
        keybind("r", "reply", theme),
        keybind("T", "themes", theme),
        keybind("G", "jump to latest", theme),
        Line::from(""),
        rule(theme),
        Line::from(Span::styled(
            " github.com/pwnwriter/eipi.boo/issues/new",
            Style::default().fg(theme.text_dim),
        )),
    ];

    frame.render_widget(Paragraph::new(lines).wrap(Wrap { trim: false }), inner);
}

fn centered_popup(frame: &mut Frame, area: Rect, w: u16, h: u16) -> Rect {
    let pw = w.min(area.width.saturating_sub(2));
    let ph = h.min(area.height.saturating_sub(2));
    let popup = Rect::new(
        (area.width.saturating_sub(pw)) / 2,
        (area.height.saturating_sub(ph)) / 2,
        pw,
        ph,
    );
    frame.render_widget(Clear, popup);
    popup
}

fn keybind(key: &str, value: &str, theme: &Theme) -> Line<'static> {
    let key_width: usize = 18;
    let pad = key_width.saturating_sub(display_width(key));
    Line::from(vec![
        Span::styled(
            format!(" {}{} ", key, " ".repeat(pad)),
            Style::default().fg(theme.text).add_modifier(Modifier::BOLD),
        ),
        Span::styled("│ ", Style::default().fg(theme.border_dim)),
        Span::styled(value.to_string(), Style::default().fg(theme.text_secondary)),
    ])
}

fn rule(theme: &Theme) -> Line<'static> {
    Line::from(Span::styled(
        "────────────────────────────────────────────────────────",
        Style::default().fg(theme.border_dim),
    ))
}

fn header_line(width: u16, theme: &Theme) -> Line<'static> {
    let left = " keybinds";
    let right_label = "close: ";
    let right_value = "esc q ? enter";
    let used = display_width(left) + display_width(right_label) + display_width(right_value);
    let gap = usize::from(width).saturating_sub(used);

    Line::from(vec![
        Span::styled(left.to_string(), Style::default().fg(theme.accent)),
        Span::raw(" ".repeat(gap)),
        Span::styled(right_label.to_string(), Style::default().fg(theme.text_dim)),
        Span::styled(right_value.to_string(), Style::default().fg(theme.warning)),
    ])
}

fn display_width(text: &str) -> usize {
    text.chars().map(char_width).sum()
}

fn char_width(ch: char) -> usize {
    if ch.is_ascii() { 1 } else { 2 }
}
