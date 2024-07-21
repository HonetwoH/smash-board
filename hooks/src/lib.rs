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