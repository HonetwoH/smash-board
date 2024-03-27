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

impl Db {
    fn create_db() -> Result<()> {
        let path = Path::new(DB_PATH);
        if !path.exists() {
            let conn = Connection::open(path)?;
            conn.execute(
                "CREATE TABLE pastes (
            id   INTEGER PRIMARY KEY,
            data BLOB
        )",
                (), // empty list of parameters.
            )?;
        }
        Ok(())
    }

    // compute index by respecting the constraints imposed
    // i.e is octal or hexadecimal
    // TODO: Make sure that there is the db does not change right underneath the function call
    fn compute_index(&self, idx: usize) -> usize {
        self.top_id
    }

    // push is expected to work on single blob at a time hence the string directly
    fn push(&self, blob: Blob) -> Result<()> {
        self.conn
            .execute("INSERT INTO pastes (data) VALUES (?1)", params![blob])?;
        Ok(())
    }

    // fetch is expected to work with batch of blob indices
    fn fetch(&self, blob_idxs: Vec<usize>) {
        self.conn.execute_batch()
    }
}
fn main() -> Result<()> {
    let path = Path::new(DB_PATH);
    let conn = Connection::open(path)?;
    for i in 0..11 {
        let me = Paste {
            data: Some("This as message".to_string()),
        };
        conn.execute("INSERT INTO person (data) VALUES (?1)", params![&me.data])?;
    }
    let mut stmt = conn.prepare("SELECT id, data FROM person")?;
    let out_iter = stmt.query_map([], |row| {
        Ok(Out {
            id: row.get(0)?,
            data: row.get(1)?,
        })
    })?;

    for person in out_iter {
        println!("Found person {:?}", person.unwrap());
    }
    Ok(())
}
