use clap::{arg, Command};

fn main() {
    argument();
}

#[derive(Debug)]
enum ActionForCore {
    Show,
    Composed(String),
}

fn argument() {
    let matches = dbg!(cli().get_matches());
    dbg!(match matches.subcommand() {
        Some(("show", _)) => {
            println!("Show them some thing");
            ActionForCore::Show
        }
        Some(("compose", buffers)) => {
            let parsed_arg =
                buffers
                    .get_raw("BUFFERS")
                    .unwrap()
                    .fold(String::new(), |mut arg, x| {
                        arg.push_str(x.to_str().expect("Convertion failed"));
                        arg
                    });
            ActionForCore::Composed(parsed_arg)
        }
        _ => unreachable!(),
    });
}

fn cli() -> Command {
    Command::new("sb")
        .about("New kind of clipboard")
        .arg_required_else_help(true)
        .subcommand(Command::new("show").short_flag('s'))
        .subcommand(
            Command::new("compose").short_flag('c').arg(
                arg!(<BUFFERS> ... "series of buffer").value_parser(clap::value_parser!(String)),
            ),
        )
}
