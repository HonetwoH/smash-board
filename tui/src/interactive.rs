use crate::widgets::{Preview, PromptText};
use crossterm::{
    event::{self, Event, KeyCode},
    queue,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};
use std::io::{self, stdout};

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

// The main function for in this module
pub fn compose_ui(blobs: Vec<String>) -> io::Result<()> {
    // init for terminal
    queue!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    // setup widget
    let mut prompt_string = PromptText::new();
    let mut buffers = Preview::new(blobs);

    // the main loop
    let mut should_quit = false;
    while !should_quit {
        // event
        should_quit = handle_events(&mut prompt_string)?;

        // render
        terminal.draw(|frame| layout_and_render(frame, &prompt_string, &buffers))?;
    }

    // deinit for terminal
    disable_raw_mode()?;
    queue!(stdout(), LeaveAlternateScreen)?;
    Ok(())
}

#[test]
fn t1() {
    let _ = compose_ui(vec![
        "Hello world".to_string(),
        "Goodbye world".to_string(),
        "Hello world".to_string(),
        "Goodbye world".to_string(),
        "Hello world".to_string(),
        "Goodbye world".to_string(),
        "Hello world".to_string(),
        "Hello world".to_string(),
        "Goodbye world".to_string(),
    ]);
}

// the main frame
fn layout_and_render<'a>(frame: &mut Frame, prompt: &PromptText, buffers: &Preview<'a>) {
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

    let prompt = Paragraph::new(prompt.dump()).block(block_config("Prompt"));

    // let scroll = Scrollbar::new(ScrollbarOrientation::VerticalRight);
    // let mut scrollstate = ScrollbarState::new(buffers.len()).position(0);

    frame.render_widget(prompt, main_layout[0]);
    frame.render_widget(buffers, main_layout[1]);
    // frame.render_stateful_widget(
    //     scroll,
    //     main_layout[1].inner(&Margin {
    //         horizontal: 0,
    //         vertical: 0,
    //     }),
    //     &mut scrollstate,
    // );
}
