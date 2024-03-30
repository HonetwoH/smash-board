mod Cli {
    use clap::{arg, ArgMatches, Command};

    fn start() {
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
            .subcommand(Command::new("compose").short_flag('c').arg(
                arg!(<BUFFERS> ... "series of buffer").value_parser(clap::value_parser!(String)),
            ))
            .subcommand(Command::new("edit").short_flag('e').arg(
                arg!(<BUFFERS> ... "series of buffer").value_parser(clap::value_parser!(String)),
            ))
    }
}

mod Db {
    use rusqlite::{params, Connection, Result};
    use std::path::Path;

    /*
        The database will be supporting a stack where their is no notion of
        pop only PUSH can happen and peeking at any value is possible if that
        index exists in db and the index can only be calculated with respect to
        the TOP and in this context peek will be called FETCH

        The indexing will be limited to 8 or 16 which will be represented as
        octal or hexademical number respectively

    */

    #[derive(Debug)]
    struct Db {
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
        fn new() -> Result<Self> {
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
        fn push(&self, blob: Blob) -> Result<()> {
            self.conn
                .execute("INSERT INTO pastes (paste) VALUES (?1)", params![blob])?;
            Ok(())
        }

        // fetch is expected to work with batch of blob indices
        fn fetch(&self, blob_idxs: Vec<usize>) -> Vec<Result<Blob>> {
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

        fn show(&self, range: usize) -> Result<()> {
            let mut stmt = self
                .conn
                .prepare("SELECT id, paste FROM pastes ORDER BY id DESC;")?;
            let que = stmt.query([])?;
            que.mapped(|r| {
                Ok(Output {
                    id: r.get(0)?,
                    paste: r.get(1)?,
                })
            })
            .take(range)
            .for_each(|x| {
                let output = x.unwrap();
                println!("{}\t{}", output.id, output.paste);
            });
            Ok(())
        }
    }
    #[test]
    fn test() {
        let db = Db::new().unwrap();
        let _ = dbg!(db.push(String::from("Hello World")));
        let _ = dbg!(db.push(String::from("Hello Mars")));
        let _ = dbg!(db.push(String::from("Hello Mars")));
        let _ = dbg!(db.push(String::from("Hello Venus")));
        let _ = dbg!(db.push(String::from("Hello Jupiter")));
        let _ = dbg!(db.push(String::from("Hello Neptune")));
        let _ = dbg!(db.push(String::from("Hello Mercury")));
        let _ = dbg!(db.push(String::from("Hello Uranas")));
        db.show(8);
        dbg!(db.fetch(vec![0, 1, 2, 3]));
    }
}
