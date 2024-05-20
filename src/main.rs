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
    // will neqed to poll for changes in hopes on not hogging the cpu
}

mod grammer {
    use ratatui::buffer;

    // the grammer for
    use crate::config::Base;

    pub fn check(line: &str, cap: Base) -> Vec<u8> {
        let radix = match cap {
            Base::Hexa => 6,
            Base::Octal => 8,
            Base::Decimal => 10,
            Base::HexaDecimal => 16,
        };
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

mod composer;

mod cli_app {
    use clap::{arg, ArgMatches, Command};

    #[derive(Debug)]
    pub enum ActionForCore {
        // TODO: rename the enum
        /// Will show top 8 or 10 or 16 buffers
        Show,
        /// Will paste the content of specified buffer
        Paste(String),
        /// Will copy the given string to the db
        Copy(String),
        /// compose action will only handle the compostion in
        /// iteractive manner and will push it resultant to the
        /// top or on stdout depending to command
        Compose,
    }

    pub fn argument() -> ActionForCore {
        let collect_argument = |buffers: &ArgMatches| {
            let buf = buffers.get_raw("BUFFERS").unwrap();
            buf.fold(String::new(), |mut arg, x| {
                arg.push_str(x.to_str().expect("Convertion failed"));
                arg.push(' ');
                arg
            })
        };
        let matches = cli().get_matches();
        match matches.subcommand() {
            Some(("show", _)) => ActionForCore::Show,
            Some(("compose", _)) => ActionForCore::Compose,
            Some(("copy", buffers)) => {
                let parsed_arg = collect_argument(buffers);
                ActionForCore::Copy(parsed_arg)
            }
            Some(("paste", buffers)) => {
                let parsed_arg = collect_argument(buffers);
                ActionForCore::Paste(parsed_arg)
            }
            _ => unreachable!(),
        }
    }

    fn cli() -> Command {
        Command::new("sb")
            .about("New kind of clipboard")
            .arg_required_else_help(true)
            .subcommand(Command::new("paste").about("Paste text from buffer").arg(
                arg!(<BUFFERS> ... "series of buffer").value_parser(clap::value_parser!(String)),
            ))
            .subcommand(Command::new("copy").about("Copy text to buffer"))
            .subcommand(Command::new("show").about("Shows the recent buffers"))
            .subcommand(
                Command::new("compose").about("Compose existing buffer to make a new paste"),
            )
    }
}

mod db;

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
    // { // testing the compose ui
    //     use crate::composer::main;
    //     main();
    // }
    {
        // testing the grammer
        use crate::config::Base;
        use crate::grammer::check;
        check("1.2.3.4.5", Base::Hexa);
    }
}
