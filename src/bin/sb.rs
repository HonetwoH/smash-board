use smash_board::cli::{args, Action};
use smash_board::config::Config;
use smash_board::db::Db;
use smash_board::grammar::check;

use tui::inline::show_preview;

#[cfg(feature = "tui")]
use tui::interactive::compose_ui;

#[cfg(feature = "read-config")]
use smash_board::config::read_config;

fn main() {
    let config = Config::default();

    #[cfg(feature = "read-config")]
    let config = read_config();

    let base = config.base();
    let parser = check(base);
    let pastes_db: Db = Db::new_connection(base).unwrap();
    match args() {
        Action::Show => {
            let blobs = pastes_db.show();
            if blobs.is_empty() {
                println!("There is nothing here")
            } else {
                show_preview(blobs.into_iter().enumerate().collect());
            }
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
