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

type Blob = String;

const DB_PATH: &str = "/tmp/smash.db";
const LIMIT: usize = 8;

impl Db {
    fn new() -> Result<Self> {
        let path = Path::new(DB_PATH);
        let conn = if !path.exists() {
            let conn = Connection::open(path)?;
            conn.execute(
                "CREATE TABLE pastes (
                    id    INTEGER PRIMARY KEY,
                    paste BLOB
                )",
                (), // empty list of parameters.
            )?;
            conn
        } else {
            let conn = Connection::open(path)?;
            conn
        };
        // let top_id = conn.query("SELECT id FROM pastes ORDER BY id DESC LIMIT 1", [])?;
        Ok(Self { conn, top_id: 0 })
    }

    // compute index by respecting the constraints imposed
    // i.e is octal or hexadecimal
    // TODO: Make sure that there is the db does not change right underneath the function call
    fn compute_index(&self, idx: usize) -> usize {
        //TODO: This method should return error along with the top_id
        if idx <= LIMIT {
            self.top_id - idx
        } else {
            self.top_id
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
            // .map(|x| self.compute_index(x))
            .map(|x| {
                // TODO: make sure that error from this part dont get unnoticed
                query.query_row([x], |row| row.get(0))
            })
            .collect()
    }

    fn show(&self) {
        dbg!(self.conn.execute("SELECT id, paste FROM pastes;", []));
    }
}
fn main() {
    let db = Db::new().unwrap();
    let _ = dbg!(db.push(String::from("Hello World")));
    let _ = dbg!(db.push(String::from("Hello Mars")));
    let _ = dbg!(db.push(String::from("Hello Mars")));
    let _ = dbg!(db.push(String::from("Hello Venus")));
    let _ = dbg!(db.push(String::from("Hello Jupiter")));
    let _ = dbg!(db.push(String::from("Hello Neptune")));
    let _ = dbg!(db.push(String::from("Hello Mercury")));
    let _ = dbg!(db.push(String::from("Hello Uranas")));
    dbg!(db.fetch(vec![1, 2, 3]));
    // dbg!(db.conn.prepare());
    // .execute("SELECT id FROM pastes ORDER BY id DESC LIMIT 1", []));
}
