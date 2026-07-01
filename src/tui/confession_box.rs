use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph, Wrap};

use crate::helper::consts;
use crate::model::confession::{self, Confession};

use super::theme::Theme;

pub fn render(
    frame: &mut Frame,
    c: &Confession,
    area: Rect,
    selected: bool,
    has_voted: bool,
    theme: &Theme,
) {
    let border_style = if selected {
        Style::default()
            .fg(theme.accent)
            .add_modifier(Modifier::BOLD)
    } else if c.votes > consts::VOTES_MAGENTA {
        Style::default()
            .fg(theme.glow_high)
            .add_modifier(Modifier::BOLD)
    } else if c.votes > consts::VOTES_CYAN {
        Style::default().fg(theme.glow_mid)
    } else {
        Style::default().fg(theme.text_dim)
    };

    let heart = if has_voted { "󰋑" } else { "♥" };
    let reply_str = if c.reply_count > 0 {
        format!("󰍧 {}  ", c.reply_count)
    } else {
        String::new()
    };
    let vote_display = format!("{}{} {}", reply_str, heart, c.votes);

    let age = confession::time_ago(&c.created_at);

    let mut block = Block::bordered()
        .border_style(border_style)
        .title_top(
            Line::from(Span::styled(
                format!(" {} ", age),
                Style::default().fg(theme.border),
            ))
            .right_aligned(),
        )
        .title_bottom(
            Line::from(Span::styled(
                vote_display,
                Style::default().fg(theme.heart),
            ))
            .right_aligned(),
        );

    if selected {
        block = block.title(Line::from(Span::styled(
            " ▶ ",
            Style::default().fg(theme.accent),
        )));
    }

    let text_style = if c.votes > consts::VOTES_MAGENTA {
        Style::default()
            .fg(theme.text)
            .add_modifier(Modifier::BOLD)
    } else if c.votes > consts::VOTES_CYAN {
        Style::default().fg(theme.text)
    } else {
        Style::default().fg(theme.text_secondary)
    };

    let paragraph = Paragraph::new(c.text.as_str())
        .block(block)
        .style(text_style)
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, area);
}
