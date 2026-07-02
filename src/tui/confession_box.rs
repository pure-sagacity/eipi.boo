use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::Widget;

use crate::helper::consts;
use crate::model::confession::{self, Confession, wrap_text};

use super::theme::Theme;

struct Cloud {
    text_lines: Vec<String>,
    age: String,
    vote_display: String,
    selected: bool,
    border_style: Style,
    text_style: Style,
    heart_style: Style,
}

impl Widget for Cloud {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width < 6 || area.height < 5 {
            return;
        }

        let w = area.width as usize;
        let x = area.x;
        let y = area.y;
        let inner_w = w.saturating_sub(4);
        let bulge_inner = w.saturating_sub(4);

        if self.selected {
            // row 0: cat ears   ∧  ∧
            buf.set_string(x + 1, y, "∧  ∧", self.border_style);

            // row 1: cat face + border  ( ·ω· )──────╰╮
            let dash_w = inner_w.saturating_sub(5);
            let cat_line = format!("( ·ω· ){}", "─".repeat(dash_w));
            buf.set_string(x, y + 1, &cat_line, self.border_style);
            buf.set_string(x + 7 + dash_w as u16, y + 1, "╰╮", self.border_style);
        } else {
            // row 0: top cap  ╭───────╮
            let top = format!(" ╭{}╮", "─".repeat(inner_w));
            buf.set_string(x, y, &top, self.border_style);

            // row 1: bulge top  ╭╯         ╰╮
            let bulge_top = format!("╭╯{}╰╮", " ".repeat(bulge_inner));
            buf.set_string(x, y + 1, &bulge_top, self.border_style);
        }

        // text rows
        let text_inner = w.saturating_sub(4);
        for (i, line) in self.text_lines.iter().enumerate() {
            let row = y + 2 + i as u16;
            if row >= area.y + area.height {
                break;
            }
            let padded = format!("{:<width$}", line, width = text_inner);
            buf.set_string(x, row, "│ ", self.border_style);
            buf.set_string(x + 2, row, &padded, self.text_style);
            buf.set_string(x + 2 + text_inner as u16, row, " │", self.border_style);
        }

        let text_count = self.text_lines.len() as u16;

        // bulge bottom: ╰╮         ╭╯
        let bulge_bot_y = y + 2 + text_count;
        if bulge_bot_y < area.y + area.height {
            let bulge_bot = format!("╰╮{}╭╯", " ".repeat(bulge_inner));
            buf.set_string(x, bulge_bot_y, &bulge_bot, self.border_style);
        }

        // bottom cap:  ╰───────╯
        let bot_y = y + 3 + text_count;
        if bot_y < area.y + area.height {
            let bot = format!(" ╰{}╯", "─".repeat(inner_w));
            buf.set_string(x, bot_y, &bot, self.border_style);

            // age on the bottom cap
            let age_str = format!(" {} ", self.age);
            let age_len = age_str.len() as u16;
            if age_len + 4 < area.width {
                let age_x = x + area.width - age_len - 2;
                buf.set_string(age_x, bot_y, &age_str, self.border_style);
            }
        }

        // thought bubble + vote info
        let info_y = y + 4 + text_count;
        if info_y < area.y + area.height {
            buf.set_string(x + 2, info_y, "○", self.border_style);
            buf.set_string(x + 4, info_y, &self.vote_display, self.heart_style);
        }
    }
}

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

    let text_style = if c.votes > consts::VOTES_MAGENTA {
        Style::default().fg(theme.text).add_modifier(Modifier::BOLD)
    } else if c.votes > consts::VOTES_CYAN {
        Style::default().fg(theme.text)
    } else {
        Style::default().fg(theme.text_secondary)
    };

    let heart = if has_voted { "󰋑" } else { "♥" };
    let reply_str = if c.reply_count > 0 {
        format!("󰍧 {}  ", c.reply_count)
    } else {
        String::new()
    };
    let vote_display = format!("{}{} {}", reply_str, heart, c.votes);

    let age = confession::time_ago(&c.created_at);
    let inner_w = (area.width as usize).saturating_sub(4);
    let text_lines = wrap_text(&c.text, inner_w);

    let cloud = Cloud {
        text_lines,
        age,
        vote_display,
        selected,
        border_style,
        text_style,
        heart_style: Style::default().fg(theme.heart),
    };

    frame.render_widget(cloud, area);
}
