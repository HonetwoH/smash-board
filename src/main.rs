mod cli {
    use clap::{arg, ArgMatches, Command};
    use core::panic;

    pub fn start() {
        use crate::db::Db;
        let pastesbin = Db::new().unwrap();
        match argument() {
            ActionForCore::Show => pastesbin.show(8),
            ActionForCore::Composed(order_of_buffers) => {
                dbg!(order_of_buffers);
            }
            ActionForCore::Push(new_paste) => {
                dbg!(&new_paste);
                pastesbin.push(new_paste).unwrap();
            }
            _ => panic!(),
        }
    }

    #[derive(Debug)]
    pub enum ActionForCore {
        Show,
        Paste,
        Push(String),
        StartDaemon,
        Edit,
        // compose action will only handle the compostion and
        // it cannot operate on any thing on itself
        Composed(String),
    }

    fn argument() -> ActionForCore {
        let collect_argument = |buffers: &ArgMatches, with_spaces: bool| {
            let buf = buffers.get_raw("BUFFERS").unwrap();
            if with_spaces {
                buf.fold(String::new(), |mut arg, x| {
                    arg.push_str(x.to_str().expect("Convertion failed"));
                    arg.push(' ');
                    arg
                })
            } else {
                buf.fold(String::new(), |mut arg, x| {
                    arg.push_str(x.to_str().expect("Convertion failed"));
                    arg
                })
            }
        };
        let matches = cli().get_matches();
        match matches.subcommand() {
            Some(("show", _)) => ActionForCore::Show,
            Some(("edit", _)) => ActionForCore::Edit,
            Some(("push", buffers)) => {
                let parsed_arg = collect_argument(buffers, true);
                ActionForCore::Push(parsed_arg)
            }
            Some(("compose", buffers)) => {
                let parsed_arg = collect_argument(buffers, false);
                ActionForCore::Composed(parsed_arg)
            }
            // daemon is called without arguments
            // Some(("daemon", _)) => ActionForCore::StartDaemon,
            _ => unreachable!(),
        }
    }

    fn cli() -> Command {
        Command::new("sb")
            .about("New kind of clipboard")
            .arg_required_else_help(true)
            .subcommand(
                Command::new("show")
                    .short_flag('s')
                    .about("Shows the recent buffers"),
            )
            .subcommand(
                Command::new("push")
                    .short_flag('p')
                    .about("Add the following string to board")
                    .arg(
                        arg!(<BUFFERS> ... "series of buffer")
                            .value_parser(clap::value_parser!(String)),
                    ),
            )
            .subcommand(
                Command::new("compose")
                    .short_flag('c')
                    .about("Reads sequence of buffer for edit command")
                    .arg(
                        arg!(<BUFFERS> ... "series of buffer")
                            .value_parser(clap::value_parser!(String)),
                    ),
            )
            .subcommand(Command::new("edit").short_flag('e').arg(
                arg!(<BUFFERS> ... "series of buffer").value_parser(clap::value_parser!(String)),
            ))
            .subcommand(Command::new("daemon").short_flag('d'))
    }
}

// mod Daemon {
//     // Will take care of interactions between the client and database and handle
//     // There should be only one daemon obviously !!
//     //
//     // setup daemon and wait for wl-paste to spew out stuff
//     use ctrlc;
//     use std::fs::File;
//     use std::path::Path;
//     const LOCKFILE: &str = "/tmp/smashed";

//     enum Msg {
//         Created,
//         AlreadyPresent,
//     }

//     fn lockfile() -> Msg {
//         match File::create(LOCKFILE) {
//             Ok(_) => Msg::Created,
//             Err(_) => Msg::AlreadyPresent,
//         }
//     }
// }

mod db {
    use rusqlite::{params, Connection, Result};
    use std::path::Path;

    /*
        The database will be supporting a stack where their is no notion of
        pop only PUSH can happen and peeking at any value is possible if that
        index exists in db and the index can only be calculated with respect to
        the TOP and in this context peek will be called FETCH

        The indexing will be limited to 8 or 16 which will be represented as
        octal or hexademical number respectively

        All the sql related things will be confined in this module

    */

    #[derive(Debug)]
    pub struct Db {
        conn: Connection,
        top_id: usize,
    }

    enum IndexErrors {
        NotEnoughEntries,
        ExceededLimit,
    }

    type Blob = String;

    const DB_PATH: &str = "/tmp/smash.db";
    const LIMIT: usize = 8;

    #[derive(Debug)]
    struct Output {
        id: usize,
        paste: Blob,
    }

    impl Db {
        pub fn new() -> Result<Self> {
            let path = Path::new(DB_PATH);
            let (conn, top_id) = if !path.exists() {
                let conn = Connection::open(path)?;
                conn.execute(
                    "CREATE TABLE pastes (
                    id    INTEGER PRIMARY KEY,
                    paste BLOB
                )",
                    [],
                )?;
                (conn, 0)
            } else {
                let conn = Connection::open(path)?;
                let top_id: usize =
                    match conn.query_row("SELECT MAX(id) FROM pastes;", [], |row| row.get(0)) {
                        Ok(x) => x,
                        Err(_) => 0,
                    };
                (conn, top_id)
            };
            Ok(Self { conn, top_id })
        }

        // compute index by respecting the constraints imposed
        // i.e is octal or hexadecimal
        // TODO: Make sure that there is the db does not change right underneath the function call
        fn compute_index(&self, idx: usize) -> Result<usize, IndexErrors> {
            //TODO: This method should return error along with the top_id
            if idx <= LIMIT && self.top_id > idx {
                Ok(self.top_id - idx)
            } else if idx > LIMIT {
                Err(IndexErrors::ExceededLimit)
            } else if idx > self.top_id {
                Err(IndexErrors::NotEnoughEntries)
            } else {
                unreachable!(
                    "Found another edge case right here:\n Top: {},\t Requested: {}",
                    self.top_id, idx
                )
            }
        }

        // push is expected to work on single blob at a time hence the string directly
        pub fn push(&self, blob: Blob) -> Result<()> {
            self.conn
                .execute("INSERT INTO pastes (paste) VALUES (?1)", params![blob])?;
            Ok(())
        }

        // fetch is expected to work with batch of blob indices
        pub fn fetch(&self, blob_idxs: Vec<usize>) -> Vec<Result<Blob>> {
            let mut query = self
                .conn
                .prepare("SELECT paste FROM pastes WHERE id = ?1")
                .unwrap();
            blob_idxs
                .into_iter()
                .map(|x| self.compute_index(x))
                .map(|x| {
                    // TODO: make sure that error from this part dont get unnoticed
                    match x {
                        Ok(y) => query.query_row([y], |row| row.get(0)), // this part generates errors
                        Err(_) => Ok(String::new()),
                    }
                })
                .collect()
        }

        pub fn show(&self, range: usize) {
            let mut recent = self.fetch(Vec::from_iter(0..range)).into_iter();
            let mut index = 1;
            while let Some(Ok(buf)) = recent.next() {
                println!("\t{}\t{}", index, buf);
                index += 1;
            }
        }
    }
    // #[test]
    // fn test() {
    //     let db = Db::new().unwrap();
    //     let _ = dbg!(db.push(String::from("Hello World")));
    //     let _ = dbg!(db.push(String::from("Hello Mars")));
    //     let _ = dbg!(db.push(String::from("Hello Mars")));
    //     let _ = dbg!(db.push(String::from("Hello Venus")));
    //     let _ = dbg!(db.push(String::from("Hello Jupiter")));
    //     let _ = dbg!(db.push(String::from("Hello Neptune")));
    //     let _ = dbg!(db.push(String::from("Hello Mercury")));
    //     let _ = dbg!(db.push(String::from("Hello Uranas")));
    //     db.show(8);
    //     dbg!(db.fetch(vec![0, 1, 2, 3]));
    // }
}

fn main() {
    dbg!(cli::start());
}
