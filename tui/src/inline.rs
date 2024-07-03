use ratatui::{prelude::*, widgets::*};

// meant to be called with show command
pub fn show_preview<'a>(buffers: Vec<(usize, String)>) -> Table<'a> {
    let mut color = true;
    let rows = buffers
        .into_iter()
        .map(|(i, st)| {
            color = !color;
            (i.to_string(), st, color)
        })
        .map(|(i, st, color)| {
            Row::new([i, st]).style(Style::default().bg(if color {
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
    table
}

#[test]
fn t2() {
    use std::io;
    let rows: Vec<_> = vec![
        "Hello".to_string(),
        r"asfdddddddddddddddddddddddsfasfdsafasfa\n\vsfasfasfasdfsafsa".to_string(),
        "uier".to_string(),
    ]
    .into_iter()
    .enumerate()
    .collect();

    let l = (rows.len() + 2) as u16;
    let table = show_preview(rows);

    let mut terminal = Terminal::with_options(
        CrosstermBackend::new(io::stdout()),
        TerminalOptions {
            viewport: Viewport::Inline(l),
        },
    )
    .unwrap();

    terminal
        .draw(|frame| frame.render_widget(&table, frame.size()))
        .unwrap();
}
