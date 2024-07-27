pub mod db;
pub mod grammar;
pub mod cli {
    use clap::{Parser, Subcommand};
    use std::fmt::Debug;

    #[derive(Parser, Debug)]
    #[clap(version, about, long_about = None, arg_required_else_help = true)]
    struct Arg {
        #[clap(subcommand)]
        action: Command,
    }
    #[derive(Subcommand, Debug)]
    enum Command {
        /// Will show upto 6, 8, 10 or 16 buffers
        Show,
        /// Paste the content (of specified buffer)
        Paste {
            #[clap(value_enum)]
            buffer_sequence: Option<String>,
        },
        /// Copy the given string to the db
        Copy {
            #[clap(value_enum)]
            input_text: String,
        },
        /// Compose together buffer interactively
        #[cfg(feature = "interactive")]
        Compose,
    }

    pub enum Action {
        Paste(Option<String>),
        Copy(String),
        Show,
        #[cfg(feature = "interactive")]
        Compose,
    }

    pub fn args() -> Action {
        let args = match Arg::try_parse() {
            Ok(a) => a,
            Err(e) => {
                let _ = e.print();
                std::process::exit(0);
            }
        };
        match args.action {
            Command::Paste { buffer_sequence } => {
                // parse the buffer sequence
                Action::Paste(buffer_sequence)
            }
            Command::Copy { input_text } => {
                //TODO: check the mime type
                Action::Copy(input_text)
            }
            Command::Show => Action::Show,
            #[cfg(feature = "interactive")]
            Command::Compose => Action::Compose,
        }
    }
}
