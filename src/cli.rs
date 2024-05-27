use std::fmt::Debug;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None, arg_required_else_help = true)]
struct Arg {
    #[clap(subcommand)]
    action: Action,
}
#[derive(Subcommand, Debug)]
enum Action {
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
    Compose,
}

pub(crate) fn args() {
    let args = match Arg::try_parse() {
        Ok(a) => a,
        Err(e) => {
            let _ = e.print();
            std::process::exit(0);
        }
    };
    match args.action {
        Action::Paste {
            buffer_sequence: Some(seq),
        } => {
            // parse the buffer sequence
            dbg!(pasrse_buf_seq(seq, 8));
        }
        Action::Copy { input_text } => {
            //TODO: check the mime type
        }
        _ => {}
    }
}

#[derive(Debug)]
struct ValueOverBase {}

fn pasrse_buf_seq(seq: String, base: u32) -> Result<Vec<u8>, ValueOverBase> {
    // all the numbers will be of single digit in the base the user choose in config
    seq.chars()
        .into_iter()
        .map(|n| n.to_digit(base))
        .fold(Ok(Vec::new()), |acc, x| {
            if acc.is_ok() {
                if let Some(n) = x {
                    acc.map(|mut ac| {
                        ac.push(n as u8);
                        ac
                    })
                } else {
                    Err(ValueOverBase {})
                }
            } else {
                acc
            }
        })
}
