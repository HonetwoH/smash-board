// the interative panel for the compose command
use crate::config::Base;
use crossterm::{
    event::{self, Event, KeyCode},
    queue,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};
use std::io::{self, stdout};

struct PromptText {
    field: String,
}

impl PromptText {
    fn new() -> Self {
        Self {
            field: String::from("█"),
        }
    }
    fn push(&mut self, x: char) {
        // TODO: need a constraint on the length at some point
        let cursor = self.field.pop().unwrap();
        self.field.push(x);
        self.field.push(cursor);
    }
    fn pop(&mut self) {
        // 3 because the cursor character is 3 byte unicode
        if self.field.len() > 3 {
            let cursor = self.field.pop().unwrap();
            let _ = self.field.pop().unwrap();
            self.field.push(cursor);
        }
    }
    fn dump(&self) -> &str {
        &self.field
    }
    fn return_input(&self) -> &str {
        &self.field[0..&self.field.len() - 3]
    }
}

fn handle_events(text: &mut PromptText) -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                if KeyCode::Enter == key.code || KeyCode::Esc == key.code {
                    return Ok(true);
                } else {
                    if let KeyCode::Char(x) = key.code {
                        text.push(x);
                        return Ok(false);
                    } else if let KeyCode::Backspace = key.code {
                        text.pop();
                        return Ok(false);
                    } else {
                        dbg!(key.code);
                        return Ok(false);
                    }
                }
            }
        }
    }
    Ok(false)
}

// for the buffer's view and the preview pane but without the highlight maybe
// TODO: this should be immune to  the trickery of blank spaces
fn process_items<'a>(items: Vec<String>, base: Base) -> impl Fn(Rect) -> Vec<Text<'a>> {
    move |area: Rect| -> Vec<Text<'a>> {
        let (max_width, max_height) = (area.width, area.height);
        items
            .iter()
            // TODO: improve this
            // .map(|blob| {
            //     let lines = blob.split('\n');
            //     lines
            //         .take((max_height as usize) / base as usize)
            //         .map(|line| {
            //             if line.len() < max_width.into() {
            //                 line
            //             } else {
            //                 line.get(0..max_width.into())
            //                     .expect("Some error in truncation")
            //             }
            //         })
            //         .fold(String::new(), |mut acc, line| {
            //             acc.push_str(line);
            //             acc.push('\n');
            //             acc
            //         })
            // })
            .enumerate()
            .map(|(i, x)| {
                let text = Text::from(x.to_owned());
                if i % 2 == 0 {
                    text.style(Style::default().bg(Color::Black))
                } else {
                    text.style(Style::default().bg(Color::DarkGray))
                }
            })
            .collect()
    }
}

// The preview widget this will used not only here but also in the the cli command show
// the the printing will be inlined hence it need 2 mode and nice interaface with db
// fn preview(items: Vec<&String>) { }

pub fn compose_ui(
    items: Vec<String>,
    parser: impl Fn(&str) -> Vec<u8>,
    base: Base,
) -> io::Result<()> {
    // init for terminal
    queue!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut prompt_string = PromptText::new();
    let mut list_state = ListState::default();
    // items will be populated by the fetch from the database
    let items = process_items(items, base);

    // the main loop
    let mut should_quit = false;
    while !should_quit {
        should_quit = handle_events(&mut prompt_string)?;
        terminal.draw(|frame| layout_and_render(frame, &prompt_string, &mut list_state, &items))?;
        parser(&prompt_string.return_input())
            .into_iter()
            .for_each(|x| list_state.select(Some(x as usize)));
    }

    // deinit for terminal
    disable_raw_mode()?;
    queue!(stdout(), LeaveAlternateScreen)?;
    Ok(())
}

// the main frame
fn layout_and_render<'a>(
    frame: &mut Frame,
    prompt: &PromptText,
    list_state: &mut ListState,
    items: &impl Fn(Rect) -> Vec<Text<'a>>,
) {
    let block_config = |title| {
        Block::default()
            .title_position(ratatui::widgets::block::Position::Top)
            .title_alignment(ratatui::layout::Alignment::Left)
            .title(title)
            .borders(Borders::ALL)
    };

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(3), Constraint::Fill(1)])
        .split(frame.size());
    let preview_and_buffers = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(Constraint::from_percentages([45, 55]))
        .split(main_layout[1]);

    let prompt = Paragraph::new(prompt.dump()).block(block_config("Prompt"));
    let preview = Paragraph::new("").block(block_config("Preview"));

    let buffers = List::new(items(preview_and_buffers[1]))
        .block(block_config("Buffers"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD | Modifier::ITALIC))
        .direction(ListDirection::TopToBottom);

    frame.render_widget(prompt, main_layout[0]);
    frame.render_widget(preview, preview_and_buffers[0]);
    frame.render_stateful_widget(buffers, preview_and_buffers[1], list_state);
}
