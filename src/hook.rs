use crate::config::Base;
use crate::db::Db;
use std::{
    io::{self, Read},
    thread,
    time::Duration,
};
use wl_clipboard_rs::paste::{get_contents, ClipboardType, Error, MimeType, Seat};

// Use logging and respawn the process and if it failed too much then notify user
fn poll_clipboard() -> io::Result<()> {
    fn get_from_clipboard(previous: &mut Vec<u8>) -> io::Result<Vec<u8>> {
        let mut contents = vec![];
        loop {
            let result = get_contents(ClipboardType::Regular, Seat::Unspecified, MimeType::Text);
            match result {
                Ok((mut pipe, _)) => {
                    contents.clear();
                    pipe.read_to_end(&mut contents)?;
                    if !are_equal(&contents, &previous) {
                        *previous = contents.clone();
                        return Ok(contents);
                    }
                }
                Err(Error::NoSeats) | Err(Error::ClipboardEmpty) | Err(Error::NoMimeType) => {
                    // The clipboard is empty or doesn't contain text, nothing to worry about.
                    todo!("This needs some working")
                }
                Err(_) => panic!(),
            };
            thread::sleep(Duration::from_secs(1));
        }
    }

    let mut previous = vec![];
    let db = Db::new_connection(Base::HexaDecimal).expect("Failed the create connection with db");
    loop {
        let content = get_from_clipboard(&mut previous)?;
        db.push(String::from_utf8(content).expect("Failed to convert to string"))
            .expect("Failed to push the paste to the db");
    }
}

fn are_equal(a: &Vec<u8>, b: &Vec<u8>) -> bool {
    if a.len() != b.len() {
        false
    } else {
        let l = a.len();
        for i in 0..l {
            if a[i] != b[i] {
                return false;
            }
        }
        true
    }
}
