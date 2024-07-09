use ratatui::{prelude::*, widgets::*};
use std::io;

// meant to be called with show command
pub fn show_preview(buffers: Vec<(usize, String)>) {
    let (lines, table) = make_table(buffers);
    let mut l = 2;
    l += lines;
    let mut terminal = Terminal::with_options(
        CrosstermBackend::new(io::stdout()),
        TerminalOptions {
            viewport: Viewport::Inline(l as u16),
        },
    )
    .unwrap();

    terminal
        .draw(|frame| frame.render_widget(&table, frame.size()))
        .unwrap();
}

fn count_lines(blob: &str) -> u16 {
    let mut count = 1;
    for i in blob.chars() {
        count += if i == '\n' { 1 } else { 0 };
    }
    count as u16
}

fn make_table<'a>(buffers: Vec<(usize, String)>) -> (u16, Table<'a>) {
    let mut color = true;
    let mut total_lines = 0;
    let rows = buffers
        .into_iter()
        .map(|(i, st)| {
            color = !color;
            let h = count_lines(&st);
            total_lines += h;
            (i.to_string(), st, h, color)
        })
        .map(|(i, st, h, color)| {
            Row::new([i, st])
                .height(h)
                .style(Style::default().bg(if color {
                    Color::DarkGray
                } else {
                    Color::default()
                }))
        });
    let header = Row::new(vec!["Id", "Buffers"])
        .style(Style::default().fg(Color::Green).bold().underlined());
    let constraints = [Constraint::Ratio(1, 11), Constraint::Ratio(10, 11)];
    let table = Table::new(rows, constraints)
        .widths(constraints)
        .header(header.clone())
        .footer(header);
    (total_lines, table)
}

#[test]
fn t2() {
    let rows: Vec<_> = vec![
        "Hello".to_string(),
        r"asfdddddddddddddddddddddddsfasfdsafasfa\n\vsfasfasfasdfsafsa".to_string(),
        "uier".to_string(),
    ]
    .into_iter()
    .enumerate()
    .collect();

    let mut l = rows.len() as u16 + 2;
    let (lines, table) = make_table(rows);
    l += lines;

    let mut terminal = Terminal::with_options(
        CrosstermBackend::new(io::stdout()),
        TerminalOptions {
            viewport: Viewport::Inline(l as u16),
        },
    )
    .unwrap();

    terminal
        .draw(|frame| frame.render_widget(&table, frame.size()))
        .unwrap();
}
