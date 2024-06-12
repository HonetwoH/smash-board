use lib::cli::{args, Action};
#[cfg(feature = "read-config")]
use lib::config::read_config;
use lib::config::Config;
use lib::db::Db;
use lib::grammar::check;

fn main() {
    // #[cfg(feature = "tui")]
    // use crate::composer::compose_ui;
    // use crate::config::read_config;
    // use crate::grammer::check;
    let config = Config::default();

    #[cfg(feature = "read-config")]
    let config = read_config();

    let base = config.base();
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
