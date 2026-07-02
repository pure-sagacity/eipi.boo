use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use super::theme::Theme;

pub const TOTAL_FRAMES: u8 = 9;

// Characters typed in sequence: e i p i . b o o
const TYPED: &[char] = &['e', 'i', 'p', 'i', '.', 'b', 'o', 'o'];

fn is_key_active(ch: char, typed_count: usize) -> bool {
    if typed_count == 0 {
        return false;
    }
    let current_idx = typed_count - 1;
    if current_idx < TYPED.len() {
        TYPED[current_idx] == ch
    } else {
        false
    }
}

fn build_key(label: &str, active: bool, theme: &Theme) -> Span<'static> {
    if active {
        Span::styled(
            format!(" {} ", label),
            Style::default().fg(Color::Black).bg(theme.accent_rose),
        )
    } else {
        Span::styled(
            format!(" {} ", label),
            Style::default().fg(theme.text_secondary),
        )
    }
}

pub fn render(frame: &mut Frame, splash_frame: u8, area: Rect, theme: &Theme) {
    // frame 0: empty keyboard, no typing
    // frame 1-8: type each character of "eipi.boo"
    // frame 9: keyboard fades, text stays, transition

    let typed_count = if splash_frame == 0 {
        0
    } else {
        (splash_frame as usize).min(TYPED.len())
    };

    let left_keys: [[char; 5]; 3] = [
        ['q', 'w', 'e', 'r', 't'],
        ['a', 's', 'd', 'f', 'g'],
        ['z', 'x', 'c', 'v', 'b'],
    ];
    let right_keys: [[char; 5]; 3] = [
        ['y', 'u', 'i', 'o', 'p'],
        ['h', 'j', 'k', 'l', ';'],
        ['n', 'm', ',', '.', '/'],
    ];

    let border = Style::default().fg(theme.text_dim);
    let gap = "       "; // gap between halves

    let mut lines: Vec<Line> = Vec::new();

    // typed text above keyboard
    let typed_text: String = TYPED[..typed_count].iter().collect();
    lines.push(
        Line::from(vec![
            Span::styled(&typed_text, Style::default().fg(theme.accent_rose)),
            Span::styled("_", Style::default().fg(theme.text)),
        ])
        .centered(),
    );
    lines.push(Line::from(""));

    // build keyboard rows
    for row in 0..3 {
        // top border of row
        let mut top_spans: Vec<Span> = Vec::new();
        // left half
        for col in 0..5 {
            if col == 0 {
                top_spans.push(Span::styled("┌───", border));
            } else {
                top_spans.push(Span::styled("┬───", border));
            }
        }
        top_spans.push(Span::styled("┐", border));
        top_spans.push(Span::raw(gap));
        // right half
        for col in 0..5 {
            if col == 0 {
                top_spans.push(Span::styled("┌───", border));
            } else {
                top_spans.push(Span::styled("┬───", border));
            }
        }
        top_spans.push(Span::styled("┐", border));
        lines.push(Line::from(top_spans));

        // key labels row
        let mut key_spans: Vec<Span> = Vec::new();
        // left half
        for &ch in &left_keys[row] {
            let active = is_key_active(ch, typed_count);
            key_spans.push(Span::styled("│", border));
            key_spans.push(build_key(&ch.to_string(), active, theme));
        }
        key_spans.push(Span::styled("│", border));
        key_spans.push(Span::raw(gap));
        // right half
        for &ch in &right_keys[row] {
            let active = is_key_active(ch, typed_count);
            key_spans.push(Span::styled("│", border));
            key_spans.push(build_key(&ch.to_string(), active, theme));
        }
        key_spans.push(Span::styled("│", border));
        lines.push(Line::from(key_spans));
    }

    // bottom border for row 3 + thumb keys
    // left: └───┴───┴───┼───┼───┤
    let mut bot_spans: Vec<Span> = Vec::new();
    for col in 0..5 {
        if col == 0 {
            bot_spans.push(Span::styled("└───", border));
        } else if col == 3 {
            bot_spans.push(Span::styled("┼───", border));
        } else {
            bot_spans.push(Span::styled("┴───", border));
        }
    }
    bot_spans.push(Span::styled("┤", border));
    bot_spans.push(Span::raw(gap));
    // right: ├───┼───┴───┴───┴───┘
    for col in 0..5 {
        if col == 0 {
            bot_spans.push(Span::styled("├───", border));
        } else if col == 2 {
            bot_spans.push(Span::styled("┼───", border));
        } else {
            bot_spans.push(Span::styled("┴───", border));
        }
    }
    bot_spans.push(Span::styled("┘", border));
    lines.push(Line::from(bot_spans));

    // thumb row
    lines.push(Line::from(vec![
        Span::raw("            "),
        Span::styled("│", border),
        build_key("⌥", false, theme),
        Span::styled("│", border),
        build_key("⎵", false, theme),
        Span::styled("│", border),
        Span::raw(gap),
        Span::styled("│", border),
        build_key("⏎", false, theme),
        Span::styled("│", border),
        build_key("⌘", false, theme),
        Span::styled("│", border),
    ]));

    // thumb bottom border
    lines.push(Line::from(vec![
        Span::raw("            "),
        Span::styled("└───┴───┘", border),
        Span::raw(gap),
        Span::styled("└───┴───┘", border),
    ]));

    let block_h = lines.len() as u16;
    let block_w = 49_u16; // approximate total width
    let cx = area.x + area.width.saturating_sub(block_w) / 2;
    let cy = area.y + area.height.saturating_sub(block_h) / 2;

    let paragraph = Paragraph::new(lines);
    let render_area = Rect::new(cx, cy, block_w.min(area.width), block_h.min(area.height));
    frame.render_widget(paragraph, render_area);
}
