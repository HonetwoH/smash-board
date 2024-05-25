// the interative panel for the compose command
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
    if event::poll(std::time::Duration::from_millis(500))? {
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

// for the buffer's view
// TODO: this should be immune to  the trickery of blank spaces
fn process_items<'a>(items: Vec<String>) -> impl Fn(Rect) -> Vec<Text<'a>> {
    move |area: Rect| -> Vec<Text<'a>> {
        let (max_width, max_height) = (area.width, area.height);
        items
            .iter()
            .map(|blob| {
                let lines = blob.split('\n');
                lines
                    .take(max_height as usize / 6)
                    .map(|line| {
                        if line.len() < max_width.into() {
                            line
                        } else {
                            line.get(0..max_width.into())
                                .expect("Some error in truncation")
                        }
                    })
                    .fold(String::new(), |mut acc, line| {
                        acc.push_str(line);
                        acc.push('\n');
                        acc
                    })
            })
            .enumerate()
            .map(|(i, x)| {
                let text = Text::from(x);
                if i % 2 == 0 {
                    text.style(Style::default().bg(Color::Black))
                } else {
                    text.style(Style::default().bg(Color::DarkGray))
                }
            })
            .collect()
    }
}

// TODO: The preview will require some smart trickery as the good old'
// appending will not do the job, and honestly should not be used

pub fn compose_ui(items: Vec<String>, parser: impl Fn(&str) -> Vec<u8>) -> io::Result<()> {
    // init for terminal
    queue!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    // TODO: maby this should be passed into the function rather than declared here
    let mut prompt_string = PromptText::new();
    let mut list_state = ListState::default();
    // items will be populated by the fetch from the database
    let items = process_items(items);

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

    // TODO: seprate the layout from or delay the process till the last moment
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(3), Constraint::Fill(1)])
        .split(frame.size());
    let preview_and_buffers = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(Constraint::from_percentages([45, 55]))
        .split(main_layout[1]);

    let prompt = Paragraph::new(prompt.dump()).block(block_config("Prompt"));
    //TODO: Need to work on the preview too.
    let preview = Paragraph::new("").block(block_config("Preview"));

    let buffers = List::new(items(preview_and_buffers[1]))
        .block(block_config("Buffers"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD | Modifier::ITALIC))
        .direction(ListDirection::TopToBottom);

    frame.render_widget(prompt, main_layout[0]);
    frame.render_widget(preview, preview_and_buffers[0]);
    frame.render_stateful_widget(buffers, preview_and_buffers[1], list_state);
}

#[test]
fn test_compose_ui() {
    // testing the compose ui
    use crate::composer::compose_ui;
    use crate::config::Base;
    use crate::grammer::check;

    let parser = check(Base::Octal);
    compose_ui(
        vec![
            "pub fn render(items: Vec<String>) -> io::Result<()> k
    // init for terminal
    queue!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    // TODO: maby this should be passed into the function rather than declared here
    let mut prompt_string = PromptText::new();
    let mut list_state = ListState::default();
    // items will be populated by the fetch from the database
    let items = process_items(items);
    
    // the main loop
    ",
            "{
    if event::poll(std::time::Duration::from_millis(500))? {
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
}",
        ]
        .into_iter()
        .map(|x| x.to_string())
        .collect(),
        parser,
    );
}
