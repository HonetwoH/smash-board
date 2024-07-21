use config::Base;
use rusqlite::{params, Connection, Result};
use std::path::Path;

// The database will be supporting a stack where their is no notion of
// pop only PUSH can happen and fetching at any postsion is possible if that
// index exists in db and the index can only be calculated with respect to
// the TOP and in this context peek will be called FETCH

// But the indexing will be limited to 8 or 16 which will be represented as
// octal or hexademical number respectively

// All the sql related things will be confined in this module

#[derive(Debug)]
pub struct Db {
    conn: Connection,
    top_id: usize,
    base: u8,
}

enum IndexErrors {
    NotEnoughEntries,
    NotPopulatedYet,
    ExceededLimit,
}

type Blob = String;

//TODO: make this path ramdom
const DB_PATH: &str = "/tmp/smash.db";

impl Db {
    pub fn new_connection(base: Base) -> Result<Self> {
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
            let top_id: usize = conn
                .query_row("SELECT MAX(id) FROM pastes;", [], |row| row.get(0))
                .unwrap_or(0);
            (conn, top_id)
        };
        Ok(Self {
            conn,
            top_id,
            base: base as u8,
        })
    }

    // compute index by respecting the constraints imposed
    // i.e is octal or hexadecimal
    fn compute_index(&self, idx: usize) -> Result<usize, IndexErrors> {
        //TODO: This method should return error along with the top_id
        if idx < self.base.into() && self.top_id > idx {
            Ok(self.top_id - idx)
        } else if idx > self.base.into() {
            Err(IndexErrors::ExceededLimit)
        } else if idx > self.top_id {
            Err(IndexErrors::NotEnoughEntries)
        } else {
            // this also shows up incase of no DB present
            // unreachable!(
            //     "Found another edge case right here:\n Top: {},\t Requested: {}",
            //     self.top_id, idx
            // )
            Err(IndexErrors::NotPopulatedYet)
        }
    }

    // push is expected to work on single blob at a time hence the string directly
    pub fn push(&self, blob: Blob) -> Result<()> {
        self.conn
            .execute("INSERT INTO pastes (paste) VALUES (?1)", params![blob])?;
        Ok(())
    }

    // fetch is expected to work with batch of blob indices
    pub fn fetch(&self, blobs: Vec<u8>) -> Vec<Blob> {
        let mut query = self
            .conn
            .prepare("SELECT paste FROM pastes WHERE id = ?1")
            .unwrap();
        blobs
            .into_iter()
            .map(|x| self.compute_index(x as usize))
            .filter(|x| x.is_ok())
            .map(|x| {
                // TODO: make sure that error from this part dont get unnoticed
                match x {
                    // get(0) is fine because thier can be only one value associated with an index
                    Ok(y) => query.query_row([y], |row| row.get(0)).unwrap(), // this part generates errors
                    Err(_) => unreachable!(),
                }
            })
            .collect()
    }

    pub fn peek(&self) -> Option<Blob> {
        let fetched = self.fetch(Vec::from([0]));
        assert!(fetched.len() == 1);
        fetched.first().cloned()
    }

    pub fn show(&self) -> Vec<Blob> {
        self.fetch(Vec::from_iter(0..self.base))
    }
}

#[test]
fn db_connection() {
    let db = Db::new_connection(Base::HexaDecimal).unwrap();

    let _ = dbg!(db.push(Blob::from("Hello World")));
    let _ = dbg!(db.push(Blob::from("Hello Mars")));
    let _ = dbg!(db.push(Blob::from("Hello Mars")));
    let _ = dbg!(db.push(Blob::from("Hello Venus")));
    let _ = dbg!(db.push(Blob::from("Hello Jupiter")));
    let _ = dbg!(db.push(Blob::from("Hello Neptune")));
    let _ = dbg!(db.push(Blob::from("Hello Mercury")));
    let _ = dbg!(db.push(Blob::from("Hello Uranas")));

    dbg!(db.show());
}
