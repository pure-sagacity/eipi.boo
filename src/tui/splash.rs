use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

pub const TOTAL_FRAMES: u8 = 8;

pub fn render(frame: &mut Frame, splash_frame: u8, area: Rect) {
    let dim = Style::default().fg(Color::Indexed(242));
    let bright = Style::default().fg(Color::White);
    let accent = Style::default().fg(Color::Indexed(175)); // rose

    let mut lines: Vec<Line> = Vec::new();

    match splash_frame {
        0 => {
            // person sitting alone
            lines.push(Line::from(Span::styled("   ○", dim)));
            lines.push(Line::from(Span::styled("  /|\\", dim)));
            lines.push(Line::from(Span::styled("  / \\", dim)));
        }
        1 => {
            // first thought bubble
            lines.push(Line::from(Span::styled("      ·", dim)));
            lines.push(Line::from(Span::styled("   ○", dim)));
            lines.push(Line::from(Span::styled("  /|\\", dim)));
            lines.push(Line::from(Span::styled("  / \\", dim)));
        }
        2 => {
            // thought grows
            lines.push(Line::from(Span::styled("     ○", dim)));
            lines.push(Line::from(Span::styled("      ·", dim)));
            lines.push(Line::from(Span::styled("   ○", dim)));
            lines.push(Line::from(Span::styled("  /|\\", dim)));
            lines.push(Line::from(Span::styled("  / \\", dim)));
        }
        3 => {
            // thought cloud forms
            lines.push(Line::from(Span::styled("  ╭───────────────╮", accent)));
            lines.push(Line::from(Span::styled("  │               │", accent)));
            lines.push(Line::from(Span::styled("  ╰───────────────╯", accent)));
            lines.push(Line::from(Span::styled("     ○", dim)));
            lines.push(Line::from(Span::styled("      ·", dim)));
            lines.push(Line::from(Span::styled("   ○", dim)));
            lines.push(Line::from(Span::styled("  /|\\", dim)));
            lines.push(Line::from(Span::styled("  / \\", dim)));
        }
        4 => {
            // words appear
            lines.push(Line::from(Span::styled("  ╭───────────────╮", accent)));
            lines.push(Line::from(vec![
                Span::styled("  │ ", accent),
                Span::styled("i need to     ", bright),
                Span::styled("│", accent),
            ]));
            lines.push(Line::from(vec![
                Span::styled("  │ ", accent),
                Span::styled("tell someone. ", bright),
                Span::styled("│", accent),
            ]));
            lines.push(Line::from(Span::styled("  ╰───────────────╯", accent)));
            lines.push(Line::from(Span::styled("     ○", dim)));
            lines.push(Line::from(Span::styled("      ·", dim)));
            lines.push(Line::from(Span::styled("   ○", dim)));
            lines.push(Line::from(Span::styled("  /|\\", dim)));
            lines.push(Line::from(Span::styled("  / \\", dim)));
        }
        5 => {
            // decides
            lines.push(Line::from(Span::styled("  ╭───────────────╮", accent)));
            lines.push(Line::from(vec![
                Span::styled("  │ ", accent),
                Span::styled("yeah.         ", bright),
                Span::styled("│", accent),
            ]));
            lines.push(Line::from(vec![
                Span::styled("  │ ", accent),
                Span::styled("right now.    ", bright),
                Span::styled("│", accent),
            ]));
            lines.push(Line::from(Span::styled("  ╰───────────────╯", accent)));
            lines.push(Line::from(Span::styled("     ○", dim)));
            lines.push(Line::from(Span::styled("      ·", dim)));
            lines.push(Line::from(Span::styled("   ○", dim)));
            lines.push(Line::from(Span::styled("  /|\\", dim)));
            lines.push(Line::from(Span::styled("  / \\", dim)));
        }
        6 => {
            // thought pops, person walks
            lines.push(Line::from(Span::styled("        *", accent)));
            lines.push(Line::from(Span::styled("     *    *", accent)));
            lines.push(Line::from(Span::styled("        *", accent)));
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled("    ○", dim)));
            lines.push(Line::from(Span::styled("   /|\\→", dim)));
            lines.push(Line::from(Span::styled("   / \\", dim)));
        }
        _ => {
            // final frame - empty, about to transition
            lines.push(Line::from(""));
        }
    }

    let block_h = lines.len() as u16;
    let block_w = 21_u16; // widest line is ~21 chars

    let cx = area.x + area.width.saturating_sub(block_w) / 2;
    let cy = area.y + area.height.saturating_sub(block_h) / 2;

    let paragraph = Paragraph::new(lines);
    let render_area = Rect::new(cx, cy, block_w.min(area.width), block_h.min(area.height));
    frame.render_widget(paragraph, render_area);

    // show skip hint at the bottom
    if splash_frame < 7 {
        let hint = Line::from(Span::styled(
            "press any key to skip",
            Style::default().fg(Color::Indexed(238)),
        ))
        .centered();
        let hint_y = area.y + area.height.saturating_sub(2);
        frame.render_widget(
            Paragraph::new(hint),
            Rect::new(area.x, hint_y, area.width, 1),
        );
    }
}
