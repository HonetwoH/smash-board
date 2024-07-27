use crate::widgets::{Preview, PromptText, ShuffleOperation};
use config::Base;

use std::io::{self, stdout};

use crossterm::{
    event::{self, Event, KeyCode},
    queue,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

enum Operation {
    ExitSuccessful,
    ExitError,
    ExitUnsure,
}

fn handle_events(
    text: &mut PromptText,
) -> io::Result<(Option<Operation>, Option<ShuffleOperation>)> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                if KeyCode::Enter == key.code || KeyCode::Esc == key.code {
                    return Ok((Some(Operation::ExitSuccessful), None));
                } else {
                    if let KeyCode::Char(x) = key.code {
                        return Ok((None, text.push(x)));
                    } else if let KeyCode::Backspace = key.code {
                        return Ok((None, text.pop()));
                    } else {
                        // dbg!(key.code);
                        return Ok((None, None));
                    }
                }
            }
        }
    }
    Ok((Some(Operation::ExitUnsure), None))
}

// The main function for in this module
pub fn compose_ui(base: Base, blobs: Vec<String>) -> io::Result<()> {
    // init for terminal
    queue!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    // setup widget
    let mut prompt_string = PromptText::new(base);
    let mut buffers = Preview::new(blobs);

    // the main loop
    let mut should_quit = false;
    while !should_quit {
        // event
        let (to_quit, operation_for_list) = handle_events(&mut prompt_string)?;
        match to_quit {
            None => should_quit = false,
            Some(op) => match op {
                Operation::ExitSuccessful => should_quit = true,
                Operation::ExitError => unreachable!(),
                Operation::ExitUnsure => todo!("Handle me PLEASE"),
            },
        }
        match operation_for_list {
            Some(op) => match op {
                ShuffleOperation::Pop(n) => buffers.unselect(n),
                ShuffleOperation::Push(n) => buffers.select(n),
            },
            None => {}
        }
        // render
        terminal.draw(|frame| layout_and_render(frame, &prompt_string, &buffers))?;
    }
    // exract out from buffer

    // deinit for terminal
    disable_raw_mode()?;
    queue!(stdout(), LeaveAlternateScreen)?;
    Ok(())
}

#[test]
fn t1() {
    let _ = compose_ui(
        Base::Octal,
        vec![
            "Goodbye world".to_string(),
            "Hello world".to_string(),
            "Goodbye world".to_string(),
            "Hello world".to_string(),
            "Goodbye world".to_string(),
            "Hello world".to_string(),
            "Hello world".to_string(),
            "Goodbye world".to_string(),
        ],
    );
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
