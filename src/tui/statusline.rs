use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::model::confession;
use crate::server::input::InputMode;

use super::RenderState;

fn hint<'a>(key: &'a str, label: &'a str) -> Vec<Span<'a>> {
    vec![
        Span::styled(key, Style::default().fg(Color::White)),
        Span::styled(
            format!(" {}   ", label),
            Style::default().fg(Color::DarkGray),
        ),
    ]
}

pub fn render(frame: &mut Frame, state: &RenderState, area: Rect) {
    if area.height < 3 {
        return;
    }

    let theme = &state.theme;

    let info_area = Rect::new(area.x, area.y, area.width, 1);
    let rule_area = Rect::new(area.x, area.y + 1, area.width, 1);
    let hints_area = Rect::new(area.x, area.y + 2, area.width, 1);

    let info_line = match state.mode {
        InputMode::Browse | InputMode::CardView | InputMode::SearchResults => Line::from(vec![
            Span::styled(
                format!("{} confessions", state.total_confessions),
                Style::default().fg(theme.text_dim),
            ),
            Span::styled(" · ", Style::default().fg(theme.text_dim)),
            Span::styled(
                format!("{} humans", state.total_humans),
                Style::default().fg(theme.text_dim),
            ),
            Span::styled(" · ", Style::default().fg(theme.text_dim)),
            Span::styled(
                format!("{} online", state.online),
                Style::default().fg(theme.online),
            ),
            Span::styled(" · ", Style::default().fg(theme.text_dim)),
            Span::styled("pwnwriter/eipi.boo", Style::default().fg(theme.text_dim)),
        ])
        .centered(),
        _ => Line::from(""),
    };
    let info_p = Paragraph::new(info_line).style(Style::default().fg(theme.text_dim));
    frame.render_widget(info_p, info_area);

    let rule = "─".repeat(area.width as usize);
    let rule_p = Paragraph::new(rule).style(Style::default().fg(theme.text_dim));
    frame.render_widget(rule_p, rule_area);

    if let Some(msg) = state.message
        && matches!(
            state.mode,
            InputMode::Browse | InputMode::CardView | InputMode::ViewReplies
        )
    {
        let line = Line::from(msg).centered();
        let p = Paragraph::new(line).style(Style::default().fg(theme.warning));
        frame.render_widget(p, hints_area);
        return;
    }

    let mut spans: Vec<Span> = Vec::new();

    match state.mode {
        InputMode::Browse => {
            spans.extend(hint("↑↓/jk", "scroll"));
            spans.extend(hint("G", "last"));
            spans.extend(hint("tab", "select"));
            spans.extend(hint("v", "vote"));
            spans.extend(hint("⏎", "replies"));
            spans.extend(hint("␣", "feed"));
            spans.extend(hint("/", "search"));
            spans.extend(hint("n", "confess"));
            spans.extend(hint("T", "theme"));
            spans.extend(hint("q", "quit"));
        }
        InputMode::CardView => {
            spans.extend(hint("←→/hl", "prev/next"));
            spans.extend(hint("v", "vote"));
            spans.extend(hint("⏎", "replies"));
            spans.extend(hint("/", "search"));
            spans.extend(hint("n", "confess"));
            spans.extend(hint("T", "theme"));
            spans.extend(hint("␣", "canvas"));
        }
        InputMode::Compose => {
            spans.push(Span::styled(
                format!("{}/{}", state.compose_buf.len(), confession::MAX_LENGTH),
                Style::default().fg(theme.text_dim),
            ));
            spans.push(Span::raw("   "));
            spans.extend(hint("⏎", "submit"));
            spans.extend(hint("esc", "cancel"));
        }
        InputMode::ViewReplies => {
            spans.push(Span::styled(
                format!("{} replies", state.replies.len()),
                Style::default().fg(theme.text_dim),
            ));
            spans.push(Span::raw("   "));
            spans.extend(hint("r", "reply"));
            spans.extend(hint("↑↓/jk", "scroll"));
            spans.extend(hint("v", "vote"));
            spans.extend(hint("esc", "back"));
        }
        InputMode::ComposeReply => {
            if state.reply_name_phase {
                spans.push(Span::styled(
                    "name (optional): ",
                    Style::default().fg(theme.text_dim),
                ));
                spans.push(Span::styled(
                    format!("{}_", state.reply_name_buf),
                    Style::default().fg(theme.text),
                ));
                spans.push(Span::raw("   "));
                spans.extend(hint("⏎", "next"));
                spans.extend(hint("esc", "cancel"));
            } else {
                spans.push(Span::styled(
                    format!(
                        "{}/{}",
                        state.compose_buf.len(),
                        crate::model::reply::MAX_LENGTH
                    ),
                    Style::default().fg(theme.text_dim),
                ));
                spans.push(Span::raw("   "));
                spans.extend(hint("⏎", "submit"));
                spans.extend(hint("esc", "cancel"));
            }
        }
        InputMode::Search => {
            spans.extend(hint("⏎", "search"));
            spans.extend(hint("esc", "cancel"));
        }
        InputMode::SearchResults => {
            spans.push(Span::styled(
                format!(
                    "{}/{} matches",
                    state.search_index + 1,
                    state.search_result_count
                ),
                Style::default().fg(theme.accent_search),
            ));
            spans.push(Span::raw("   "));
            spans.extend(hint("←→/hl", "prev/next"));
            spans.extend(hint("v", "vote"));
            spans.extend(hint("⏎", "replies"));
            spans.extend(hint("esc", "back"));
        }
        InputMode::ConfirmQuit | InputMode::Splash | InputMode::ThemePicker | InputMode::Help => {}
    }

    let line = Line::from(spans).centered();
    frame.render_widget(Paragraph::new(line), hints_area);
}
