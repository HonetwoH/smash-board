mod config {
    // this modules will read and interpret config which is
    // as of now just the number of available buffers

    pub enum Base {
        Hexa,
        Octal,
        Decimal,
        HexaDecimal,
    }

    struct Config {
        available_buffers: Base,
    }
}

mod clipboard_sync {
    // will need to poll for changes in hopes on not hogging the cpu
}
mod grammer {
    // the core grammer
    use crate::config::Base;

    pub fn check(cap: Base) -> impl Fn(&str) -> Vec<u8> {
        let radix = match cap {
            Base::Hexa => 6,
            Base::Octal => 8,
            Base::Decimal => 10,
            Base::HexaDecimal => 16,
        };

        move |line: &str| {
            let points = line.as_bytes();
            let mut buffers = vec![];

            let ignore = |x: u8| {
                let x = char::from_u32(x as u32).unwrap();
                let redundant = ['.', ' ', ','];
                let mut found = false;
                for i in redundant {
                    found |= if i == x { true } else { false };
                }
                found
            };
            for token in points {
                if ignore(*token) {
                    continue;
                }
                // this should work and yeild only a single digit number for the given base
                if let Some(number) = char::from(*token).to_digit(radix) {
                    dbg!(number);
                    buffers.push(number as u8);
                } else {
                    panic!("Number higher than the Base");
                }
            }
            buffers
        }
    }
}

mod composer;

// mod cli;

// mod db;

fn main() {
    // { // testing the main app
    //     use crate::cli_app::{argument, ActionForCore};
    //     use crate::db::Db;

    //     let pastesbin = Db::new().unwrap();
    //     match argument() {
    //         ActionForCore::Show => pastesbin.show(8),
    //         ActionForCore::Compose => {}
    //         ActionForCore::Paste(new_paste) => {
    //             dbg!(&new_paste);
    //         }
    //         _ => panic!(),
    //     }
    // }
    {
        // testing the compose ui
        use crate::composer::render;
        render(vec!["pub fn render(items: Vec<String>) -> io::Result<()> k
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
    ", "{
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
}"].into_iter().map(|x| x.to_string()).collect());
    }
    // {
    //     // testing the grammer
    //     use crate::config::Base;
    //     use crate::grammer::check;
    //     check("1.2.3.4.5", Base::Hexa);
    // }
}
