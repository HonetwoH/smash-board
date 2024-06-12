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
/*
// WIP
struct BufferPane {
    processed_buffers: Vec<String>,
    // some internal stuff
    // Block will take care of the borders and stuff
    // draw in inner area
    block: Block,
    // Then will need something like list with scrollbar with mutliple select
    scrollbar: Scrollbar,
}

// this will require a big state struct
impl BufferPane {
    fn new(items: Vec<String>) -> Self {
        // Self {
        //     processed_buffers: process(items),
        //     ..
        // }
        todo!("")
    }

    fn process(items: Vec<String>) {}
}
*/
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

pub fn compose_ui() -> io::Result<()> {
    // init for terminal
    queue!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut prompt_string = PromptText::new();

    // the main loop
    let mut should_quit = false;
    while !should_quit {
        should_quit = handle_events(&mut prompt_string)?;
        terminal.draw(|frame| layout_and_render(frame, &prompt_string))?;
    }

    // deinit for terminal
    disable_raw_mode()?;
    queue!(stdout(), LeaveAlternateScreen)?;
    Ok(())
}

#[test]
fn t1() {
    compose_ui();
}

// the main frame
fn layout_and_render<'a>(frame: &mut Frame, prompt: &PromptText) {
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
    //TODO: make a new widget for these
    let preview = Paragraph::new("").block(block_config("Preview"));
    let buffers = List::new([Text::from("Hello world"), Text::from("goodbye world")])
        .block(block_config("Buffers"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD | Modifier::ITALIC))
        .direction(ListDirection::TopToBottom);
    let scroll = Scrollbar::new(ScrollbarOrientation::VerticalRight);
    let mut scrollstate = ScrollbarState::new(buffers.len()).position(0);

    frame.render_widget(prompt, main_layout[0]);
    frame.render_widget(preview, preview_and_buffers[0]);
    frame.render_widget(buffers, preview_and_buffers[1]);
    frame.render_stateful_widget(
        scroll,
        preview_and_buffers[1].inner(&Margin {
            horizontal: 0,
            vertical: 0,
        }),
        &mut scrollstate,
    );
}
