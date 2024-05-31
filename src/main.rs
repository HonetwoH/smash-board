mod config {
    // this modules will read and interpret config which is
    // as of now just the number of available buffers

    #[derive(Clone, Copy)]
    pub enum Base {
        Hexa = 6,
        Octal = 8,
        Decimal = 10,
        HexaDecimal = 16,
    }

    struct Config {
        available_buffers: Base,
        // in seconds
        polling_rate: u8,
    }
}

mod clipboard_sync {
    // will need to poll for changes in hopes on not hogging the cpu
}
mod grammer {
    // the core grammer
    use crate::config::Base;

    pub(crate) struct HigherNumber {}

    pub(crate) fn check(cap: Base) -> impl Fn(&str) -> Vec<Result<u8, HigherNumber>> {
        move |line: &str| {
            let points = line.as_bytes();
            let mut buffers = vec![];

            let ignore = |x: u8| {
                let redundant = [b'.', b' ', b','];
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
                if let Some(number) = char::from(*token).to_digit(cap as u32) {
                    buffers.push(Ok(number as u8));
                } else {
                    buffers.push(Err(HigherNumber {}));
                }
            }
            buffers
        }
    }
}

mod cli;
mod composer;
mod db;

fn main() {
    use crate::composer::compose_ui;
    use crate::config::Base;
    use crate::db::Db;
    use crate::grammer::check;
    use cli::{args, Action};

    let base = Base::HexaDecimal;
    let parser = check(base);
    let pastes_db: Db = Db::new_connection(base).unwrap();
    match args() {
        Action::Show => {
            pastes_db.show().into_iter().for_each(|x| {
                println!("{}", x);
            });
        }
        Action::Compose => {
            let items = pastes_db.show();
            let _ = compose_ui(items, parser, base);
        }
        Action::Paste(_bufs) => {
            if let Some(paste) = pastes_db.peek() {
                dbg!(paste);
            };
        }
        Action::Copy(input) => {
            let _ = pastes_db.push(input);
        }
    }
}
