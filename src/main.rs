use clap::{arg, ArgMatches, Command};

fn main() {
    match argument() {
        ActionForCore::Composed(x) | ActionForCore::Edit(x) => {
            dbg!(x);
        }
        _ => {}
    }
}

#[derive(Debug)]
enum ActionForCore {
    Show,
    Composed(String),
    Edit(String),
}

fn argument() -> ActionForCore {
    let collect_argument = |buffers: &ArgMatches| {
        buffers
            .get_raw("BUFFERS")
            .unwrap()
            .fold(String::new(), |mut arg, x| {
                arg.push_str(x.to_str().expect("Convertion failed"));
                arg
            })
    };
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("show", _)) => {
            // println!("Show them some thing");
            ActionForCore::Show
        }
        Some(("compose", buffers)) => {
            let parsed_arg = collect_argument(buffers);
            ActionForCore::Composed(parsed_arg)
        }
        Some(("edit", buffers)) => {
            let parsed_arg = collect_argument(buffers);
            ActionForCore::Edit(parsed_arg)
        }
        _ => unreachable!(),
    }
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
        .subcommand(
            Command::new("edit").short_flag('e').arg(
                arg!(<BUFFERS> ... "series of buffer").value_parser(clap::value_parser!(String)),
            ),
        )
}
