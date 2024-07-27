#[cfg(feature = "wayland")]
pub mod wayland;
#[cfg(feature = "x11")]
mod x11 {}

mod utils {
    pub(crate) fn are_equal(a: &[u8], b: &[u8]) -> bool {
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
}

use std::io::Result;
pub trait Clipboard {
    fn get_from_clipboard(polling_delay: u64, previous: &mut Vec<u8>) -> Result<Vec<u8>>;
}
