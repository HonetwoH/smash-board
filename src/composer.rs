// the interative panel for the compose command
use std::io::{self, stdout};

use crossterm::{
    event::{self, Event, KeyCode},
    queue,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};

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
        let cursor = self.field.pop().unwrap();
        self.field.push(x);
        self.field.push(cursor);
    }
    fn pop(&mut self) {
        // 3 because the cursor character is unicode
        if self.field.len() > 3 {
            let cursor = self.field.pop().unwrap();
            let _ = self.field.pop().unwrap();
            self.field.push(cursor);
        }
    }
    fn dump(&self) -> &str {
        &self.field
    }
}

pub fn main() -> io::Result<()> {
    queue!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut prompt_string = PromptText::new();

    let mut should_quit = false;
    while !should_quit {
        should_quit = handle_events(&mut prompt_string)?;
        terminal.draw(|frame| layout(frame, &prompt_string))?;
    }

    disable_raw_mode()?;
    queue!(stdout(), LeaveAlternateScreen)?;

    Ok(())
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

// the main frame
fn layout(frame: &mut Frame, prompt: &PromptText) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(12), Constraint::Percentage(88)])
        .split(frame.size());
    let preview_and_buffers = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(35), Constraint::Percentage(65)])
        .split(main_layout[1]);
    let prompt =
        Paragraph::new(prompt.dump()).block(Block::default().title("prompt").borders(Borders::ALL));
    let preview =
        Paragraph::new("Preview").block(Block::default().title("Preview").borders(Borders::ALL));
    let buffer =
        Paragraph::new("Buffer").block(Block::default().title("Buffer").borders(Borders::ALL));
    frame.render_widget(prompt, main_layout[0]);
    frame.render_widget(preview, preview_and_buffers[0]);
    frame.render_widget(buffer, preview_and_buffers[1]);
}
