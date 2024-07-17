use core::hook::poll_clipboard;

//TODO: make a proper daemon with good UX
fn main() {
    loop {
        _ = poll_clipboard();
    }
}
