use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::helper::consts;
use crate::model::confession;

use super::RenderState;

pub fn card_rect(state: &RenderState, area: Rect) -> Option<Rect> {
    if state.confessions.is_empty() {
        return None;
    }

    let pct = if area.width < 50 { 90 } else { 60 };
    let desired = (area.width * pct / 100).clamp(consts::CARD_MIN_W, consts::CARD_MAX_W);
    let card_w = desired.min(area.width.saturating_sub(2)) as usize;
    if card_w < consts::CARD_MIN_W as usize {
        return None;
    }
    let iw = card_w - 2;
    let idx = state.card_index.min(state.confessions.len() - 1);
    let wrapped = confession::wrap_text(&state.confessions[idx].text, iw.saturating_sub(4));
    let text_lines = wrapped.len() as u16;
    let card_h = 8 + text_lines;
    let show_char = area.height >= card_h + 4 + 2;
    let total_h = if show_char { card_h + 4 } else { card_h };
    if total_h > area.height {
        return None;
    }
    let cx = area.x + area.width.saturating_sub(card_w as u16) / 2;
    let cy = area.y + area.height.saturating_sub(total_h) / 2;
    Some(Rect::new(cx, cy, card_w as u16, total_h))
}

pub fn render(frame: &mut Frame, state: &RenderState, area: Rect) {
    let theme = &state.theme;

    if state.confessions.is_empty() {
        let hint = Paragraph::new("No confessions yet. Press [n] to write one.")
            .style(Style::default().fg(theme.text_dim));
        let cx = area.x + area.width.saturating_sub(46) / 2;
        let cy = area.y + area.height / 2;
        frame.render_widget(hint, Rect::new(cx, cy, 46.min(area.width), 1));
        return;
    }

    let idx = state.card_index.min(state.confessions.len() - 1);
    let c = &state.confessions[idx];

    // responsive card width: use more screen on small terminals
    let Some(card_rect) = card_rect(state, area) else {
        return;
    };
    let card_w = card_rect.width as usize;
    let iw = card_w - 2; // inner width between │ borders

    let wrapped = confession::wrap_text(&c.text, iw.saturating_sub(4));
    let text_lines = wrapped.len() as u16;

    // card: top(1) + dots(1) + sep(1) + pad(1) + text + pad(1) + sep(1) + footer(1) + bottom(1) = 8 + text
    let card_h = 8 + text_lines;
    let show_char = card_rect.height > card_h;

    let border = Style::default().fg(theme.text_dim);
    let dim = Style::default().fg(theme.text_dim);
    let text_style = Style::default().fg(theme.text);
    let age_style = Style::default().fg(theme.text_secondary);
    let char_style = Style::default().fg(theme.text_secondary);

    let age = confession::time_ago(&c.created_at);
    let reaction_count = confession::total_reactions(c);
    let love_count = confession::love_reactions(c);
    let position = format!("{}/{}", idx + 1, state.confessions.len());

    let mut lines: Vec<Line> = Vec::new();

    // ╭────╮
    lines.push(Line::from(Span::styled(
        format!("╭{}╮", "─".repeat(iw)),
        border,
    )));

    // │  ● ● ●              age │
    let age_display = format!(" {} ", age);
    let age_dcols = age.len() + 2;
    let dots_dcols = 7; // "  ● ● ●" = 7 display cols
    let dots_pad = iw.saturating_sub(dots_dcols + age_dcols);
    lines.push(Line::from(vec![
        Span::styled("│", border),
        Span::styled("  ", dim),
        Span::styled("●", Style::default().fg(theme.dot_red)),
        Span::styled(" ●", Style::default().fg(theme.dot_yellow)),
        Span::styled(" ●", Style::default().fg(theme.dot_green)),
        Span::raw(" ".repeat(dots_pad)),
        Span::styled(age_display, age_style),
        Span::styled("│", border),
    ]));

    // │──────│
    lines.push(Line::from(Span::styled(
        format!("│{}│", "─".repeat(iw)),
        border,
    )));

    // │      │
    lines.push(Line::from(vec![
        Span::styled("│", border),
        Span::raw(" ".repeat(iw)),
        Span::styled("│", border),
    ]));

    // text lines
    for line in &wrapped {
        let dcols = line.chars().count();
        let right_pad = iw.saturating_sub(dcols + 2);
        lines.push(Line::from(vec![
            Span::styled("│", border),
            Span::raw("  "),
            Span::styled(line.as_str(), text_style),
            Span::raw(" ".repeat(right_pad)),
            Span::styled("│", border),
        ]));
    }

    // │      │
    lines.push(Line::from(vec![
        Span::styled("│", border),
        Span::raw(" ".repeat(iw)),
        Span::styled("│", border),
    ]));

    // │──────│
    lines.push(Line::from(Span::styled(
        format!("│{}│", "─".repeat(iw)),
        border,
    )));

    // footer: │  ♡ 3  󰍧 2        3/12  │
    let (love_display, love_dcols) = if love_count > 0 {
        let s = format!("♥ {}", love_count);
        let dcols = 1 + 1 + love_count.to_string().len();
        (s, dcols)
    } else {
        (String::new(), 0)
    };

    let (reply_display, reply_dcols) = if c.reply_count > 0 {
        let s = format!("  \u{F0367} {}", c.reply_count);
        let dcols = 2 + 1 + 1 + c.reply_count.to_string().len();
        (s, dcols)
    } else {
        (String::new(), 0)
    };

    let (reaction_display, reaction_dcols) = if reaction_count > 0 {
        let s = format!("  ✦ {}", reaction_count);
        let dcols = 2 + 1 + 1 + reaction_count.to_string().len();
        (s, dcols)
    } else {
        (String::new(), 0)
    };

    let pos_dcols = position.len();
    let left_dcols = 2 + love_dcols + reply_dcols + reaction_dcols;
    let right_dcols = pos_dcols + 2;
    let footer_pad = iw.saturating_sub(left_dcols + right_dcols);

    lines.push(Line::from(vec![
        Span::styled("│", border),
        Span::raw("  "),
        Span::styled(love_display, Style::default().fg(theme.heart)),
        Span::styled(reply_display, Style::default().fg(theme.accent_alt)),
        Span::styled(reaction_display, Style::default().fg(theme.accent_search)),
        Span::raw(" ".repeat(footer_pad)),
        Span::styled(&position, age_style),
        Span::raw("  "),
        Span::styled("│", border),
    ]));

    // bottom border
    if show_char {
        let mid = iw / 2;
        lines.push(Line::from(Span::styled(
            format!("╰{}┬{}╯", "─".repeat(mid), "─".repeat(iw - mid - 1)),
            border,
        )));

        let center = mid + 1;
        let face = match c.text.len() {
            0..70 => "\\(^_^)/",
            70..150 => "\\(o_o)/",
            150..220 => "\\(>_<)/",
            _ => "\\(x_x)/",
        };
        lines.push(Line::from(Span::styled(
            format!("{}│", " ".repeat(center)),
            char_style,
        )));
        lines.push(Line::from(Span::styled(
            format!("{}{}", " ".repeat(center.saturating_sub(3)), face),
            char_style,
        )));
        lines.push(Line::from(Span::styled(
            format!("{}│", " ".repeat(center)),
            char_style,
        )));
        lines.push(Line::from(Span::styled(
            format!("{}/ \\", " ".repeat(center.saturating_sub(1))),
            char_style,
        )));
    } else {
        lines.push(Line::from(Span::styled(
            format!("╰{}╯", "─".repeat(iw)),
            border,
        )));
    }

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, card_rect);
    super::reactions::render(frame, c, card_rect, state.render_tick, theme);

    if reaction_count > consts::VOTES_GLOW {
        super::glow::render_ring(frame, reaction_count, card_rect, area, theme);
    }
}
