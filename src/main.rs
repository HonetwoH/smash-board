mod config {
    // this modules will read and interpret config which is
    // as of now just the number of available buffers
    use serde::Deserialize;
    use std::{env, fs};

    #[derive(Clone, Copy, Debug, Deserialize)]
    pub enum Base {
        Hexa = 6,
        Octal = 8,
        Decimal = 10,
        HexaDecimal = 16,
    }

    #[derive(Deserialize)]
    pub(super) struct Config {
        pub(super) base: Base,
        // in seconds
        polling_rate: u16,
    }

    pub fn read_config() -> Config {
        let config_filename = {
            let mut config_dir = env::var("XDG_CONFIG_HOME").expect(
                "Could not find XDG_CONFIG_HOME variable in environment, or maybe it is overloaded",
            );
            config_dir.push_str("/smashboard.toml");
            config_dir
        };
        let config_string = fs::read_to_string(config_filename)
            .expect("Could not find the config file \"smashboard.toml\" in config directory.");
        let config: Config = toml::from_str(&config_string).unwrap_or(Config {
            base: Base::Octal,
            polling_rate: 2,
        });
        config
    }

    #[test]
    fn t() {
        let toml_str = r#"
        available_buffers = "Hexa"
        polling_rate = 3
        "#;

        let decoded: Config = toml::from_str(toml_str).unwrap();
        println!("{:#?}", decoded.base);
    }
}

mod clipboard_sync {
    // will need to poll for changes in hopes on not hogging the cpu
}
mod grammer {
    // the core grammer
    use crate::config::Base;

    #[derive(Debug)]
    pub(crate) enum ParsingErrors {
        HigherOrderNumber,
    }

    //TODO: give some special meaning to each symbol add feature for graceful error returns
    pub(crate) fn check(cap: Base) -> impl Fn(&str) -> Vec<Result<u8, ParsingErrors>> {
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
                    buffers.push(Err(ParsingErrors::HigherOrderNumber));
                }
            }
            buffers
        }
    }
}

mod cli;
#[cfg(feature = "tui")]
mod composer;
mod db;

fn main() {
    #[cfg(feature = "tui")]
    use crate::composer::compose_ui;
    use crate::config::read_config;
    use crate::db::Db;
    use crate::grammer::check;
    use cli::{args, Action};

    let config = read_config();
    let base = config.base;
    let parser = check(base);
    let pastes_db: Db = Db::new_connection(base).unwrap();
    match args() {
        Action::Show => {
            pastes_db.show().into_iter().for_each(|x| {
                println!("{}", x);
            });
        }
        #[cfg(feature = "tui")]
        Action::Compose => {
            let items = pastes_db.show();
            let _ = compose_ui(items, parser, base);
        }
        Action::Paste(bufs) => {
            if let Some(buf) = bufs {
                let indices = parser(&buf);
                if indices.iter().all(|x| x.is_ok()) {
                    let buffers = indices.into_iter().map(|x| x.unwrap()).collect();
                    pastes_db.fetch(buffers).iter().for_each(|x| {
                        println!("{x}");
                    })
                } else {
                    eprintln!("Make sure the all the buffer indices are valid.");
                }
            } else {
                if let Some(paste) = pastes_db.peek() {
                    println!("{}", paste);
                }
            }
        }
        Action::Copy(input) => {
            let _ = pastes_db.push(input);
        }
    }
}
