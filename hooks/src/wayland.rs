use crate::utils::are_equal;
use std::{
    io::{Read, Result},
    thread,
    time::Duration,
};

use wl_clipboard_rs::paste::{get_contents, ClipboardType, Error, MimeType, Seat};

pub fn get_from_clipboard(previous: &mut Vec<u8>) -> Result<Vec<u8>> {
    let mut contents = vec![];
    loop {
        let result = get_contents(ClipboardType::Regular, Seat::Unspecified, MimeType::Text);
        match result {
            Ok((mut pipe, _)) => {
                contents.clear();
                pipe.read_to_end(&mut contents)?;
                if !are_equal(&contents, previous) {
                    previous.clone_from(&contents);
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
