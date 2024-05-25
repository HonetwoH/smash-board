use std::fmt::Debug;

use clap::{Arg, ArgAction, Command};

#[derive(Debug)]
pub enum Action {
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

pub fn argument() -> Option<Action> {
    let matches = cli().get_matches();
    // dbg!(&matches.get_one::<String>("paste").unwrap());
    dbg!(&matches);
    dbg!(matches.ids());
    // panic!("Done there");
    match matches.ids().nth(0).map(|id| id.as_str()).unwrap() {
        "show" => Some(Action::Show),
        "compose" => Some(Action::Compose),
        "paste" => {
            let argument: String = matches
                .get_one::<String>("paste")
                .map(|x| x.to_owned())
                .unwrap_or_default();
            Some(Action::Paste(argument))
        }
        "copy" => {
            let argument: String = matches
                .get_one::<String>("copy")
                .map(|x| x.to_owned())
                .unwrap_or_default();
            Some(Action::Copy(argument))
        }
        _ => None,
    }
}

fn cli() -> Command {
    Command::new("sb")
        .about("New kind of clipboard")
        .arg_required_else_help(true)
        .arg(
            Arg::new("copy")
                .exclusive(true)
                .short('i')
                .long("copy")
                .id("copy")
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("paste")
                .exclusive(true)
                .short('o')
                .long("paste")
                .id("paste")
                .default_value("0")
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("compose")
                .exclusive(true)
                .short('c')
                .long("compose")
                .id("compose")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("show")
                .exclusive(true)
                .short('s')
                .long("show")
                .id("show")
                .action(ArgAction::SetTrue),
        )
}
